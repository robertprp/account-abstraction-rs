use super::{AccountError, BaseAccount};

use crate::contracts::EntryPoint;
use crate::contracts::{CreateAccountCall, SimpleAccountFactoryCalls};
use crate::paymaster::Paymaster;

use async_trait::async_trait;
use ethers::abi::AbiEncode;
use ethers::providers::Http;
use ethers::{
    providers::Provider,
    types::{Address, Bytes, U256},
};
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
struct SimpleAccount {
    inner: Arc<Provider<Http>>,
    account_address: RwLock<Option<Address>>,
    is_deployed: RwLock<bool>,
}

#[async_trait]
impl BaseAccount for SimpleAccount {
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
        unimplemented!()
    }

    fn get_entry_point_address(&self) -> Address {
        "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789"
            .parse()
            .unwrap()
    }

    fn get_entry_point(&self) -> EntryPoint<Self::Inner> {
        let address: Address = self.get_entry_point_address();
        EntryPoint::new(address, self.inner.clone())
    }

    fn get_paymaster(&self) -> Option<Box<dyn Paymaster + Send + Sync>> {
        unimplemented!()
    }

    async fn get_account_init_code(&self) -> Result<Bytes, AccountError<Self::Inner>> {
        let factory_address: Address = "0x9406Cc6185a346906296840746125a0E44976454"
            .parse()
            .unwrap();

        let owner: Address = "".parse().unwrap();

        let index = U256::from(1);

        let call =
            SimpleAccountFactoryCalls::CreateAccount(CreateAccountCall { owner, salt: index });

        let mut result: Vec<u8> = Vec::new();

        result.extend_from_slice(factory_address.as_bytes());
        result.extend_from_slice(&call.encode());

        let result_bytes = Bytes::from(result);

        Ok(result_bytes)
    }

    async fn get_nonce(&self) -> Result<U256, AccountError<Self::Inner>> {
        unimplemented!() // You will need to provide an actual implementation.
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
        data: &Bytes,
    ) -> Result<Vec<u8>, AccountError<Self::Inner>> {
        unimplemented!() // You will need to provide an actual implementation.
    }

    async fn sign_user_op_hash(
        &self,
        user_op_hash: [u8; 32],
    ) -> Result<Bytes, AccountError<Self::Inner>> {
        unimplemented!() // You will need to provide an actual implementation.
    }
}
