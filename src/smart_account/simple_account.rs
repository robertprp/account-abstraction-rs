use super::{AccountError, BaseAccount, SmartAccountSigner};

use crate::contracts::{simple_account, simple_account_factory, SimpleAccountFactoryCalls};
use crate::contracts::{EntryPoint as EthersEntryPoint, ExecuteBatchCall, SimpleAccountCalls};
use crate::types::ExecuteCall;

use async_trait::async_trait;
use ethers::abi::AbiEncode;
use ethers::providers::Http;
use ethers::types::Chain;
use ethers::{
    providers::Provider,
    types::{Address, Bytes, U256},
};
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::RwLock;

// const ENTRY_POINT_ADDRESS: &str = "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789";
// const SIMPLE_ACCOUNT_FACTORY_ADDRESS: &str = "0x9406Cc6185a346906296840746125a0E44976454";

#[derive(Debug)]
pub struct SimpleAccount {
    inner: Arc<Provider<Http>>,
    owner: Address,
    account_address: RwLock<Option<Address>>,
    factory_address: Address,
    is_deployed: RwLock<bool>,
    entry_point: Arc<EthersEntryPoint<Provider<Http>>>,
    chain: Chain,
}

impl SimpleAccount {
    pub fn new(
        inner: Arc<Provider<Http>>,
        owner: Address,
        account_address: RwLock<Option<Address>>,
        factory_address: Address,
        entry_point_address: Address,
        is_deployed: RwLock<bool>,
        chain: Chain,
    ) -> Self {
        let entry_point = Arc::new(EthersEntryPoint::new(entry_point_address, inner.clone()));

        Self {
            inner,
            owner,
            account_address,
            factory_address,
            is_deployed,
            entry_point,
            chain,
        }
    }
}

