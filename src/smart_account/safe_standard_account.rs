use super::{AccountError, BaseAccount, SmartAccountSigner};

use crate::contracts::safe_proxy_factory::{SafeProxyFactory, SafeProxyFactoryCalls};
use crate::contracts::{
    self, safe_proxy_factory, EnableModulesCall, ExecuteUserOpCall, Safe4337ModuleCalls, SafeAddModuleLib, SafeL2Calls
};
use crate::contracts::{EntryPoint as EthersEntryPoint};
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

const RPC_URL: &str = "https://base-sepolia.g.alchemy.com/v2/HvnvemJhpDTfxwhfcGGnXHuo_dtgZVN6";//"https://eth-goerli.g.alchemy.com/v2/Lekp6yzHz5yAPLKPNvGpMKaqbGunnXHS"; //"https://eth-mainnet.g.alchemy.com/v2/lRcdJTfR_zjZSef3yutTGE6OIY9YFx1E";

const SAFE_4337_MODULE_ADDRESS: &str = "0xa581c4A4DB7175302464fF3C06380BC3270b4037"; // 
const ADD_MODULES_LIB_ADDRESS: &str = "0x8EcD4ec46D4D2a6B64fE960B3D64e8B94B2234eb"; //
const SAFE_PROXY_FACTORY_ADDRESS: &str = "0x4e1DCf7AD4e460CfD30791CCC4F9c8a4f820ec67"; //
const SAFE_SINGLETON_ADDRESS: &str = "0x29fcB43b46531BcA003ddC8FCB67FFE91900C762"; //
const ENTRYPOINT_ADDRESS: &str = "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789";
const FALLBACK_HANDLER_ADDRESS: &str = "0xfd0732Dc9E303f09fCEf3a7388Ad10A83459Ec99";

const SafeProxyBytecode: &str = "0x608060405234801561001057600080fd5b506040516101e63803806101e68339818101604052602081101561003357600080fd5b8101908080519060200190929190505050600073ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614156100ca576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260228152602001806101c46022913960400191505060405180910390fd5b806000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505060ab806101196000396000f3fe608060405273ffffffffffffffffffffffffffffffffffffffff600054167fa619486e0000000000000000000000000000000000000000000000000000000060003514156050578060005260206000f35b3660008037600080366000845af43d6000803e60008114156070573d6000fd5b3d6000f3fea264697066735822122003d1488ee65e08fa41e58e888a9865554c535f2c77126a82cb4c0f917f31441364736f6c63430007060033496e76616c69642073696e676c65746f6e20616464726573732070726f7669646564";

// keccak256(toUtf8Bytes('Safe Account Abstraction'))
const PREDETERMINED_SALT_NONCE: &str =
  "0xb1073742015cbcf5a3a4d9d1ae33ecf619439710b89475f92e2abd2117e90f90";


#[derive(Debug)]
struct SafeStandardAccount {
    inner: Arc<Provider<Http>>,
    owners: Vec<Address>,
    threshold: U256,
    account_address: RwLock<Option<Address>>,
    factory_address: Address,
    is_deployed: RwLock<bool>,
    entry_point: Arc<EthersEntryPoint<Provider<Http>>>,
    chain: Chain,
}

impl SafeStandardAccount {
    fn new(
        inner: Arc<Provider<Http>>,
        owners: Vec<Address>,
        threshold: U256,
        account_address: RwLock<Option<Address>>,
        factory_address: Address,
        entry_point_address: Address,
        is_deployed: RwLock<bool>,
        chain: Chain,
    ) -> Self {
        let entry_point = Arc::new(EthersEntryPoint::new(entry_point_address, inner.clone()));

        Self {
            inner,
            owners,
            threshold,
            account_address,
            factory_address,
            is_deployed,
            entry_point,
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
            return Ok(address)
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
        let fallback_handler_address: Address = safe_4337_module_address;//FALLBACK_HANDLER_ADDRESS.parse().unwrap();
        let setup_call_params = contracts::safe_l2::SetupCall {
            owners: self.owners.clone(),
            threshold: self.threshold,
            to: add_modules_lib_address,
            data: encoded_enable_modules_call,
            fallback_handler: fallback_handler_address,
            payment_token: Address::zero(),
            payment: U256::zero(),
            payment_receiver: Address::zero()
        };
        let setup_call = SafeL2Calls::Setup(setup_call_params);

        let singleton_address: Address = SAFE_SINGLETON_ADDRESS.parse().unwrap();
        let create_proxy_with_nonce_call_params = safe_proxy_factory::CreateProxyWithNonceCall {
            singleton: singleton_address,
            initializer: setup_call.encode().into(),
            salt_nonce: index, //PREDETERMINED_SALT_NONCE,
        };
        let create_proxy_with_nonce_call = SafeProxyFactoryCalls::CreateProxyWithNonce(create_proxy_with_nonce_call_params);

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
        let call = Safe4337ModuleCalls::ExecuteUserOp(ExecuteUserOpCall{
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
    use std::time::Duration;

    use ethers::signers::{LocalWallet, Signer};
    use tokio::time;
    use url::Url;

    use crate::{smart_account::{SmartAccountMiddleware, SmartAccountProvider}, types::{AccountCall, UserOperationRequest}};

    use super::*;

    #[tokio::test]
    async fn test_get_safe_address() {
        let factory_address: Address = SAFE_PROXY_FACTORY_ADDRESS.parse().unwrap();
        let entry_point_address: Address = ENTRYPOINT_ADDRESS.parse().unwrap();
        let wallet: LocalWallet =
            "82aba1f2ce3d1a0f6eca0ade8877077b7fc6fd06fb0af48ab4a53650bde69979"
                .parse()
                .unwrap();

        // let account_address: Address = "0x8898886f1adacdb475a8c6778d8c3a011e2c54a6"
        //     .parse()
        //     .unwrap();
        let provider = Provider::<Http>::try_from(RPC_URL).unwrap();

        let account = SafeStandardAccount::new(
            Arc::new(provider), 
            vec![wallet.address()], 
            U256::one(), 
            RwLock::new(None), //RwLock::new(Some(account_address)), 
            factory_address, 
            entry_point_address, 
            RwLock::new(false), 
            Chain::BaseSepolia
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

        // let account_address: Address = "0x8898886f1adacdb475a8c6778d8c3a011e2c54a6"
        //     .parse()
        //     .unwrap();
        let provider = Provider::<Http>::try_from(RPC_URL).unwrap();

        let account = SafeStandardAccount::new(
            Arc::new(provider), 
            vec![wallet.address()], 
            U256::one(), 
            RwLock::new(None), //RwLock::new(Some(account_address)), 
            factory_address, 
            entry_point_address, 
            RwLock::new(false), 
            Chain::BaseSepolia
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

    fn make_provider(account: SafeStandardAccount) -> SmartAccountProvider<Http, SafeStandardAccount> {
        let url: Url = RPC_URL.try_into().unwrap();
        let http_provider = Http::new(url);

        let account_provider = SmartAccountProvider::new(http_provider, account);

        account_provider
    }
}
