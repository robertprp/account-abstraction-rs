use super::{AccountError, BaseAccount, SmartAccountSigner};

use crate::contracts::safe_proxy_factory::SafeProxyFactoryCalls;
use crate::contracts::EntryPoint as EthersEntryPoint;
use crate::contracts::{
    self, safe_proxy_factory, EnableModulesCall, ExecuteUserOpCall, Safe4337Module,
    Safe4337ModuleCalls, SafeL2Calls, UserOperation,
};
use crate::types::{ExecuteCall, UserOperationRequest};

use async_trait::async_trait;
use ethers::abi::AbiEncode;
use ethers::providers::{Http, Middleware};
use ethers::types::Chain;
use ethers::{
    providers::Provider,
    types::{Address, Bytes, U256},
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::RwLock;

// const ENTRY_POINT_ADDRESS: &str = "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789";
// const SIMPLE_ACCOUNT_FACTORY_ADDRESS: &str = "0x9406Cc6185a346906296840746125a0E44976454";

const SAFE_4337_MODULE_ADDRESS: &str = "0xa581c4A4DB7175302464fF3C06380BC3270b4037";
const ADD_MODULES_LIB_ADDRESS: &str = "0x8EcD4ec46D4D2a6B64fE960B3D64e8B94B2234eb";
const SAFE_PROXY_FACTORY_ADDRESS: &str = "0x4e1DCf7AD4e460CfD30791CCC4F9c8a4f820ec67";
const SAFE_SINGLETON_ADDRESS: &str = "0x29fcB43b46531BcA003ddC8FCB67FFE91900C762";
const ENTRYPOINT_ADDRESS: &str = "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789";
const FALLBACK_HANDLER_ADDRESS: &str = "0xfd0732Dc9E303f09fCEf3a7388Ad10A83459Ec99";

#[derive(Debug)]
pub struct SafeStandardAccount {
    inner: Arc<Provider<Http>>,
    owners: Vec<Address>,
    threshold: U256,
    // TODO: Remove RwLock
    account_address: RwLock<Option<Address>>,
    factory_address: Address,
    is_deployed: RwLock<bool>,
    entry_point: Arc<EthersEntryPoint<Provider<Http>>>,
    safe_4337_module: Arc<Safe4337Module<Provider<Http>>>,
    chain: Chain,
}

impl SafeStandardAccount {
    pub fn new(
        inner: Arc<Provider<Http>>,
        owners: Vec<Address>,
        threshold: U256,
        account_address: RwLock<Option<Address>>,
        factory_address: Address,
        entry_point_address: Address,
        safe_4337_module_address: Address,
        is_deployed: RwLock<bool>,
        chain: Chain,
    ) -> Self {
        let entry_point = Arc::new(EthersEntryPoint::new(entry_point_address, inner.clone()));
        let safe_4337_module =
            Arc::new(Safe4337Module::new(safe_4337_module_address, inner.clone()));

        Self {
            inner,
            owners,
            threshold,
            account_address,
            factory_address,
            is_deployed,
            entry_point,
            safe_4337_module,
            chain,
        }
    }
}

#[async_trait]
impl BaseAccount for SafeStandardAccount {
    type EntryPoint = EthersEntryPoint<Provider<Http>>;
    type Provider = Http;
    type Inner = Provider<Http>;

    fn inner(&self) -> &Self::Inner {
        &self.inner
    }

    fn entry_point(&self) -> &Self::EntryPoint {
        &self.entry_point
    }

    fn get_chain(&self) -> Chain {
        self.chain
    }

    async fn get_account_address(&self) -> Result<Address, AccountError> {
        let Some(account_address) = *self.account_address.read().await else {
            let address: Address = self.get_counterfactual_address().await?;
            println!("Counterfactual address: {:x}", address);
            *self.account_address.write().await = Some(address);
            return Ok(address);
        };

        Ok(account_address)
    }

    async fn get_account_init_code(&self) -> Result<Bytes, AccountError> {
        // TODO: Add optional index
        let index = U256::from(0);

        // TODO: getChainSpecificDefaultSaltNonce
        let safe_4337_module_address: Address = SAFE_4337_MODULE_ADDRESS.parse().unwrap();
        let add_modules_lib_address: Address = ADD_MODULES_LIB_ADDRESS.parse().unwrap();
        let enable_modules_call = EnableModulesCall {
            modules: vec![safe_4337_module_address],
        };
        let encoded_enable_modules_call: Bytes = enable_modules_call.encode().into();

        // Should be same as 4337 module address.
        let fallback_handler_address: Address = safe_4337_module_address;
        let setup_call_params = contracts::safe_l2::SetupCall {
            owners: self.owners.clone(),
            threshold: self.threshold,
            to: add_modules_lib_address,
            data: encoded_enable_modules_call,
            fallback_handler: fallback_handler_address,
            payment_token: Address::zero(),
            payment: U256::zero(),
            payment_receiver: Address::zero(),
        };

        let setup_call = SafeL2Calls::Setup(setup_call_params);

        let singleton_address: Address = SAFE_SINGLETON_ADDRESS.parse().unwrap();
        let create_proxy_with_nonce_call_params = safe_proxy_factory::CreateProxyWithNonceCall {
            singleton: singleton_address,
            initializer: setup_call.encode().into(),
            salt_nonce: index, //PREDETERMINED_SALT_NONCE,
        };
        let create_proxy_with_nonce_call =
            SafeProxyFactoryCalls::CreateProxyWithNonce(create_proxy_with_nonce_call_params);

        let safe_proxy_factory_address: Address = SAFE_PROXY_FACTORY_ADDRESS.parse().unwrap();

        let mut result: Vec<u8> = Vec::new();

        result.extend_from_slice(safe_proxy_factory_address.as_bytes());
        result.extend_from_slice(&create_proxy_with_nonce_call.encode());

        let result_bytes = Bytes::from(result);

        Ok(result_bytes)
    }

    async fn is_deployed(&self) -> bool {
        *self.is_deployed.read().await
    }

    async fn set_is_deployed(&self, is_deployed: bool) {
        *self.is_deployed.write().await = is_deployed;
    }

    async fn encode_execute(&self, call: ExecuteCall) -> Result<Vec<u8>, AccountError> {
        let call = Safe4337ModuleCalls::ExecuteUserOp(ExecuteUserOpCall {
            to: call.target,
            value: call.value,
            data: call.data,
            // Call: 0 DelegateCall: 1
            operation: 0,
        });

        Ok(call.encode())
    }

    async fn encode_execute_batch(&self, calls: Vec<ExecuteCall>) -> Result<Vec<u8>, AccountError> {
        // return INTERFACES.encodeFunctionData('multiSend', [
        //     encodeMultiSendData(
        //       transactions.map((tx) => ({ ...tx, operation: tx.operation ?? OperationType.Call }))
        //     )
        //   ])

        unimplemented!();
    }

    async fn get_user_op_hash<U: Into<UserOperation> + Send + Sync>(
        &self,
        user_op: U,
    ) -> Result<[u8; 32], AccountError> {
        unimplemented!()
    }

    async fn sign_user_op_hash<S: SmartAccountSigner>(
        &self,
        user_op_hash: [u8; 32],
        signer: &S,
    ) -> Result<Bytes, AccountError> {
        // unimplemented!()
        signer
            .sign_message(&user_op_hash)
            .await
            .map_err(|_| AccountError::SignerError)
    }

    async fn sign_user_op<U: Into<UserOperation> + Send + Sync, S: SmartAccountSigner>(
        &self,
        user_op: U,
        signer: &S,
    ) -> Result<Bytes, AccountError> {
        let safe_user_op_hash = self.get_safe_user_op_hash(user_op).await?;
        let signature = signer
            .sign_hash_data(safe_user_op_hash.into())
            .map_err(|_| AccountError::SignerError)?;

        let packed_signature: Bytes = self.encode_signatures(0, 0, &signature.to_vec()).into();

        Ok(packed_signature)
    }
}

impl SafeStandardAccount {
    async fn get_safe_user_op_hash<U: Into<UserOperation> + Send + Sync>(
        &self,
        user_op: U,
    ) -> Result<[u8; 32], AccountError> {
        let user_op: UserOperation = user_op.into();
        // Empty signature since get_operation_hash doesn't care about it.
        let packed_signature = self.encode_signatures(0, 0, &Bytes::new().to_vec());

        let safe_4337_module_user_op = contracts::safe_4337_module::UserOperation {
            sender: user_op.sender,
            nonce: user_op.nonce,
            init_code: user_op.init_code,
            call_data: user_op.call_data,
            call_gas_limit: user_op.call_gas_limit,
            verification_gas_limit: user_op.verification_gas_limit,
            pre_verification_gas: user_op.pre_verification_gas,
            max_fee_per_gas: user_op.max_fee_per_gas,
            max_priority_fee_per_gas: user_op.max_priority_fee_per_gas,
            paymaster_and_data: user_op.paymaster_and_data,
            signature: packed_signature.into(),
        };

        let user_op_hash = self
            .safe_4337_module
            .get_operation_hash(safe_4337_module_user_op)
            .call()
            .await
            .map_err(|_| AccountError::SignerError)?;

        Ok(user_op_hash)
    }

    fn encode_signatures(&self, valid_until: u64, valid_after: u64, signatures: &[u8]) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend_from_slice(&valid_until.to_le_bytes()[..6]);
        buffer.extend_from_slice(&valid_after.to_le_bytes()[..6]);

        buffer.extend_from_slice(signatures);

        buffer
    }

    pub async fn get_paymaster_and_data<U: Into<UserOperationRequest> + Send + Sync>(
        &self,
        user_op: U,
    ) -> Result<UserOperationRequest, AccountError> {
        let user_op_req: UserOperationRequest = user_op.into();
        let nonce: U256 = self.get_nonce().await.unwrap_or(U256::from(0));
        let init_code: Bytes = self.get_init_code().await.unwrap_or(Bytes::new());

        let policy_id = "831ad866-14bf-4f4e-96e7-a5f3d083ba0a";

        let updated_user_op_req = user_op_req
            .nonce(nonce)
            .init_code(init_code);

        let params = AlchemyPaymasterParams {
            policy_id: policy_id.to_string(),
            entry_point: ENTRYPOINT_ADDRESS.to_string(),
            dummy_signature: "0xe8fe34b166b64d118dccf44c7198648127bf8a76a48a042862321af6058026d276ca6abb4ed4b60ea265d1e57e33840d7466de75e13f072bbd3b7e64387eebfe1b".to_string(),
            user_operation: updated_user_op_req.clone(),
        };
        
        let result: Result<AlchemyPaymasterResponse, AccountError> = self.inner()
            .provider()
            .request("alchemy_requestGasAndPaymasterAndData", [params])
            .await
            .map_err(|e| AccountError::ProviderError(e.into()));

        match result {
            Ok(resp) => {
                let final_user_op_req: UserOperationRequest = updated_user_op_req
                    .paymaster_and_data(resp.paymaster_and_data)
                    .call_gas_limit(resp.call_gas_limit)
                    .verification_gas_limit(resp.verification_gas_limit)
                    .pre_verification_gas(resp.pre_verification_gas)
                    .max_fee_per_gas(resp.max_fee_per_gas)
                    .max_priority_fee_per_gas(resp.max_priority_fee_per_gas);

                Ok(final_user_op_req)
            },
            Err(err) => Err(err),
        }
    }
}