#[async_trait]
impl BaseAccount for SimpleAccount {
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
            *self.account_address.write().await = Some(address);
            return Ok(address);
        };

        Ok(account_address)
    }

    async fn get_account_init_code(&self) -> Result<Bytes, AccountError> {
        let factory_address: Address = self.factory_address;
        let owner: Address = self.owner;

        // TODO: Add optional index
        let index = U256::from(0);

        let call =
            SimpleAccountFactoryCalls::CreateAccount(simple_account_factory::CreateAccountCall {
                owner,
                salt: index,
            });

        let mut result: Vec<u8> = Vec::new();

        result.extend_from_slice(factory_address.as_bytes());
        result.extend_from_slice(&call.encode());

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
        let call = SimpleAccountCalls::Execute(simple_account::ExecuteCall {
            dest: call.target,
            value: call.value,
            func: call.data,
        });

        Ok(call.encode())
    }

    async fn encode_execute_batch(&self, calls: Vec<ExecuteCall>) -> Result<Vec<u8>, AccountError> {
        let targets: Vec<Address> = calls.iter().map(|call| call.target).collect();
        let data: Vec<Bytes> = calls.iter().map(|call| call.data.clone()).collect();
        let multi_call = SimpleAccountCalls::ExecuteBatch(ExecuteBatchCall {
            dest: targets,
            func: data,
        });

        Ok(multi_call.encode())
    }

    async fn sign_user_op_hash<S: SmartAccountSigner>(
        &self,
        user_op_hash: [u8; 32],
        signer: &S,
    ) -> Result<Bytes, AccountError> {
        signer
            .sign_message(&user_op_hash)
            .await
            .map_err(|_| AccountError::SignerError)
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use ethers::{
        prelude::k256::ecdsa::SigningKey,
        providers::{Http, Provider},
        signers::{LocalWallet, Signer, Wallet},
        types::{Address, Bytes, Chain, U256},
    };
    use tokio::{sync::RwLock, time};
    use url::Url;

    use crate::{
        contracts::UserOperation,
        smart_account::{
            simple_account::SimpleAccount, BaseAccount, SmartAccountMiddleware,
            SmartAccountProvider,
        },
        types::{AccountCall, ExecuteCall, UserOperationRequest},
    };

    const RPC_URL: &str = "https://base-sepolia.g.alchemy.com/v2/HvnvemJhpDTfxwhfcGGnXHuo_dtgZVN6"; //"https://eth-goerli.g.alchemy.com/v2/Lekp6yzHz5yAPLKPNvGpMKaqbGunnXHS"; //"https://eth-mainnet.g.alchemy.com/v2/lRcdJTfR_zjZSef3yutTGE6OIY9YFx1E";
    const SIMPLE_ACCOUNT_FACTORY_ADDRESS: &str = "0x9406Cc6185a346906296840746125a0E44976454";
    const ENTRY_POINT_ADDRESS: &str = "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789";

    #[tokio::test]
    async fn test_get_counterfactual_address() {
        let account = make_simple_account();

        let result = account.get_counterfactual_address().await.unwrap();

        assert_eq!(
            result,
            "0x982ffac966b962bddf89d3b26fee91da6f68df13"
                .parse()
                .unwrap()
        )
    }

    #[tokio::test]
    async fn test_sign_user_op() {
        let account = make_simple_account();

        let wallet = make_wallet();

        let target_address: Address = "0xA87395ef99Fc13Bb043245521C559030aA1827a7"
            .parse()
            .unwrap();

        let user_op = crate::contracts::UserOperation {
            sender: target_address,
            nonce: U256::from(1),
            init_code: Bytes::from(vec![]),
            call_data: Bytes::from(vec![]),
            call_gas_limit: U256::from(0),
            verification_gas_limit: U256::from(21000),
            pre_verification_gas: U256::from(0),
            max_fee_per_gas: U256::from(0),
            max_priority_fee_per_gas: U256::from(0),
            paymaster_and_data: Bytes::from(vec![]),
            signature: Bytes::from(vec![]),
        };

        let result = account.sign_user_op(user_op, &wallet).await.unwrap();

        let expected_signature: Bytes = "0xe24cd218d33046a7f0f9d3a296ebb0f89d4bc34149a4ee29b036f101ace9d2f85b86451955472e607feca50b51451887a742cee69f16e6a15a9354abce4ab50c1b".parse().unwrap();

        assert_eq!(result, expected_signature)
    }

    #[tokio::test]
    async fn test_account_init_code() {
        let account = make_simple_account();

        let result = account.get_account_init_code().await.unwrap();

        let expected_init_code: Bytes = "0x9406cc6185a346906296840746125a0e449764545fbfb9cf0000000000000000000000002c7536e3605d9c16a7a3d7b1898e529396a65c230000000000000000000000000000000000000000000000000000000000000000".parse().unwrap();

        assert_eq!(result, expected_init_code)
    }

    #[tokio::test]
    async fn test_encode_execute() {
        let account = make_simple_account();
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
    async fn test_user_op_hash() {
        let account: SimpleAccount = make_simple_account();

        let user_op = UserOperation {
            sender: Address::zero(),
            nonce: U256::from(0),
            init_code: Bytes::from(vec![]),
            call_data: Bytes::from(vec![]),
            call_gas_limit: U256::from(0),
            verification_gas_limit: U256::from(0),
            pre_verification_gas: U256::from(0),
            max_fee_per_gas: U256::from(0),
            max_priority_fee_per_gas: U256::from(0),
            paymaster_and_data: Bytes::from(vec![]),
            signature: Bytes::from(vec![]),
        };

        let result: [u8; 32] = account.get_user_op_hash(user_op).await.unwrap();

        let expected_result: [u8; 32] = [
            133, 41, 35, 170, 101, 119, 97, 163, 97, 233, 30, 51, 120, 77, 163, 47, 199, 123, 148,
            96, 162, 191, 15, 241, 96, 138, 206, 27, 4, 83, 75, 151,
        ];

        assert_eq!(result, expected_result)
    }

    #[tokio::test]
    async fn test_user_op_hash_equals_onchain_hash() {
        let owner: Address = "0xde3e943a1c2211cfb087dc6654af2a9728b15536"
            .parse()
            .unwrap();

        let account = make_simple_account();

        let user_op = UserOperation {
            sender: owner,
            nonce: U256::from(1),
            init_code: account.get_account_init_code().await.unwrap(),
            call_data: Bytes::from(vec![]),
            call_gas_limit: U256::from(0),
            verification_gas_limit: U256::from(0),
            pre_verification_gas: U256::from(0),
            max_fee_per_gas: U256::from(0),
            max_priority_fee_per_gas: U256::from(0),
            paymaster_and_data: Bytes::from(vec![]),
            signature: Bytes::from(vec![]),
        };

        let onchain_hash = account.get_onchain_user_op_hash(user_op.clone()).await;
        let offchain_hash = account.get_user_op_hash(user_op.clone()).await.unwrap();

        assert!(onchain_hash == offchain_hash)
    }

    #[tokio::test]
    async fn test_estimate_user_op() {
        let wallet: LocalWallet =
            "82aba1f2ce3d1a0f6eca0ade8877077b7fc6fd06fb0af48ab4a53650bde69979"
                .parse()
                .unwrap();

        let account_address: Address = "0x8898886f1adacdb475a8c6778d8c3a011e2c54a6"
            .parse()
            .unwrap();
        let provider = Provider::<Http>::try_from(RPC_URL).unwrap();
        let factory_address: Address = SIMPLE_ACCOUNT_FACTORY_ADDRESS.parse().unwrap();
        let entry_point_address: Address = ENTRY_POINT_ADDRESS.parse().unwrap();

        println!("account address {:?}", account_address);
        let account = SimpleAccount::new(
            Arc::new(provider),
            wallet.address(),
            RwLock::new(Some(account_address)),
            factory_address,
            entry_point_address,
            RwLock::new(false),
            Chain::Goerli,
        );

        let nonce = account.get_nonce().await.unwrap();

        let provider = make_provider(account);

        let to_address: Address = "0xde3e943a1c2211cfb087dc6654af2a9728b15536"
            .parse()
            .unwrap();

        let sender: Address = "0x8898886f1adacdb475a8c6778d8c3a011e2c54a6"
            .parse()
            .unwrap();

        let req = UserOperationRequest::new()
            .call(AccountCall::Execute(ExecuteCall::new(
                to_address,
                100,
                Bytes::new(),
            )))
            .sender(sender)
            .max_fee_per_gas(100)
            .max_priority_fee_per_gas(10)
            .nonce(nonce);

        let result = provider
            .estimate_user_operation_gas(&req.with_defaults())
            .await;

        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_send_transaction() {
        let wallet: LocalWallet =
            "82aba1f2ce3d1a0f6eca0ade8877077b7fc6fd06fb0af48ab4a53650bde69979"
                .parse()
                .unwrap();

        let account_address: Address = "0x8898886f1adacdb475a8c6778d8c3a011e2c54a6"
            .parse()
            .unwrap();
        let provider = Provider::<Http>::try_from(RPC_URL).unwrap();
        let factory_address: Address = SIMPLE_ACCOUNT_FACTORY_ADDRESS.parse().unwrap();
        let entry_point_address: Address = ENTRY_POINT_ADDRESS.parse().unwrap();

        println!("account address {:?}", account_address);
        let account = SimpleAccount::new(
            Arc::new(provider),
            wallet.address(),
            RwLock::new(Some(account_address)),
            factory_address,
            entry_point_address,
            RwLock::new(false),
            Chain::Goerli,
        );

        let provider = make_provider(account);

        let to_address: Address = "0xde3e943a1c2211cfb087dc6654af2a9728b15536"
            .parse()
            .unwrap();

        let req = UserOperationRequest::new().call(AccountCall::Execute(ExecuteCall::new(
            to_address,
            100,
            Bytes::new(),
        )));

        let result = provider.send_user_operation(req, &wallet).await;

        let user_op_hash = result.unwrap();

        let mut interval = time::interval(Duration::from_secs(10));
        let mut attempts = 0;
        let max_attempts = 20;

        println!("user op hash {:?}", user_op_hash);

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

    // #[tokio::test]
    // async fn test_new_wallet() {
    //     let wallet = LocalWallet::new(&mut rand::thread_rng());

    //     // Bytes(0x82aba1f2ce3d1a0f6eca0ade8877077b7fc6fd06fb0af48ab4a53650bde69979)
    //     // 0xa666d9ebcc3feecf8e09c050c9c2379df1e5b333

    //     // SA 0x8898886f1adacdb475a8c6778d8c3a011e2c54a6
    //     println!("{:?}", wallet.address());

    //     let provider = Provider::<Http>::try_from(RPC_URL).unwrap();

    //     let account = SimpleAccount::new(
    //         Arc::new(provider),
    //         wallet.address(),
    //         RwLock::new(None),
    //         RwLock::new(false),
    //         RPC_URL.to_string(),
    //     );

    //     // let contract_address = account.get_counterfactual_address().await.unwrap();

    //     // println!("{:?}", contract_address);

    //     let middleware = SmartAccountMiddleware::new(Provider::<Http>::try_from("https://api.stackup.sh/v1/node/2e0bd6d2d67c8003121fb3ac53c3cd866dc2ce425f68f817d6e9b723fe3cfd5f").unwrap(), account);

    //     let to_address: Address = "0xde3e943a1c2211cfb087dc6654af2a9728b15536"
    //         .parse()
    //         .unwrap();

    //     let req = UserOperationRequest::new().call(AccountCall::Execute(ExecuteCall::new(
    //         to_address,
    //         100,
    //         Bytes::new(),
    //     )));
    //     // .target_address(to_address)
    //     // .tx_value(100);
    //     // .tx_data(Bytes::new());
    //     // .call_gas_limit(40000);
    //     let result = middleware.send_user_operation(req, &wallet).await;

    //     // fix : Err(ProviderError(JsonRpcClientError(JsonRpcError(JsonRpcError { code: -32602, message: "callGasLimit: below expected gas of 33100", data: Some(String("callGasLimit: below expected gas of 33100")) }))))

    //     println!("send result {:?}", result);

    //     let user_op_hash = result.unwrap();

    //     let mut interval = time::interval(Duration::from_secs(10));
    //     let mut attempts = 0;
    //     let max_attempts = 20;

    //     loop {
    //         interval.tick().await;
    //         attempts += 1;

    //         match middleware
    //             .get_user_operation_receipt(user_op_hash.clone())
    //             // .get_user_operation(user_op_hash)
    //             .await
    //         {
    //             Ok(receipt) => {
    //                 println!("Received receipt: {:?}", receipt);
    //                 break;
    //             }
    //             Err(e) => {
    //                 println!("Failed to get user operation receipt: {:?}", e);
    //                 if attempts >= max_attempts {
    //                     println!("Exceeded max attempts, stopping retries");
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    //     // let receipt = middleware.get_user_operation_receipt(result.unwrap()).await;
    //     //0x6303715bf1ecc999f96baf320896de93ff7951e908506e6ed68ac46190c09746
    //     // println!("receipt {:?}", receipt);
    // }

    // #[tokio::test]
    // async fn test_get_user_op() {
    //     let wallet: LocalWallet =
    //         "82aba1f2ce3d1a0f6eca0ade8877077b7fc6fd06fb0af48ab4a53650bde69979"
    //             .parse()
    //             .unwrap();

    //     // Bytes(0x82aba1f2ce3d1a0f6eca0ade8877077b7fc6fd06fb0af48ab4a53650bde69979)
    //     // 0xa666d9ebcc3feecf8e09c050c9c2379df1e5b333

    //     // SA 0x8898886f1adacdb475a8c6778d8c3a011e2c54a6
    //     println!("{:?}", wallet.address());

    //     let provider = Provider::<Http>::try_from(RPC_URL).unwrap();

    //     let account = SimpleAccount::new(
    //         Arc::new(provider),
    //         wallet.address(),
    //         RwLock::new(Some(
    //             "0x8898886f1adacdb475a8c6778d8c3a011e2c54a6"
    //                 .parse()
    //                 .unwrap(),
    //         )),
    //         RwLock::new(true),
    //         RPC_URL.to_string(),
    //     );
    //     let middleware = SmartAccountMiddleware::new(Provider::<Http>::try_from("https://api.stackup.sh/v1/node/2e0bd6d2d67c8003121fb3ac53c3cd866dc2ce425f68f817d6e9b723fe3cfd5f").unwrap(), account);

    //     let user_op_hash: UserOpHash =
    //         "0x6303715bf1ecc999f96baf320896de93ff7951e908506e6ed68ac46190c09746"
    //             .parse::<H256>()
    //             .unwrap()
    //             .into();

    //     let user_op = middleware.get_user_operation(user_op_hash).await;

    //     println!("{:?}", user_op);
    // }

    fn make_simple_account() -> SimpleAccount {
        let account_address: Address = make_wallet().address();
        let provider = Provider::<Http>::try_from(RPC_URL).unwrap();
        let factory_address: Address = SIMPLE_ACCOUNT_FACTORY_ADDRESS.parse().unwrap();
        let entry_point_address: Address = ENTRY_POINT_ADDRESS.parse().unwrap();

        println!("account address {:?}", account_address);
        SimpleAccount::new(
            Arc::new(provider),
            account_address,
            RwLock::new(None),
            factory_address,
            entry_point_address,
            RwLock::new(false),
            Chain::Goerli,
        )
    }

    fn make_provider(account: SimpleAccount) -> SmartAccountProvider<Http, SimpleAccount> {
        let url: Url = RPC_URL.try_into().unwrap();
        let http_provider = Http::new(url);

        let account_provider = SmartAccountProvider::new(http_provider, account);

        account_provider
    }

    fn make_wallet() -> Wallet<SigningKey> {
        "4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318"
            .parse()
            .unwrap()
    }

    impl SimpleAccount {
        async fn get_onchain_user_op_hash(&self, user_op: UserOperation) -> [u8; 32] {
            self.entry_point()
                .get_user_op_hash(user_op.into())
                .call()
                .await
                .unwrap()
        }
    }
}
