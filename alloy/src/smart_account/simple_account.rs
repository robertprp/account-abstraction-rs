use alloy::{
    network::Ethereum,
    primitives::{Address, Bytes, ChainId, U256},
    providers::Provider,
    sol,
    sol_types::SolInterface,
    transports::http::{Client, Http},
};
use async_trait::async_trait;
use std::sync::{Arc, RwLock};
use SimpleAccountContract::{executeBatchCall, executeCall, SimpleAccountContractCalls};
use SimpleAccountFactoryContract::{createAccountCall, SimpleAccountFactoryContractCalls};

use super::{
    AccountError, EntryPointContractWrapper, SmartAccount,
    SmartAccountSigner,
};
use crate::types::ExecuteCall;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    SimpleAccountContract,
    "src/abi/SimpleAccount.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    SimpleAccountFactoryContract,
    "src/abi/SimpleAccountFactory.json"
);

#[derive(Debug)]
pub struct SimpleAccount<P: Provider<Http<Client>, Ethereum>> {
    provider: Arc<P>,
    owner: Address,
    account_address: RwLock<Option<Address>>,
    factory_address: Address,
    entry_point: Arc<EntryPointContractWrapper<P, Http<Client>, Ethereum>>,
    chain_id: ChainId,
}

impl<P> SimpleAccount<P>
where
    P: Provider<Http<Client>, Ethereum> + Clone + std::fmt::Debug,
{
    pub fn new(
        provider: Arc<P>,
        owner: Address,
        factory_address: Address,
        entry_point_address: Address,
        chain_id: ChainId,
    ) -> Self {
        let entry_point = Arc::new(EntryPointContractWrapper::new(
            entry_point_address,
            (*provider).clone(),
        ));

        Self {
            provider,
            owner,
            account_address: RwLock::new(None),
            factory_address,
            entry_point,
            chain_id,
        }
    }
}

