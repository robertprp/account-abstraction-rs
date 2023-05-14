use async_trait::async_trait;
use std::{error::Error, fmt::Debug};
use ethers::{providers::{Middleware, JsonRpcClient, Provider}, types::{U256, NameOrAddress, Bytes}};

#[async_trait]
pub trait BaseAccount: Sync + Send + Debug {
    type Error: Sync + Send + Error + FromErr<<Self::Inner as Middleware>::Error>;
    type Provider: JsonRpcClient;
    type Inner: Middleware<Provider = Self::Provider>;

    fn inner(&self) -> &Self::Inner;

    fn get_account_address(&self) -> NameOrAddress;
    
    fn provider(&self) -> &Provider<Self::Provider> {
        self.inner().provider()
    }

    async fn get_account_init_code(&self) -> Result<Bytes, Self::Error> {
        self.inner().get_code(self.get_account_address(), None).await.map_err(FromErr::from)
    }

    // TODO: Could also make it sync and have a initialize method on BaseAccount
    async fn get_nonce(&self) -> Result<U256, Self::Error> {
        self.inner().get_transaction_count(self.get_account_address(), None).await.map_err(FromErr::from)
    }

    async fn encode_execute(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;

    async fn sign_user_op_hash(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait FromErr<T> {
    fn from(src: T) -> Self;
}