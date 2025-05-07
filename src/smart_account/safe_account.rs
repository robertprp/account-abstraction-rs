use alloy::{
    network::Ethereum,
    primitives::{Address, Bytes, ChainId, U256},
    providers::Provider,
    sol,
    sol_types::SolInterface,
};
use async_trait::async_trait;
use SafeModuleSetupContract::{enableModulesCall, SafeModuleSetupContractCalls};

use Safe4337ModuleContract::{executeUserOpCall, Safe4337ModuleContractCalls};
use SafeL2Contract::{setupCall, SafeL2ContractCalls};
use SafeProxyFactoryContract::{createProxyWithNonceCall, SafeProxyFactoryContractCalls};

use crate::{
    entry_point::EntryPointContractWrapper,
    signer::SmartAccountSigner,
    types::{ExecuteCall, UserOperation},
    utils,
};

use super::{AccountError, SmartAccount};

//
// Generate Alloy interfaces for the Safe contracts
//
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    SafeL2Contract,
    "src/abi/safe/SafeL2.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    SafeProxyFactoryContract,
    "src/abi/safe/SafeProxyFactory.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Safe4337ModuleContract,
    "src/abi/safe/Safe4337Module.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    SafeModuleSetupContract,
    "src/abi/safe/SafeModuleSetup.json"
);

// Constants
const SAFE_4337_MODULE_ADDRESS: &str = "0x75cf11467937ce3F2f357CE24ffc3DBF8fD5c226";
const SAFE_MODULE_SETUP_ADDRESS: &str = "0x2dd68b007B46fBe91B9A7c3EDa5A7a1063cB5b47";
const SAFE_SINGLETON_ADDRESS: &str = "0x29fcB43b46531BcA003ddC8FCB67FFE91900C762";
const SAFE_PROXY_FACTORY_ADDRESS: &str = "0x4e1DCf7AD4e460CfD30791CCC4F9c8a4f820ec67";
const ENTRYPOINT_ADDRESS: &str = "0x0000000071727De22E5E9d8BAf0edAc6f37da032";

/// Safe smart account.
#[derive(Clone, Debug)]
pub struct SafeAccount<P: Provider<Ethereum>> {
    provider: P,
    owners: Vec<Address>,
    threshold: U256,
    account_address: Option<Address>,
    entry_point: EntryPointContractWrapper<P, Ethereum>,
    chain_id: ChainId,
}

impl<P> SafeAccount<P>
where
    P: Provider<Ethereum> + Clone + std::fmt::Debug,
{
    pub fn new(
        provider: P,
        owners: Vec<Address>,
        threshold: U256,
        account_address: Option<Address>,
        chain_id: ChainId,
    ) -> Self {
        let entry_point = EntryPointContractWrapper::new(
            ENTRYPOINT_ADDRESS.parse().unwrap(),
            provider.clone(),
        );

        Self {
            provider,
            owners,
            threshold,
            account_address,
            entry_point,
            chain_id,
        }
    }

    fn encode_signatures(&self, valid_until: u64, valid_after: u64, signatures: &[u8]) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&valid_until.to_le_bytes()[..6]);
        buffer.extend_from_slice(&valid_after.to_le_bytes()[..6]);
        buffer.extend_from_slice(signatures);
        buffer
    }
}

impl<P> SafeAccount<P>
where
    P: Provider<Ethereum> + Clone + std::fmt::Debug + Send + Sync,
{
    /// Gets the Safe-specific user operation hash based on a 4337 user operation.
    async fn get_safe_user_op_hash<U>(&self, user_op: U) -> Result<[u8; 32], AccountError>
    where
        U: Into<UserOperation> + Send + Sync,
    {
        let user_op: UserOperation = user_op.into();
        // Empty signature since the contract doesn't use it.
        let packed_signature = self.encode_signatures(0, 0, &Bytes::new().to_vec());

        let packed_user_op = utils::pack_user_op(&user_op);

        let module_user_op = Safe4337ModuleContract::PackedUserOperation {
            sender: packed_user_op.sender,
            nonce: packed_user_op.nonce,
            initCode: packed_user_op.initCode,
            callData: packed_user_op.callData,
            accountGasLimits: packed_user_op.accountGasLimits,
            preVerificationGas: packed_user_op.preVerificationGas,
            gasFees: packed_user_op.gasFees,
            paymasterAndData: packed_user_op.paymasterAndData,
            signature: packed_signature.into(),
        };

        let contract = Safe4337ModuleContract::new(
            SAFE_4337_MODULE_ADDRESS.parse().unwrap(),
            self.provider.clone(),
        );

        let hash: [u8; 32] = contract
            .getOperationHash(module_user_op)
            .call()
            .await
            .map_err(|e| AccountError::SignerError(format!("Failed to get user op hash: {}", e)))
            .unwrap()
            .operationHash
            .into();

        Ok(hash)
    }
}

