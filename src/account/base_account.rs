use async_trait::async_trait;
use ethers::{providers::{Middleware, JsonRpcClient, Provider}, types::U256};

#[async_trait]
pub trait BaseAccount {

    type Provider: JsonRpcClient;
    type Inner: Middleware<Provider = Self::Provider>;

    fn inner(&self) -> &Self::Inner;
    
    fn provider(&self) -> &Provider<Self::Provider> {
        self.inner().provider()
    }

    async fn get_account_init_code(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;

    // TODO: Could also make it sync and have a initialize method on BaseAccount
    async fn get_nonce(&self) -> Result<U256, Box<dyn std::error::Error>>;

    async fn encode_execute(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;

    async fn sign_user_op_hash(&self) -> Result<(), Box<dyn std::error::Error>>;
}
