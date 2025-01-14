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

use crate::entry_point::EntryPointContractWrapper;
use crate::signer::SmartAccountSigner;
use crate::types::ExecuteCall;

use super::{AccountError, SmartAccount};

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

    fn get_factory_address(&self) -> Address {
        self.factory_address
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
        unimplemented!()
        // let targets: Vec<Address> = calls.iter().map(|call| call.target).collect();
        // let data: Vec<Bytes> = calls.iter().map(|call| call.data.clone()).collect();
        // let values: Vec<U256> = calls.iter().map(|call| call.value).collect();
        // let call = SimpleAccountContractCalls::executeBatch(executeBatchCall {
        //     dest: targets,
        //     func: data,
        // })
        // .abi_encode();

        // Ok(call)
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
    use crate::{provider::{SmartAccountProvider, SmartAccountProviderTrait}, types::{AccountCall, UserOperation, UserOperationRequest}};

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
    const SIMPLE_ACCOUNT_FACTORY_ADDRESS: &str = "0x91E60e0613810449d098b0b5Ec8b51A0FE8c8985";//"0x9406Cc6185a346906296840746125a0E44976454";

    #[tokio::test]
    async fn test_account_init_code() {
        let signer: PrivateKeySigner =
            "4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318"
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
        let signer: PrivateKeySigner =
            "4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318"
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
        let signer: PrivateKeySigner =
            "4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318"
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

    #[tokio::test]
    async fn test_sign_user_op() {
        let signer: PrivateKeySigner =
            "4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318"
                .parse()
                .unwrap();

        let address: Address = signer.address();
        let wallet = EthereumWallet::from(signer.clone());

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

        let user_op = UserOperation {
            sender: target_address,
            nonce: U256::from(1),
            factory: None,
            factory_data: None,
            call_data: Bytes::default(),
            call_gas_limit: U256::ZERO,
            verification_gas_limit: U256::from(21000),
            pre_verification_gas: U256::ZERO,
            max_fee_per_gas: U256::ZERO,
            max_priority_fee_per_gas: U256::ZERO,
            paymaster: None,
            paymaster_verification_gas_limit: None,
            paymaster_post_op_gas_limit: None,
            paymaster_data: None,
            signature: Bytes::default(),
        };

        let result = account.sign_user_op(user_op, &signer).await.unwrap();

        let expected_signature: Bytes = "0x20cef8f1e5b636465cabaa6091be01b06d06afe27591892668498e76b4bc9b2d0e454f5e4a42233b243880a92ea906e3be6f064523d67974da53306d4cc746ef1c".parse().unwrap();
        
        assert_eq!(result, expected_signature);
    }

    #[tokio::test]
    async fn test_estimate_user_op() {
        let signer: PrivateKeySigner =
            "82aba1f2ce3d1a0f6eca0ade8877077b7fc6fd06fb0af48ab4a53650bde69979"
                .parse()
                .unwrap();

        let wallet = EthereumWallet::from(signer.clone());
        let rpc_url = Url::parse("https://base-sepolia.g.alchemy.com/v2/IVqOyg3PqHzBQJMqa_yZAfyonF9ne2Gx").unwrap();
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url);

        let account = SimpleAccount::new(
            Arc::new(provider.clone()),
            signer.address(),
            Address::from_str(SIMPLE_ACCOUNT_FACTORY_ADDRESS).unwrap(),
            Address::from_str(ENTRY_POINT_ADDRESS).unwrap(),
            84532,
        );

        let nonce = account.get_nonce().await.unwrap();

        let to_address: Address = "0xde3e943a1c2211cfb087dc6654af2a9728b15536"
            .parse()
            .unwrap();

        let sender: Address = "0xd03d38efd09e8ba5e2108d602059886c4c4ffefd"
            .parse()
            .unwrap();

        let req = UserOperationRequest::new(AccountCall::Execute(ExecuteCall::new(
            to_address,
            U256::from(100),
            Bytes::new(),
        )))
        .sender(sender)
        .max_fee_per_gas(U256::from(100000))
        .max_priority_fee_per_gas(U256::from(10000))
        .factory(Address::from_str(SIMPLE_ACCOUNT_FACTORY_ADDRESS).unwrap())
        .factory_data("0x5fbfb9cf000000000000000000000000a666d9ebcc3feecf8e09c050c9c2379df1e5b3330000000000000000000000000000000000000000000000000000000000000000")
        .call_data("0xb61d27f6000000000000000000000000de3e943a1c2211cfb087dc6654af2a9728b15536000000000000000000000000000000000000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000000")
        .signature("0xfffffffffffffffffffffffffffffff0000000000000000000000000000000007aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1c")
        .nonce(nonce);

        let smart_account_provider = SmartAccountProvider::new(provider, account);
        let result = smart_account_provider
            .estimate_user_operation_gas(&req.with_defaults(), Address::from_str(ENTRY_POINT_ADDRESS).unwrap())
            .await;

        println!("Gas estimation result: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_transaction() {
        let signer: PrivateKeySigner =
            "82aba1f2ce3d1a0f6eca0ade8877077b7fc6fd06fb0af48ab4a53650bde69979"
                .parse()
                .unwrap();

        println!("Signer address: {:?}", signer.address());

        let wallet = EthereumWallet::from(signer.clone());
        let rpc_url =
            Url::parse("https://base-sepolia.g.alchemy.com/v2/IVqOyg3PqHzBQJMqa_yZAfyonF9ne2Gx")
                .unwrap();
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url);

        let account = SimpleAccount::new(
            Arc::new(provider.clone()),
            signer.address(),
            Address::from_str(SIMPLE_ACCOUNT_FACTORY_ADDRESS).unwrap(),
            Address::from_str(ENTRY_POINT_ADDRESS).unwrap(),
            84532,
        );

        let to_address: Address = "0xde3e943a1c2211cfb087dc6654af2a9728b15536"
            .parse()
            .unwrap();

        let req = UserOperationRequest::new(AccountCall::Execute(ExecuteCall::new(
            to_address,
            U256::from(100),
            Bytes::default(),
        )));

        let smart_account_provider = SmartAccountProvider::new(provider, account);
        let result = smart_account_provider
            .send_user_operation(req, &signer, Address::from_str(ENTRY_POINT_ADDRESS).unwrap())
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