#[async_trait]
impl<P> SmartAccount<P, Ethereum> for SafeAccount<P>
where
    P: Provider<Ethereum> + Clone + std::fmt::Debug + Send + Sync,
{
    type P = P;
    type EntryPoint = EntryPointContractWrapper<P, Ethereum>;

    fn provider(&self) -> &Self::P {
        &self.provider
    }

    fn entry_point(&self) -> &Self::EntryPoint {
        &self.entry_point
    }

    fn chain_id(&self) -> ChainId {
        self.chain_id
    }

    fn get_factory_address(&self) -> Address {
        SAFE_PROXY_FACTORY_ADDRESS.parse().unwrap()
    }

    async fn get_account_address(&self) -> Result<Address, AccountError> {
        if let Some(addr) = self.account_address {
            return Ok(addr);
        }

        let addr = self.get_counterfactual_address().await?;
        
        Ok(addr)
    }

    async fn get_init_code(&self) -> Result<Bytes, AccountError> {
        let index = U256::ZERO;

        let enable_modules_call = SafeModuleSetupContractCalls::enableModules(enableModulesCall {
            modules: vec![SAFE_4337_MODULE_ADDRESS.parse().unwrap()],
        });

        let setup_call = SafeL2ContractCalls::setup(setupCall {
            _owners: self.owners.clone(),
            _threshold: self.threshold,
            to: SAFE_MODULE_SETUP_ADDRESS.parse().unwrap(),
            data: enable_modules_call.abi_encode().into(),
            fallbackHandler: SAFE_4337_MODULE_ADDRESS.parse().unwrap(),
            paymentToken: Address::ZERO,
            payment: U256::ZERO,
            paymentReceiver: Address::ZERO,
        });

        let create_proxy_call =
            SafeProxyFactoryContractCalls::createProxyWithNonce(createProxyWithNonceCall {
                _singleton: SAFE_SINGLETON_ADDRESS.parse().unwrap(),
                initializer: setup_call.abi_encode().into(),
                saltNonce: index,
            });

        let mut init_code = Vec::new();
        let factory_address: Address = SAFE_PROXY_FACTORY_ADDRESS.parse().unwrap();
        init_code.extend_from_slice(factory_address.as_slice());
        init_code.extend_from_slice(&create_proxy_call.abi_encode());

        Ok(Bytes::from(init_code))
    }

    async fn is_account_deployed(&self) -> Result<bool, AccountError> {
        let addr = self.get_account_address().await?;
        let code = self
            .provider
            .get_code_at(addr)
            .await
            .map_err(|e| AccountError::RpcError(e.to_string()))?;
        
        Ok(!code.is_empty())
    }

    async fn encode_execute(&self, call: ExecuteCall) -> Result<Vec<u8>, AccountError> {
        let call = Safe4337ModuleContractCalls::executeUserOp(executeUserOpCall {
            to: call.target,
            value: call.value,
            data: call.data,
            operation: 0.into(), // Call operation
        });

        Ok(call.abi_encode().into())
    }

    async fn encode_execute_batch(
        &self,
        _calls: Vec<ExecuteCall>,
    ) -> Result<Vec<u8>, AccountError> {
        // TODO: Implement batch execution
        unimplemented!()
    }

    async fn get_user_op_hash<U: Into<UserOperation> + Send + Sync>(
        &self,
        user_op: U,
    ) -> Result<[u8; 32], AccountError> {
        let user_op_hash = self.get_safe_user_op_hash(user_op).await?;

        Ok(user_op_hash)
    }

    async fn sign_user_op_hash<S: SmartAccountSigner>(
        &self,
        user_op_hash: &[u8; 32],
        signer: &S,
    ) -> Result<Bytes, AccountError> {
        let signature = signer
            .sign_hash_data(user_op_hash.into())
            .await
            .map_err(|e| AccountError::SignerError(format!("Failed to sign user op hash: {}", e)))?;

        let packed_signature = self.encode_signatures(0, 0, &signature.to_vec());

        Ok(Bytes::from(packed_signature))
    }

    async fn sign_user_op<U: Into<UserOperation> + Send + Sync, S: SmartAccountSigner>(
        &self,
        user_op: U,
        signer: &S,
    ) -> Result<Bytes, AccountError> {
        let safe_user_op_hash = self.get_user_op_hash(user_op).await?;

        self.sign_user_op_hash(&safe_user_op_hash, signer).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        provider::{SmartAccountProvider, SmartAccountProviderTrait},
        types::{AccountCall, UserOperationRequest},
    };
    use alloy::{
        network::EthereumWallet, providers::ProviderBuilder, signers::local::PrivateKeySigner,
    };
    use url::Url;

    const RPC_URL: &str = "https://base-sepolia.g.alchemy.com/v2/IVqOyg3PqHzBQJMqa_yZAfyonF9ne2Gx";

    #[tokio::test]
    async fn test_get_safe_address() {
        let signer: PrivateKeySigner =
            "82aba1f2ce3d1a0f6eca0ade8877077b7fc6fd06fb0af48ab4a53650bde69979"
                .parse()
                .unwrap();
        let wallet = EthereumWallet::from(signer.clone());

        let rpc_url = Url::parse(RPC_URL).unwrap();
        let provider = ProviderBuilder::new().wallet(wallet).on_http(rpc_url);

        let account = SafeAccount::new(
            provider,
            vec![signer.address()],
            U256::from(1),
            Some(
                "0x001D57AdB1461d456541354BBcD515d433299113"
                    .parse()
                    .unwrap(),
            ),
            84532,
        );

        let safe_address = account.get_account_address().await.unwrap();

        println!("Signer address: {:?}", signer.address());
        println!("Safe address: {:?}", safe_address);
    }

    #[tokio::test]
    async fn test_send_transaction() {
        let signer: PrivateKeySigner =
            "82aba1f2ce3d1a0f6eca0ade8877077b7fc6fd06fb0af48ab4a53650bde69979"
                .parse()
                .unwrap();
        let wallet = EthereumWallet::from(signer.clone());

        let rpc_url = Url::parse(RPC_URL).unwrap();
        let provider = ProviderBuilder::new().wallet(wallet).on_http(rpc_url);

        let account = SafeAccount::new(
            provider.clone(),
            vec![signer.address()],
            U256::from(1),
            // None,
            Some(
                "0x001D57AdB1461d456541354BBcD515d433299113"
                    .parse()
                    .unwrap(),
            ),
            84532,
        );

        let to_address: Address = "0xde3e943a1c2211cfb087dc6654af2a9728b15536"
            .parse()
            .unwrap();

        let sender = account.get_account_address().await.unwrap();
        let req = UserOperationRequest::new(AccountCall::Execute(ExecuteCall::new(
            to_address,
            U256::from(100),
            Bytes::default(),
        )))
        .sender(sender);

        let smart_account_provider = SmartAccountProvider::new(provider, account);
        let result = smart_account_provider
            .send_user_operation(req, &signer)
            .await;

        let user_op_hash = result.expect("Failed to send user operation");
        println!("User operation hash: {:?}", user_op_hash);

        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
        let mut attempts = 0;
        let max_attempts = 20;

        loop {
            interval.tick().await;
            attempts += 1;

            match smart_account_provider
                .get_user_operation_receipt(user_op_hash)
                .await
            {
                Ok(Some(receipt)) => {
                    println!("Received receipt: {:?}", receipt);
                    break;
                }
                Ok(None) => {
                    println!("Receipt not available yet, retrying...");
                }
                Err(e) => {
                    println!("Failed to get user operation receipt: {:?}", e);
                    if attempts >= max_attempts {
                        println!("Exceeded max attempts ({max_attempts}), stopping retries");
                        break;
                    }
                }
            }

            if attempts >= max_attempts {
                panic!("Failed to get receipt after {max_attempts} attempts");
            }
        }
    }
}
