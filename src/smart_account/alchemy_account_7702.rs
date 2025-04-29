use alloy::{
    network::Ethereum,
    primitives::{aliases::U192, Address, Bytes, ChainId, U256},
    providers::Provider,
    sol,
    sol_types::SolInterface,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use AlchemyModularAccountContract::{
    executeBatchCall, executeCall, AlchemyModularAccountContractCalls,
};

use crate::{
    entry_point::EntryPointContractWrapper,
    signer::SmartAccountSigner,
    types::{ExecuteCall, UserOperationRequest},
};

use super::{AccountError, SmartAccount};

/// Constants for Alchemy
const ENTRY_POINT_ADDRESS: &str = "0x0000000071727De22E5E9d8BAf0edAc6f37da032";
/// 65-byte filler signature (r‖s‖v) supplied by the user
const DUMMY_SIGNATURE: &str =
        "0xfffffffffffffffffffffffffffffff0000000000000000000000000000000007aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1c";

//
// Generate Alloy interfaces for the Alchemy contracts
//
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    AlchemyModularAccountContract,
    "src/abi/alchemy/SemiModularAccount.json"
);

/// An Alloy implementation of a ZeroDev kernel account.
#[derive(Debug)]
pub struct AlchemyModularAccount7702<P: Provider<Ethereum>> {
    provider: Arc<P>,
    account_address: Address,
    entry_point: Arc<EntryPointContractWrapper<P, Ethereum>>,
    chain_id: ChainId,
}

impl<P> AlchemyModularAccount7702<P>
where
    P: Provider<Ethereum> + Clone + std::fmt::Debug,
{
    pub fn new(provider: Arc<P>, account_address: Address, chain_id: ChainId) -> Self {
        let entry_point = Arc::new(EntryPointContractWrapper::new(
            ENTRY_POINT_ADDRESS.parse().unwrap(),
            (*provider).clone(),
        ));

        Self {
            provider,
            account_address,
            entry_point,
            chain_id,
        }
    }
}

#[async_trait]
impl<P> SmartAccount<P, Ethereum> for AlchemyModularAccount7702<P>
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

    /// Returns the factory address (from which init code will be derived).
    fn get_factory_address(&self) -> Address {
        unreachable!("7702 Alchemy modular account does not have a factory address")
    }

    fn get_dummy_signature(&self) -> Bytes {
        let signature: Bytes = DUMMY_SIGNATURE.parse().unwrap();

        let mut out = Vec::with_capacity(1 + 1 + 65);
        out.push(0xFF);
        out.push(0x00);
        out.extend_from_slice(&signature);

        out.into()
    }

    /// Returns the account address; if not known, computes it via counterfactual methods.
    async fn get_account_address(&self) -> Result<Address, AccountError> {
        Ok(self.account_address)
    }

    /// Constructs the initialization code needed to deploy the account.
    async fn get_init_code(&self) -> Result<Bytes, AccountError> {
        Ok(Bytes::default())
    }

    /// Checks if the account is deployed by using the stored flag.
    async fn is_account_deployed(&self) -> Result<bool, AccountError> {
        Ok(true)
    }

    async fn get_nonce(&self) -> Result<U256, AccountError> {
        let nonce_key = U256::from(1u8);

        let counter: U256 = self
            .get_nonce_with_key(U192::from(nonce_key))
            .await
            .map_err(|e| AccountError::RpcError(format!("Failed to get nonce: {}", e)))?;

        Ok((nonce_key << 64) | counter)
    }

    /// Encode a execute call.
    async fn encode_execute(&self, call: ExecuteCall) -> Result<Vec<u8>, AccountError> {
        let encoded = AlchemyModularAccountContractCalls::execute(executeCall {
            target: call.target,
            value: call.value,
            data: call.data,
        })
        .abi_encode();

        Ok(encoded)
    }

    /// Encode a batch-execute call.
    async fn encode_execute_batch(&self, calls: Vec<ExecuteCall>) -> Result<Vec<u8>, AccountError> {
        let encoded = AlchemyModularAccountContractCalls::executeBatch(executeBatchCall {
            calls: calls
                .into_iter()
                .map(|c| AlchemyModularAccountContract::Call::from((c.target, c.value, c.data)))
                .collect(),
        })
        .abi_encode();

        Ok(encoded)
    }

    async fn sign_user_op_hash<S: SmartAccountSigner>(
        &self,
        user_op_hash: &[u8; 32],
        signer: &S,
    ) -> Result<Bytes, AccountError> {
        let signature: Bytes = signer
            .sign_message(user_op_hash)
            .await
            .map_err(|e| AccountError::SignerError(format!("Failed to sign user op hash: {e}")))?;

        // 0xff routing  | 0x00 SignatureType::EOA | signature
        let mut out = Vec::with_capacity(1 + 1 + signature.len());
        out.push(0xFF);
        out.push(0x00);
        out.extend_from_slice(&signature.to_vec());
        Ok(out.into())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::{
        provider::{SmartAccountProvider, SmartAccountProviderTrait},
        types::{AccountCall, UserOperationRequest},
    };
    use alloy::{
        network::EthereumWallet,
        primitives::U256,
        providers::ProviderBuilder,
        rpc::types::Authorization,
        signers::local::PrivateKeySigner,
    };
    use url::Url;

    const RPC_URL: &str = "https://eth-sepolia.g.alchemy.com/v2/HoWbfthBOcacceoQbcrf66uJfh0Y9aoW";

    #[tokio::test]
    async fn test_send_transaction() {
        let signer: PrivateKeySigner =
            "5dde9fdb15a6794db681fd1c2a9b97dd43c0500057c9446fdf1668a604c25164"
                .parse()
                .unwrap();
        let wallet = EthereumWallet::from(signer.clone());
        println!("Signer address: {:?}", signer.address());

        let rpc_url = Url::parse(RPC_URL).unwrap();
        let provider = ProviderBuilder::new().wallet(wallet).on_http(rpc_url);

        let account =
            AlchemyModularAccount7702::new(Arc::new(provider.clone()), signer.address(), 11155111);

        println!(
            "Code: {:?}",
            account
                .provider
                .get_code_at(account.account_address)
                .await
                .unwrap()
        );

        let to_address: Address = "0xde3e943a1c2211cfb087dc6654af2a9728b15536"
            .parse()
            .unwrap();

        let auth = Authorization {
            chain_id: U256::from(0),
            address: Address::from_str("0x69007702764179f14f51cdce752f4f775d74e139").unwrap(),
            nonce: 3,
        };
        let signed_auth = signer.sign_hash_data(&auth.signature_hash()).await.unwrap();
        println!("Signed auth: {:?}", signed_auth);

        let req = UserOperationRequest::new(AccountCall::Execute(ExecuteCall::new(
            to_address,
            U256::from(100),
            Bytes::default(),
        )))
        // .eip7702_auth(Eip7702Auth {
        //     chain_id: U256::from(0),
        //     nonce: U256::from(3),
        //     address: Address::from_str("0x69007702764179f14f51cdce752f4f775d74e139").unwrap(),
        //     r: signature.r().into(),//B256::from_str("80272866576590756272238033559756661884484513719315537840184035893196945600562").unwrap(),
        //     s: signature.s().into(),//B256::from_str("27048359638535030316012546281053842977443639923339062128509724997388419800846").unwrap(),
        //     y_parity: U256::from(1),
        // })
        .max_priority_fee_per_gas(U256::from(318999872));

        let smart_account_provider = SmartAccountProvider::new(provider.clone(), account);

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