#[derive(Serialize, Debug)]
struct AlchemyPaymasterParams {
    #[serde(rename = "policyId", default)]
    policy_id: String,
    #[serde(rename = "entryPoint", default)]
    entry_point: String,
    #[serde(rename = "dummySignature", default)]
    dummy_signature: String,
    #[serde(rename = "userOperation", default)]
    user_operation: UserOperationRequest,
}

#[derive(Deserialize, Serialize, Debug)]
struct AlchemyPaymasterResponse {
    #[serde(rename = "paymasterAndData", default)]
    paymaster_and_data: Bytes,
    #[serde(rename = "callGasLimit", default)]
    pub call_gas_limit: U256,
    #[serde(rename = "verificationGasLimit", default)]
    pub verification_gas_limit: U256,
    #[serde(rename = "preVerificationGas", default)]
    pub pre_verification_gas: U256,
    #[serde(rename = "maxFeePerGas", default)]
    pub max_fee_per_gas: U256,
    #[serde(rename = "maxPriorityFeePerGas", default)]
    pub max_priority_fee_per_gas: U256,
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use ethers::signers::{LocalWallet, Signer};
    use tokio::time;
    use url::Url;

    use crate::{
        smart_account::{SmartAccountMiddleware, SmartAccountProvider},
        types::{UserOperationRequest},
    };

