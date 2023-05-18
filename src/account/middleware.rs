use super::{base_account::BaseAccount, AccountError};

use crate::types::FromErr;

use async_trait::async_trait;
use ethers::providers::{Middleware, MiddlewareError};
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
