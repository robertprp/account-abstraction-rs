use crate::contracts::UserOperation;

use async_trait::async_trait;
use ethers::types::Bytes;
use thiserror::Error;

#[async_trait]
pub trait Paymaster: Send + Sync {
    async fn get_paymaster_and_data(&self, user_op: UserOperation)
        -> Result<Bytes, PaymasterError>;
}

#[derive(Debug, Error)]
pub enum PaymasterError {
    #[error("custom error: {0}")]
    Custom(String),
}
