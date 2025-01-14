use crate::types::UserOperationRequest;
use async_trait::async_trait;

#[async_trait]
pub trait Paymaster {
    type Error;

    async fn get_paymaster_and_data<U>(&self, user_op: &mut U) -> Result<(), Self::Error>
    where
        U: Into<UserOperationRequest> + Send + Sync;
}
