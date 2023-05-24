use super::{base_account::BaseAccount, AccountError};

use crate::{
    paymaster::{Paymaster, PaymasterError},
    types::{
        request::{UserOpHash, UserOperationRequest},
        FromErr, UserOperation, UserOperationReceipt,
    },
};

use async_trait::async_trait;
use ethers::{
    providers::{Middleware, MiddlewareError, ProviderError},
    signers::Signer,
    types::{Bytes, U256},
    utils,
};
use std::{fmt::Debug, ops::Add};
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

    pub fn account(&self) -> &A {
        &self.account
    }

    pub async fn send_user_operation<U: Into<UserOperationRequest> + Send + Sync, S: Signer>(
        &self,
        user_op: U,
        // TODO: Passing in signer through method param for now. Consider separate signer middleware.
        signer: &S,
    ) -> Result<UserOpHash, SmartAccountMiddlewareError<M>>
    where
        A: BaseAccount<Inner = M>,
    {
        let mut user_op: UserOperationRequest = user_op.into();
        self.fill_user_operation(&mut user_op).await?;

        if user_op.signature.is_none() {
            let signature: Bytes = self.sign_user_operation(user_op.clone(), signer).await?;
            user_op.signature = Some(signature)
        }

        self.inner()
            .provider()
            .request("eth_sendUserOperation", [serialized_user_op, serialized_entry_point_address])
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

        if user_op.sender.is_none() {
            user_op.sender = Some(
                self.account
                    .get_account_address()
                    .await
                    .map_err(SmartAccountMiddlewareError::AccountError)?,
            );
        }

        if let (Some(target), Some(data)) = (user_op.contract_target, user_op.tx_data.clone()) {
            let (call_data, call_gas_limit) = self
                .account
                .encode_user_op_call_data_and_gas_limit(
                    target,
                    user_op.tx_value,
                    data,
                    user_op.call_gas_limit,
                )
                .await
                .map_err(SmartAccountMiddlewareError::AccountError)?;

            user_op.call_data = Some(call_data);
            user_op.call_gas_limit = Some(call_gas_limit);
        }

        if user_op.init_code.is_none() {
            user_op.init_code = Some(
                self.account
                    .get_init_code()
                    .await
                    .map_err(SmartAccountMiddlewareError::AccountError)?,
            );
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

        if user_op.pre_verification_gas.is_none() {
            let init_gas = self
                .account
                .estimate_creation_gas()
                .await
                .map_err(SmartAccountMiddlewareError::AccountError)?;
            user_op.verification_gas_limit =
                Some(self.account.get_verification_gas_limit().add(init_gas));
        }

        if let Some(paymaster_api) = self.account.get_paymaster() {
            let pre_verification_gas = self.account.get_pre_verification_gas(user_op.clone());

            user_op.pre_verification_gas = Some(pre_verification_gas);

            let paymaster_and_data = paymaster_api
                .get_paymaster_and_data(user_op.clone().into())
                .await
                .map_err(SmartAccountMiddlewareError::PaymasterError)?;

            user_op.paymaster_and_data = Some(paymaster_and_data);
        } else {
            user_op.paymaster_and_data = Some(Bytes::new());
        }

        user_op.pre_verification_gas = Some(self.account.get_pre_verification_gas(user_op.clone()));

        Ok(())
    }

    async fn sign_user_operation<S: Signer>(
        &self,
        user_op: UserOperationRequest,
        // TODO: Passing in signer through method param for now. Consider separate signer middleware.
        signer: &S,
    ) -> Result<Bytes, SmartAccountMiddlewareError<M>>
    where
        A: BaseAccount<Inner = M>,
    {
        self.account
            .sign_user_op(user_op, signer)
            .await
            .map_err(SmartAccountMiddlewareError::AccountError)
    }

    // TODO: Could also call eth_estimateUserOperationGas
    async fn estimate_user_operation_gas(
        &self,
        user_op: UserOperationRequest,
    ) -> Result<U256, SmartAccountMiddlewareError<M>>
    where
        A: BaseAccount<Inner = M>,
    {
        let (Some(target), Some(data)) = (user_op.contract_target, user_op.tx_data.clone()) else {
            return Ok(U256::zero())
        };

        let (_, call_gas_limit) = self
            .account
            .encode_user_op_call_data_and_gas_limit(
                target,
                user_op.tx_value,
                data,
                user_op.call_gas_limit,
            )
            .await
            .map_err(SmartAccountMiddlewareError::AccountError)?;

        Ok(call_gas_limit)
    }

    async fn get_user_operation<T: Send + Sync + Into<UserOpHash>>(
        &self,
        user_op_hash: T,
    ) -> Result<Option<UserOperation>, SmartAccountMiddlewareError<M>> {
        let hash = user_op_hash.into();

        self.inner()
            .provider()
            .request("eth_getUserOperationByHash", [hash])
            .await
            .map_err(SmartAccountMiddlewareError::ProviderError)
    }

    async fn get_user_operation_receipt<T: Send + Sync + Into<UserOpHash>>(
        &self,
        user_op_hash: T,
    ) -> Result<Option<UserOperationReceipt>, SmartAccountMiddlewareError<M>> {
        let hash = user_op_hash.into();

        self.inner()
            .provider()
            .request("eth_getUserOperationReceipt", [hash])
            .await
            .map_err(SmartAccountMiddlewareError::ProviderError)
    }

    async fn get_supported_entry_points(
        &self,
    ) -> Result<Vec<String>, SmartAccountMiddlewareError<M>> {
        self.inner()
            .provider()
            .request("eth_supportedEntryPoints", ())
            .await
            .map_err(SmartAccountMiddlewareError::ProviderError)
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

    #[error("account error {0}")]
    PaymasterError(PaymasterError),

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