    use super::*;
    
    const RPC_URL: &str = "https://base-sepolia.g.alchemy.com/v2/IVqOyg3PqHzBQJMqa_yZAfyonF9ne2Gx"; //"https://eth-goerli.g.alchemy.com/v2/Lekp6yzHz5yAPLKPNvGpMKaqbGunnXHS"; //"https://eth-mainnet.g.alchemy.com/v2/lRcdJTfR_zjZSef3yutTGE6OIY9YFx1E";

    #[tokio::test]
    async fn test_get_safe_address() {
        let factory_address: Address = SAFE_PROXY_FACTORY_ADDRESS.parse().unwrap();
        let entry_point_address: Address = ENTRYPOINT_ADDRESS.parse().unwrap();
        let wallet: LocalWallet =
            "82aba1f2ce3d1a0f6eca0ade8877077b7fc6fd06fb0af48ab4a53650bde69979"
                .parse()
                .unwrap();
        let account_address: Address = "0xEb312892f9ACADe523C838f738ed1649398257E5"
            .parse()
            .unwrap();
        let provider = Provider::<Http>::try_from(RPC_URL).unwrap();

        let account = SafeStandardAccount::new(
            Arc::new(provider),
            vec![wallet.address()],
            U256::one(),
            RwLock::new(Some(account_address)),
            factory_address,
            entry_point_address,
            SAFE_4337_MODULE_ADDRESS.parse().unwrap(),
            RwLock::new(true),
            Chain::BaseSepolia,
        );

        let safe_address = account.get_account_address().await.unwrap();

        println!("Signer address: {:x}", wallet.address());
        println!("Safe address: {:x}", safe_address);
        // Add assertions for the expected safe address value
        // For example:
        // assert_eq!(safe_address, Address::from_str("0x1234567890abcdef").unwrap());
    }

