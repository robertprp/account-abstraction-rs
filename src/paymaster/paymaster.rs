use crate::types::UserOperationRequest;
use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait Paymaster: Sync + Send + Debug {
    type Error;

    async fn get_paymaster_and_data<U>(&self, user_op: &mut U) -> Result<(), Self::Error>
    where
        U: Into<UserOperationRequest> + Send + Sync;
}