#[async_trait]
impl<P> SmartAccount<P, Http<Client>, Ethereum> for SimpleAccount<P>
where
    P: Provider<Http<Client>, Ethereum> + Clone + std::fmt::Debug + Send + Sync,
{
    type P = P;
    type EntryPoint = EntryPointContractWrapper<P, Http<Client>, Ethereum>;

    fn provider(&self) -> &Self::P {
        &self.provider
    }

    fn entry_point(&self) -> &Self::EntryPoint {
        &self.entry_point
    }

    fn chain_id(&self) -> ChainId {
        self.chain_id
    }

    async fn get_account_address(&self) -> Result<Address, AccountError> {
        // Check if we have a cached address
        if let Some(addr) = *self.account_address.read().unwrap() {
            return Ok(addr);
        }

        // If not deployed, get the counterfactual address
        let addr = self.get_counterfactual_address().await?;

        // Cache the address
        *self.account_address.write().unwrap() = Some(addr);

        Ok(addr)
    }

    async fn get_init_code(&self) -> Result<Bytes, AccountError> {
        // Create a vector to store the factory address and encoded call
        let mut init_code = Vec::new();

        // Add the factory address bytes
        init_code.extend_from_slice(self.factory_address.as_slice());

        let call = SimpleAccountFactoryContractCalls::createAccount(createAccountCall {
            owner: self.owner,
            salt: U256::ZERO,
        })
        .abi_encode();
        // Add the encoded call bytes
        init_code.extend_from_slice(&call);

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
        let call = SimpleAccountContractCalls::execute(executeCall {
            dest: call.target,
            value: call.value,
            func: call.data,
        })
        .abi_encode();

        Ok(call)
    }

    async fn encode_execute_batch(&self, calls: Vec<ExecuteCall>) -> Result<Vec<u8>, AccountError> {
        let targets: Vec<Address> = calls.iter().map(|call| call.target).collect();
        let data: Vec<Bytes> = calls.iter().map(|call| call.data.clone()).collect();
        let call = SimpleAccountContractCalls::executeBatch(executeBatchCall {
            dest: targets,
            func: data,
        })
        .abi_encode();

        Ok(call)
    }

    async fn sign_user_op_hash<S: SmartAccountSigner>(
        &self,
        user_op_hash: &[u8; 32],
        signer: &S,
    ) -> Result<Bytes, AccountError> {
        signer
            .sign_message(user_op_hash)
            .await
            .map_err(|_| AccountError::SignerError)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        network::EthereumWallet,
        primitives::{Address, Bytes},
        providers::ProviderBuilder,
        signers::local::PrivateKeySigner,
    };
    use std::str::FromStr;
    use url::Url;

    const ENTRY_POINT_ADDRESS: &str = "0x0000000071727De22E5E9d8BAf0edAc6f37da032";
    const SIMPLE_ACCOUNT_FACTORY_ADDRESS: &str = "0x9406Cc6185a346906296840746125a0E44976454";

    #[tokio::test]
    async fn test_account_init_code() {
        let signer: PrivateKeySigner = "4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318"
            .parse()
            .unwrap();

        let address: Address = signer.address();

        let wallet = EthereumWallet::from(signer);

        let rpc_url =
            Url::parse("https://base-sepolia.g.alchemy.com/v2/IVqOyg3PqHzBQJMqa_yZAfyonF9ne2Gx")
                .unwrap();
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url);

        let account = SimpleAccount::new(
            Arc::new(provider),
            address,
            Address::from_str(SIMPLE_ACCOUNT_FACTORY_ADDRESS).unwrap(),
            Address::from_str(ENTRY_POINT_ADDRESS).unwrap(),
            84532,
        );

        let result = account.get_init_code().await.unwrap();

        let expected_init_code = Bytes::from_str("0x9406cc6185a346906296840746125a0e449764545fbfb9cf0000000000000000000000002c7536e3605d9c16a7a3d7b1898e529396a65c230000000000000000000000000000000000000000000000000000000000000000").unwrap();

        assert_eq!(result, expected_init_code);
    }

    #[tokio::test]
    async fn test_encode_execute() {
        let signer: PrivateKeySigner = "4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318"
            .parse()
            .unwrap();

        let address: Address = signer.address();

        let wallet = EthereumWallet::from(signer);

        let rpc_url =
            Url::parse("https://base-sepolia.g.alchemy.com/v2/IVqOyg3PqHzBQJMqa_yZAfyonF9ne2Gx")
                .unwrap();
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url);

        let account = SimpleAccount::new(
            Arc::new(provider),
            address,
            Address::from_str(SIMPLE_ACCOUNT_FACTORY_ADDRESS).unwrap(),
            Address::from_str(ENTRY_POINT_ADDRESS).unwrap(),
            84532,
        );

        let target_address: Address = "0xA87395ef99Fc13Bb043245521C559030aA1827a7"
            .parse()
            .unwrap();

        let call_data: Bytes =
            "0xa71bbebe00000000000000000000000000000000000000000000000000000000000000010021fb3f"
                .parse()
                .unwrap();

        let result: Bytes = account
            .encode_execute(ExecuteCall::new(target_address, U256::from(100), call_data))
            .await
            .unwrap()
            .into();

        let expected_result: Bytes = "0xb61d27f6000000000000000000000000a87395ef99fc13bb043245521c559030aa1827a7000000000000000000000000000000000000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000028a71bbebe00000000000000000000000000000000000000000000000000000000000000010021fb3f000000000000000000000000000000000000000000000000".parse().unwrap();

        assert_eq!(result, expected_result)
    }
    
    #[tokio::test]
    async fn test_get_counterfactual_address() {
        let signer: PrivateKeySigner = "4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318"
            .parse()
            .unwrap();

        let address: Address = signer.address();

        let wallet = EthereumWallet::from(signer);

        let rpc_url =
            Url::parse("https://base-sepolia.g.alchemy.com/v2/IVqOyg3PqHzBQJMqa_yZAfyonF9ne2Gx")
                .unwrap();
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url);

        let account = SimpleAccount::new(
            Arc::new(provider),
            address,
            Address::from_str(SIMPLE_ACCOUNT_FACTORY_ADDRESS).unwrap(),
            Address::from_str(ENTRY_POINT_ADDRESS).unwrap(),
            84532,
        );

        let result = account.get_counterfactual_address().await.unwrap();

        assert_eq!(
            result,
            "0x982ffac966b962bddf89d3b26fee91da6f68df13"
                .parse::<Address>()
                .unwrap()
        );
    }
}