    #[tokio::test]
    async fn test_send_transaction() {
        let factory_address: Address = SAFE_PROXY_FACTORY_ADDRESS.parse().unwrap();
        let entry_point_address: Address = ENTRYPOINT_ADDRESS.parse().unwrap();
        let wallet: LocalWallet =
            "82aba1f2ce3d1a0f6eca0ade8877077b7fc6fd06fb0af48ab4a53650bde69979"
                .parse()
                .unwrap();

        let account_address: Address = "0xEb312892f9ACADe523C838f738ed1649398257E5"
            .parse()
            .unwrap();
        let provider = Provider::<Http>::try_from(RPC_URL).unwrap();

        let account = SafeStandardAccount::new(
            Arc::new(provider),
            vec![wallet.address()],
            U256::one(),
            RwLock::new(Some(account_address)),
            factory_address,
            entry_point_address,
            SAFE_4337_MODULE_ADDRESS.parse().unwrap(),
            RwLock::new(true),
            Chain::BaseSepolia,
        );

        let to_address: Address = "0xde3e943a1c2211cfb087dc6654af2a9728b15536"
            .parse()
            .unwrap();

        let sender: Address = account.get_account_address().await.unwrap();

        let call = ExecuteCall::new(
            to_address,
            100,
            Bytes::new(),
        );

        let encoded_call = account.encode_execute(call).await.unwrap();

        let req = UserOperationRequest::new()
            .call_data(encoded_call)
            .sender(sender);

        let updated_user_op: UserOperationRequest = account.get_paymaster_and_data(req).await.unwrap();

        let provider = make_provider(account);

        println!("updated_user_op {:?}", updated_user_op);
        let result = provider.send_user_operation(updated_user_op, &wallet).await;

        let user_op_hash = result.unwrap();

        let mut interval = time::interval(Duration::from_secs(10));
        let mut attempts = 0;
        let max_attempts = 20;

        loop {
            interval.tick().await;
            attempts += 1;

            match provider
                .get_user_operation_receipt(user_op_hash.clone())
                .await
            {
                Ok(receipt) => {
                    if let Some(receipt) = receipt {
                        println!("Received receipt: {:?}", receipt);
                        break;
                    }
                }
                Err(e) => {
                    println!("Failed to get user operation receipt: {:?}", e);
                    if attempts >= max_attempts {
                        println!("Exceeded max attempts, stopping retries");
                        break;
                    }
                }
            }
        }
    }

    fn make_provider(
        account: SafeStandardAccount,
    ) -> SmartAccountProvider<Http, SafeStandardAccount> {
        let url: Url = RPC_URL.try_into().unwrap();
        let http_provider = Http::new(url);

        let account_provider = SmartAccountProvider::new(http_provider, account);

        account_provider
    }
}
