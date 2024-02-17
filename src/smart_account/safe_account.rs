use super::{AccountError, BaseAccount, SmartAccountSigner};

use crate::contracts::safe_signer_launch_pad::SafeSignerLaunchPad;
use crate::contracts::{
    safe_add_module_lib, safe_proxy_factory, safe_signer_launch_pad, simple_account,
};
use crate::contracts::{EntryPoint as EthersEntryPoint, ExecuteBatchCall, SimpleAccountCalls};
use crate::types::ExecuteCall;

use async_trait::async_trait;
use ethers::abi::{encode, encode_packed, AbiEncode, Token};
use ethers::providers::Http;
use ethers::types::Chain;
use ethers::utils::{get_create2_address, keccak256};
use ethers::{
    providers::Provider,
    types::{Address, Bytes, U256},
};
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::RwLock;

// const ENTRY_POINT_ADDRESS: &str = "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789";
// const SIMPLE_ACCOUNT_FACTORY_ADDRESS: &str = "0x9406Cc6185a346906296840746125a0E44976454";

const RPC_URL: &str = "https://eth-goerli.g.alchemy.com/v2/Lekp6yzHz5yAPLKPNvGpMKaqbGunnXHS"; //"https://eth-mainnet.g.alchemy.com/v2/lRcdJTfR_zjZSef3yutTGE6OIY9YFx1E";

const SAFE_SIGNER_LAUNCHPAD_ADDRESS: &str = "0x845712F64E53AFb2bC0aB3C6559cA5BfC75eee50";
const SAFE_4337_MODULE_ADDRESS: &str = "0xD9752e2e2A577Fe7c4340d79b5DD2C9775656bD6";
const ADD_MODULES_LIB_ADDRESS: &str = "0x8EcD4ec46D4D2a6B64fE960B3D64e8B94B2234eb";
const WEBAUTHN_SIGNER_FACTORY_ADDRESS: &str = "0x329e21291fb2d7265aEf128D4A5c35287Bb0eF91";
const SAFE_PROXY_FACTORY_ADDRESS: &str = "0x4e1DCf7AD4e460CfD30791CCC4F9c8a4f820ec67";
const SAFE_SINGLETON_ADDRESS: &str = "0x29fcB43b46531BcA003ddC8FCB67FFE91900C762";
const ENTRYPOINT_ADDRESS: &str = "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789";
const XANDER_BLAZE_NFT_ADDRESS: &str = "0xBb9ebb7b8Ee75CDBf64e5cE124731A89c2BC4A07";

const SafeProxyBytecode: &str = "0x608060405234801561001057600080fd5b506040516101e63803806101e68339818101604052602081101561003357600080fd5b8101908080519060200190929190505050600073ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614156100ca576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260228152602001806101c46022913960400191505060405180910390fd5b806000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505060ab806101196000396000f3fe608060405273ffffffffffffffffffffffffffffffffffffffff600054167fa619486e0000000000000000000000000000000000000000000000000000000060003514156050578060005260206000f35b3660008037600080366000845af43d6000803e60008114156070573d6000fd5b3d6000f3fea264697066735822122003d1488ee65e08fa41e58e888a9865554c535f2c77126a82cb4c0f917f31441364736f6c63430007060033496e76616c69642073696e676c65746f6e20616464726573732070726f7669646564";

const ENTRY_POINT_ADDRESS: &str = "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789";

#[derive(Debug)]
struct SafeAccount {
    inner: Arc<Provider<Http>>,
    owner: Address,
    account_address: RwLock<Option<Address>>,
    factory_address: Address,
    is_deployed: RwLock<bool>,
    entry_point: Arc<EthersEntryPoint<Provider<Http>>>,
    safe_signer_launch_pad: Arc<SafeSignerLaunchPad<Provider<Http>>>,
    passkey: PasskeyLocalStorage,
    chain: Chain,
}

