use super::{AccountError, BaseAccount};

use crate::contracts::{CreateAccountCall, SimpleAccountFactoryCalls};
use crate::contracts::{EntryPoint, ExecuteCall, SimpleAccountCalls, UserOperation};
use crate::paymaster::{Paymaster, PaymasterError};

use async_trait::async_trait;
use ethers::abi::AbiEncode;
use ethers::providers::Http;
use ethers::signers::Signer;
use ethers::{
    providers::Provider,
    types::{Address, Bytes, U256},
};
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::RwLock;

const ENTRY_POINT_ADDRESS: &str = "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789";
const SIMPLE_ACCOUNT_FACTORY_ADDRESS: &str = "0x9406Cc6185a346906296840746125a0E44976454";

#[derive(Debug)]
struct SimpleAccount {
    inner: Arc<Provider<Http>>,
    owner: Address,
    account_address: RwLock<Option<Address>>,
    is_deployed: RwLock<bool>,
    rpc_url: String,
}

impl SimpleAccount {
    fn new(
        inner: Arc<Provider<Http>>,
        owner: Address,
        account_address: RwLock<Option<Address>>,
        is_deployed: RwLock<bool>,
        rpc_url: String,
    ) -> Self {
        Self {
            inner,
            owner,
            account_address,
            is_deployed,
            rpc_url,
        }
    }
}

#[async_trait]
impl BaseAccount for SimpleAccount {
    type Paymaster = EmptyPaymaster;
    type Provider = Http;
    type Inner = Provider<Http>;

    fn inner(&self) -> &Self::Inner {
        &self.inner
    }

    async fn get_account_address(&self) -> Result<Address, AccountError<Self::Inner>> {
        let Some(account_address) = *self.account_address.read().await else {
            return self.get_counterfactual_address().await
        };

        Ok(account_address)
    }

    fn get_rpc_url(&self) -> &str {
        self.rpc_url.as_str()
    }

    fn get_entry_point_address(&self) -> Address {
        ENTRY_POINT_ADDRESS.parse().unwrap()
    }

    fn get_entry_point(&self) -> EntryPoint<Self::Inner> {
        let address: Address = self.get_entry_point_address();
        EntryPoint::new(address, self.inner.clone())
    }

    fn get_paymaster(&self) -> Option<Self::Paymaster> {
        None
    }

    async fn get_account_init_code(&self) -> Result<Bytes, AccountError<Self::Inner>> {
        let factory_address: Address = SIMPLE_ACCOUNT_FACTORY_ADDRESS.parse().unwrap();

        let owner: Address = self.owner;

        // TODO: Add optional index
        let index = U256::from(0);

        let call =
            SimpleAccountFactoryCalls::CreateAccount(CreateAccountCall { owner, salt: index });

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

    async fn encode_execute(
        &self,
        target: Address,
        value: U256,
        data: Bytes,
    ) -> Result<Vec<u8>, AccountError<Self::Inner>> {
        let call = SimpleAccountCalls::Execute(ExecuteCall {
            dest: target,
            value,
            func: data,
        });

        Ok(call.encode())
    }

    async fn sign_user_op_hash<S: Signer>(
        &self,
        user_op_hash: [u8; 32],
        signer: &S,
    ) -> Result<Bytes, AccountError<Self::Inner>> {
        let Ok(signed_hash) = signer.sign_message(&user_op_hash).await else {
            return Err(AccountError::SignerError);
        };

        Ok(signed_hash.to_vec().into())
    }
}

#[derive(Debug)]
struct EmptyPaymaster;

#[async_trait]
impl Paymaster for EmptyPaymaster {
    async fn get_paymaster_and_data(
        &self,
        _user_op: UserOperation,
    ) -> Result<Bytes, PaymasterError> {
        Ok(Bytes::new())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ethers::{
        prelude::k256::ecdsa::SigningKey,
        providers::{Http, Provider},
        signers::{LocalWallet, Signer, Wallet},
        types::{Address, Bytes, U256},
    };
    use tokio::sync::RwLock;

    use crate::{
        account::{simple_account::SimpleAccount, BaseAccount, SmartAccountMiddleware},
        types::UserOperationRequest,
    };

    const RPC_URL: &str = "https://eth-goerli.g.alchemy.com/v2/Lekp6yzHz5yAPLKPNvGpMKaqbGunnXHS"; //"https://eth-mainnet.g.alchemy.com/v2/lRcdJTfR_zjZSef3yutTGE6OIY9YFx1E";

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

        let expected_signature: Bytes = "0xf638e4980e8e2244d951c212caeadb31e4ec53629c1c743e2046393ecfa065da2cbaeb4460ee7ac6f7c5e9b52d94c6198dd03a50ff852f1e6cc118d603b8db631c".parse().unwrap();

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
            .encode_execute(target_address, U256::from(100), call_data)
            .await
            .unwrap()
            .into();

        let expected_result: Bytes = "0xb61d27f6000000000000000000000000a87395ef99fc13bb043245521c559030aa1827a7000000000000000000000000000000000000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000028a71bbebe00000000000000000000000000000000000000000000000000000000000000010021fb3f000000000000000000000000000000000000000000000000".parse().unwrap();

        assert_eq!(result, expected_result)
    }

    fn make_simple_account() -> SimpleAccount {
        let account_address: Address = make_wallet().address();
        let provider = Provider::<Http>::try_from(RPC_URL).unwrap();

        SimpleAccount::new(
            Arc::new(provider),
            account_address,
            RwLock::new(None),
            RwLock::new(false),
            RPC_URL.to_string(),
        )
    }

    fn make_wallet() -> Wallet<SigningKey> {
        "4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318"
            .parse()
            .unwrap()
    }
}
