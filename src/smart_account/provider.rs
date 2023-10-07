use async_trait::async_trait;
use ethers::{
    providers::{JsonRpcClient, Provider, ProviderError},
    signers::Signer,
    types::Bytes,
    utils,
};
use std::error::Error;
use std::fmt::Debug;

use crate::types::{
    FromErr, UserOpHash, UserOperation, UserOperationGasEstimate, UserOperationReceipt,
    UserOperationRequest,
};

use super::BaseAccount;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct SmartAccountProvider<P, A> {
    inner: P,
    account: A,
    // entry_point_address: Address,
}

impl<P, A> SmartAccountProvider<P, A>
where
    P: JsonRpcClient,
    A: BaseAccount,
{
    pub fn new(inner: P, account: A) -> Self {
        Self { inner, account }
    }

    // fn inner(&self) -> &Self::Inner {
    //     unreachable!("There is no inner provider here")
    // }

    // fn provider(&self) -> &SmartAccountProvider<Self::Provider> {
    //     self
    // }
}

#[async_trait]
impl<P: JsonRpcClient, A: BaseAccount> SmartAccountMiddlewareNew for SmartAccountProvider<P, A> {
    type Error = SmarAccountProviderError;
    type Provider = P;
    type Account = A;
    type Inner = Self;

    fn inner(&self) -> &Self::Inner {
        unreachable!("There is no inner provider here")
    }

    // async fn sign_user_operation<S: Signer>(
    //     &self,
    //     user_op: UserOperationRequest,
    //     // TODO: Passing in signer through method param for now. Consider separate signer middleware.
    //     signer: &S,
    // ) -> Result<Bytes, SmarAccountProviderError> {
    //     self.account
    //         .sign_user_op(user_op, signer)
    //         .await
    //         .map_err(|e| SmarAccountProviderError::ProviderError)
    // }

    async fn estimate_user_operation_gas(
        &self,
        user_op: &UserOperationRequest,
    ) -> Result<UserOperationGasEstimate, SmarAccountProviderError> {
        let serialized_user_op = utils::serialize(user_op);
        let serialized_entry_point_address =
            utils::serialize(&self.account.get_entry_point_address());

        self.inner()
            .provider()
            .request(
                "eth_estimateUserOperationGas",
                [serialized_user_op, serialized_entry_point_address],
            )
            .await
            .map_err(SmarAccountProviderError::ProviderError)
    }

    async fn get_user_operation<T: Send + Sync + Into<UserOpHash>>(
        &self,
        user_op_hash: T,
    ) -> Result<Option<UserOperation>, SmarAccountProviderError> {
        let hash = user_op_hash.into();

        self.inner()
            .provider()
            .request("eth_getUserOperationByHash", [hash])
            .await
            .map_err(SmarAccountProviderError::ProviderError)
    }

    async fn get_user_operation_receipt<T: Send + Sync + Into<UserOpHash>>(
        &self,
        user_op_hash: T,
    ) -> Result<Option<UserOperationReceipt>, SmarAccountProviderError> {
        let hash = user_op_hash.into();

        self.inner()
            .provider()
            .request("eth_getUserOperationReceipt", [hash])
            .await
            .map_err(SmarAccountProviderError::ProviderError)
    }

    async fn get_supported_entry_points(&self) -> Result<Vec<String>, SmarAccountProviderError> {
        self.inner()
            .provider()
            .request("eth_supportedEntryPoints", ())
            .await
            .map_err(SmarAccountProviderError::ProviderError)
    }
}

impl<P, A> AsRef<P> for SmartAccountProvider<P, A> {
    fn as_ref(&self) -> &P {
        &self.inner
    }
}

impl FromErr<SmarAccountProviderError> for SmarAccountProviderError {
    fn from(src: SmarAccountProviderError) -> Self {
        src
    }
}

#[derive(Debug, Error)]
/// An error thrown when making a call to the provider
pub enum SmarAccountProviderError {
    /// An internal error in the JSON RPC Client
    #[error(transparent)]
    JsonRpcClientError(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("provider error {0}")]
    ProviderError(ProviderError),
    // #[error("account error {0}")]
    // AccountError(AccountError<M>),
}

#[async_trait]
pub trait SmartAccountMiddlewareNew: Sync + Send + Debug {
    type Error: Sync + Send + Error + FromErr<<Self::Inner as SmartAccountMiddlewareNew>::Error>;
    type Provider: JsonRpcClient;
    type Account: BaseAccount;
    type Inner: SmartAccountMiddlewareNew<Provider = Self::Provider>;

    /// The next middleware in the stack
    fn inner(&self) -> &Self::Inner;

    /// The HTTP or Websocket provider.
    fn provider(&self) -> &Provider<Self::Provider> {
        self.inner().provider()
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

    async fn sign_user_operation<S: Signer>(
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
}

#[derive(Error, Debug)]
/// Thrown when an error happens at the smart account
pub enum SmartAccountMiddlewareError<M: SmartAccountMiddlewareNew> {
    /// Thrown when an internal middleware errors
    #[error(transparent)]
    MiddlewareError(M::Error),
    // #[error("account error {0}")]
    // AccountError(AccountError<M>),

    // #[error("account error {0}")]
    // PaymasterError(PaymasterError),

    // #[error("provider error {0}")]
    // ProviderError(ProviderError),

    // #[error("invalid input error {0}")]
    // InvalidInputError(String),
}