impl SafeAccount {
    fn new(
        inner: Arc<Provider<Http>>,
        owner: Address,
        account_address: RwLock<Option<Address>>,
        factory_address: Address,
        entry_point_address: Address,
        is_deployed: RwLock<bool>,
        chain: Chain,
    ) -> Self {
        let entry_point = Arc::new(EthersEntryPoint::new(entry_point_address, inner.clone()));
        let safe_signer_launch_pad_address: Address = ADD_MODULES_LIB_ADDRESS.parse().unwrap();
        let safe_signer_launch_pad = Arc::new(SafeSignerLaunchPad::new(
            safe_signer_launch_pad_address,
            inner.clone(),
        ));
        let fake_passkey = PasskeyLocalStorage {
            pubkey_coordinates: PubkeyCoordinates {
                x: U256::from(1),
                y: U256::from(1),
            },
        };

        Self {
            inner,
            owner,
            account_address,
            factory_address,
            is_deployed,
            entry_point,
            safe_signer_launch_pad,
            passkey: fake_passkey,
            chain,
        }
    }
}

#[async_trait]
impl BaseAccount for SafeAccount {
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
            return Ok(address)
        };

        Ok(account_address)
    }

    async fn get_account_init_code(&self) -> Result<Bytes, AccountError> {
        // TODO: Add optional index
        let index = U256::from(0);

        let signer_data = encode_packed(&[
            Token::Uint(self.passkey.pubkey_coordinates.x),
            Token::Uint(self.passkey.pubkey_coordinates.y),
        ])
        .map_err(|_| AccountError::EncodeError("signer_data".to_string()))?;

        let add_module_call = safe_add_module_lib::EnableModulesCall {
            modules: vec![SAFE_4337_MODULE_ADDRESS.parse().unwrap()],
        }
        .encode();

        let init_hash: [u8; 32] = self
            .get_init_hash(
                SAFE_SINGLETON_ADDRESS.parse().unwrap(),
                SAFE_4337_MODULE_ADDRESS.parse().unwrap(),
                WEBAUTHN_SIGNER_FACTORY_ADDRESS.parse().unwrap(),
                Bytes::from(signer_data),
                ADD_MODULES_LIB_ADDRESS.parse().unwrap(),
                add_module_call.into(),
            )
            .await?;

        let launch_pad_initializer: Vec<u8> = self.get_launchpad_initializer(init_hash);

        // let predicted_safe_address = self.get_safe_address(
        //     launch_pad_initializer.clone(),
        //     SAFE_PROXY_FACTORY_ADDRESS.parse().unwrap(),
        //     SAFE_SINGLETON_ADDRESS.parse().unwrap(),
        //     index,
        // )?;

        let safe_deployment_data: Vec<u8> = self.get_safe_deployment_data(
            SAFE_SIGNER_LAUNCHPAD_ADDRESS.parse().unwrap(),
            launch_pad_initializer.clone(),
            index,
        );

        // let call =
        //     SimpleAccountFactoryCalls::CreateAccount(simple_account_factory::CreateAccountCall {
        //         owner,
        //         salt: index,
        //     });

        // let mut result: Vec<u8> = Vec::new();

        // result.extend_from_slice(factory_address.as_bytes());
        // result.extend_from_slice(&call.encode());

        // let result_bytes = Bytes::from(result);

        Ok(Bytes::from(safe_deployment_data))
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

impl SafeAccount {}

impl SafeAccount {
    fn get_safe_address(
        &self,
        initializer: Vec<u8>,
        factory_address: Address,
        singleton: Address,
        salt_nonce: U256,
    ) -> Result<Address, AccountError> {
        let proxy_byte_code = SafeProxyBytecode.as_bytes();
        let deployment_code = encode_packed(&[
            Token::Bytes(proxy_byte_code.into()),
            Token::Address(singleton),
        ])
        .unwrap();

        let encoded_initializer: Vec<u8> =
            encode_packed(&[Token::Bytes(initializer.into()), Token::Address(singleton)])
                .map_err(|_| AccountError::EncodeError("initializer".to_string()))?;
        let hashed_initializer: [u8; 32] = keccak256(encoded_initializer);

        let encoded_salt_nonce: Vec<u8> = encode_packed(&[
            Token::FixedBytes(hashed_initializer.to_vec()),
            Token::Uint(salt_nonce),
        ])
        .map_err(|_| AccountError::EncodeError("salt_nonce".to_string()))?;
        let hashed_salt_nonce: [u8; 32] = keccak256(encoded_salt_nonce);

        let create2_address =
            get_create2_address(factory_address, hashed_salt_nonce, deployment_code);

        Ok(create2_address)
    }

    fn get_init_hash2(&self) -> Result<Bytes, AccountError> {
        // TODO: Passkey coordinates
        let signer_data = encode_packed(&[Token::Uint(U256::from(1)), Token::Uint(U256::from(1))])
            .map_err(|_| AccountError::EncodeError("signer_data".to_string()))?;

        let add_module_call = safe_add_module_lib::EnableModulesCall {
            modules: vec![SAFE_4337_MODULE_ADDRESS.parse().unwrap()],
        }
        .encode();

        let get_init_hash_call = safe_signer_launch_pad::GetInitHashCall {
            singleton: SAFE_SINGLETON_ADDRESS.parse().unwrap(),
            fallback_handler: SAFE_4337_MODULE_ADDRESS.parse().unwrap(),
            signer_factory: WEBAUTHN_SIGNER_FACTORY_ADDRESS.parse().unwrap(),
            signer_data: Bytes::from(signer_data),
            setup_to: ADD_MODULES_LIB_ADDRESS.parse().unwrap(),
            setup_data: add_module_call.into(),
        };

        Ok(get_init_hash_call.encode().into())
    }

    fn get_launchpad_initializer(&self, init_hash: [u8; 32]) -> Vec<u8> {
        let launchpad_initializer = safe_signer_launch_pad::PreValidationSetupCall {
            init_hash,
            to: Address::zero(),
            pre_init: Bytes::new(),
        };

        launchpad_initializer.encode()
    }

    fn get_safe_deployment_data(
        &self,
        singleton: Address,
        initializer: Vec<u8>,
        salt_nonce: U256,
    ) -> Vec<u8> {
        let create_call = safe_proxy_factory::CreateProxyWithNonceCall {
            initializer: Bytes::from(initializer),
            singleton,
            salt_nonce,
        };

        create_call.encode()
    }

    async fn get_init_hash(
        &self,
        singleton: Address,
        fallback_handler: Address,
        signer_factory: Address,
        signer_data: Bytes,
        setup_to: Address,
        setup_data: Bytes,
    ) -> Result<[u8; 32], AccountError> {
        self.safe_signer_launch_pad
            .get_init_hash(
                singleton,
                signer_factory,
                signer_data,
                setup_to,
                setup_data,
                fallback_handler,
            )
            .call()
            .await
            .map_err(|_| AccountError::SignerError)
    }
}

#[derive(Debug)]
pub struct PasskeyLocalStorage {
    // pub raw_id: String,
    pub pubkey_coordinates: PubkeyCoordinates,
}

#[derive(Debug)]
pub struct PubkeyCoordinates {
    pub x: U256,
    pub y: U256,
}

#[cfg(test)]
mod tests {
    use ethers::signers::{LocalWallet, Signer};

    use super::*;

    #[test]
    fn test_get_safe_address() {
        let factory_address: Address = SAFE_PROXY_FACTORY_ADDRESS.parse().unwrap();
        let entry_point_address: Address = ENTRY_POINT_ADDRESS.parse().unwrap();
        let wallet: LocalWallet =
            "82aba1f2ce3d1a0f6eca0ade8877077b7fc6fd06fb0af48ab4a53650bde69979"
                .parse()
                .unwrap();

        let account_address: Address = "0x8898886f1adacdb475a8c6778d8c3a011e2c54a6"
            .parse()
            .unwrap();
        let provider = Provider::<Http>::try_from(RPC_URL).unwrap();

        let account = SafeAccount::new(
            Arc::new(provider),
            wallet.address(),
            RwLock::new(Some(account_address)),
            factory_address,
            entry_point_address,
            RwLock::new(false),
            Chain::Goerli,
        );

        let initializer: Vec<u8> = vec![1, 2, 3];
        let salt_nonce: U256 = U256::from(123);
        let singleton: Address = SAFE_SINGLETON_ADDRESS.parse().unwrap();
        let result = account.get_safe_address(initializer, factory_address, singleton, salt_nonce);
        assert!(result.is_ok());

        let safe_address = result.unwrap();

        println!("Safe address: {}", safe_address.to_string());
        // Add assertions for the expected safe address value
        // For example:
        // assert_eq!(safe_address, Address::from_str("0x1234567890abcdef").unwrap());
    }
}
