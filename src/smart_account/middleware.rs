use alloy::{primitives::Bytes, providers::Provider};
use async_trait::async_trait;
// use ethers::{
//     providers::{JsonRpcClient, Provider},
//     types::Bytes,
// };
use std::error::Error;
use std::fmt::Debug;

use crate::types::{
    FromErr, UserOpHash, UserOperation, UserOperationGasEstimate, UserOperationReceipt,
    UserOperationRequest,
};

use super::{BaseAccount, SmartAccountSigner};
use thiserror::Error;

#[async_trait]
pub trait SmartAccountMiddleware: Sync + Send + Debug {
    type Error: Sync + Send + Error + FromErr<<Self::Inner as SmartAccountMiddleware>::Error>;
    type Provider: JsonRpcClient;
    type Account: BaseAccount;
    type Inner: SmartAccountMiddleware<Provider = Self::Provider>;

    /// The next middleware in the stack
    fn inner(&self) -> &Self::Inner;

    /// The HTTP or Websocket provider.
    fn provider(&self) -> &Provider<Self::Provider> {
        self.inner().provider()
    }

    async fn send_user_operation<
        U: Into<UserOperationRequest> + Send + Sync,
        S: SmartAccountSigner,
    >(
        &self,
        user_op: U,
        // TODO: Passing in signer through method param for now. Consider separate signer middleware.
        signer: &S,
    ) -> Result<UserOpHash, Self::Error> {
        self.inner()
            .send_user_operation(user_op, signer)
            .await
            .map_err(FromErr::from)
    }

    async fn fill_user_operation(
        &self,
        user_op: &mut UserOperationRequest,
    ) -> Result<(), Self::Error> {
        self.inner()
            .fill_user_operation(user_op)
            .await
            .map_err(FromErr::from)
    }

    async fn sign_user_operation<S: SmartAccountSigner>(
        &self,
        user_op: UserOperationRequest,
        // TODO: Passing in signer through method param for now. Consider separate signer middleware.
        signer: &S,
    ) -> Result<Bytes, Self::Error> {
        self.inner()
            .sign_user_operation(user_op, signer)
            .await
            .map_err(FromErr::from)
    }

    async fn estimate_user_operation_gas(
        &self,
        user_op: &UserOperationRequest,
    ) -> Result<UserOperationGasEstimate, Self::Error> {
        self.inner()
            .estimate_user_operation_gas(user_op)
            .await
            .map_err(FromErr::from)
    }

    async fn get_user_operation<T: Send + Sync + Into<UserOpHash>>(
        &self,
        user_op_hash: T,
    ) -> Result<Option<UserOperation>, Self::Error> {
        self.inner()
            .get_user_operation(user_op_hash)
            .await
            .map_err(FromErr::from)
    }

    async fn get_user_operation_receipt<T: Send + Sync + Into<UserOpHash>>(
        &self,
        user_op_hash: T,
    ) -> Result<Option<UserOperationReceipt>, Self::Error> {
        self.inner()
            .get_user_operation_receipt(user_op_hash)
            .await
            .map_err(FromErr::from)
    }

    async fn get_supported_entry_points(&self) -> Result<Vec<String>, Self::Error> {
        self.inner()
            .get_supported_entry_points()
            .await
            .map_err(FromErr::from)
    }

    async fn get_paymaster_and_data<U: Into<UserOperationRequest> + Send + Sync>(
        &self,
        user_op: U,
    ) -> Result<Bytes, Self::Error> {
        self.inner()
            .get_paymaster_and_data(user_op.into())
            .await
            .map_err(FromErr::from)
    }
}

#[derive(Error, Debug)]
/// Thrown when an error happens at the smart account
pub enum SmartAccountMiddlewareError<M: SmartAccountMiddleware> {
    /// Thrown when an internal middleware errors
    #[error(transparent)]
    MiddlewareError(M::Error),
}
