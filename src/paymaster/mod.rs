use crate::contracts::UserOperation;

use async_trait::async_trait;
use ethers::types::Bytes;
use thiserror::Error;
use std::fmt::Debug;

#[async_trait]
pub trait Paymaster: Sync + Send + Debug {
    async fn get_paymaster_and_data(&self, user_op: UserOperation)
        -> Result<Bytes, PaymasterError>;
}

#[derive(Debug, Error)]
pub enum PaymasterError {
    #[error("custom error: {0}")]
    Custom(String),
}
