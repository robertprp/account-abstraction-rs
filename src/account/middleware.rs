use super::{base_account::BaseAccount, AccountError};

use crate::types::{
    user_operation::{UserOpHash, UserOperationRequest},
    FromErr,
};

use async_trait::async_trait;
use ethers::{
    providers::{Middleware, MiddlewareError, ProviderError},
    types::Bytes,
    utils,
};
use std::fmt::Debug;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct SmartAccountMiddleware<M, A> {
    inner: M,
    account: A,
}

impl<M, A> SmartAccountMiddleware<M, A>
where
    M: Middleware,
    A: BaseAccount,
{
    pub fn new(inner: M, account: A) -> Self {
        Self { inner, account }
    }

    async fn send_user_operation<U: Into<UserOperationRequest> + Send + Sync>(
        &self,
        user_op: U,
    ) -> Result<UserOpHash, SmartAccountMiddlewareError<M>>
    where
        A: BaseAccount<Inner = M>,
    {
        let mut user_op: UserOperationRequest = user_op.into();
        self.fill_user_operation(&mut user_op).await?;

        if user_op.signature.is_none() {
            let signature: Bytes = self.sign_user_operation(user_op.clone()).await?;
            user_op.signature = Some(signature)
        }

        self.inner()
            .provider()
            .request("eth_sendUserOperation", utils::serialize(&user_op))
            .await
            .map_err(SmartAccountMiddlewareError::ProviderError)
    }

    async fn fill_user_operation(
        &self,
        user_op: &mut UserOperationRequest,
    ) -> Result<(), SmartAccountMiddlewareError<M>>
    where
        A: BaseAccount<Inner = M>,
    {
        if user_op.nonce.is_none() {
            let nonce = self
                .account
                .get_nonce()
                .await
                .map_err(SmartAccountMiddlewareError::AccountError)?;

            user_op.set_nonce(nonce);
        }

        if user_op.max_fee_per_gas.is_none() || user_op.max_priority_fee_per_gas.is_none() {
            let (max_fee_per_gas, max_priority_fee_per_gas) =
                self.estimate_eip1559_fees(None).await?;

            if user_op.max_priority_fee_per_gas.is_none() {
                user_op.max_priority_fee_per_gas = Some(max_priority_fee_per_gas);
            }

            if user_op.max_fee_per_gas.is_none() {
                user_op.max_fee_per_gas = Some(max_fee_per_gas);
            }
        }

        Ok(())
    }

    async fn sign_user_operation(
        &self,
        user_op: UserOperationRequest,
    ) -> Result<Bytes, SmartAccountMiddlewareError<M>>
    where
        A: BaseAccount<Inner = M>,
    {
        self.account
            .sign_user_op(user_op)
            .await
            .map_err(|e| SmartAccountMiddlewareError::AccountError(e))
    }

}

#[async_trait]
impl<M, A> Middleware for SmartAccountMiddleware<M, A>
where
    M: Middleware,
    A: BaseAccount,
{
    type Error = SmartAccountMiddlewareError<M>;
    type Provider = M::Provider;
    type Inner = M;

    fn inner(&self) -> &M {
        &self.inner
    }
}

#[derive(Error, Debug)]
/// Thrown when an error happens at the smart account
pub enum SmartAccountMiddlewareError<M: Middleware> {
    /// Thrown when an internal middleware errors
    #[error(transparent)]
    MiddlewareError(M::Error),

    #[error("account error {0}")]
    AccountError(AccountError<M>),

    #[error("provider error {0}")]
    ProviderError(ProviderError),
}

impl<M: Middleware> MiddlewareError for SmartAccountMiddlewareError<M> {
    type Inner = M::Error;

    fn from_err(src: M::Error) -> Self {
        SmartAccountMiddlewareError::MiddlewareError(src)
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        match self {
            SmartAccountMiddlewareError::MiddlewareError(e) => Some(e),
            _ => None,
        }
    }
}

impl<M: Middleware> FromErr<M::Error> for SmartAccountMiddlewareError<M> {
    fn from(src: M::Error) -> SmartAccountMiddlewareError<M> {
        SmartAccountMiddlewareError::MiddlewareError(src)
    }
}
