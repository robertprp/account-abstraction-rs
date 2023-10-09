use async_trait::async_trait;
use ethers::{
    abi::Address,
    providers::{JsonRpcClient, ProviderError},
    signers::Signer,
    types::{transaction::eip2718::TypedTransaction, BlockId, BlockNumber, Bytes, U256},
    utils,
};
use std::fmt::Debug;

use crate::types::{
    FromErr, UserOpHash, UserOperation, UserOperationGasEstimate, UserOperationReceipt,
    UserOperationRequest,
};

use super::{BaseAccount, EntryPoint, SmartAccountMiddleware, AccountError};
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
impl<P: JsonRpcClient, A: BaseAccount> SmartAccountMiddleware for SmartAccountProvider<P, A> {
    type Error = SmartAccountProviderError;
    type Provider = P;
    type Account = A;
    type Inner = Self;

    fn inner(&self) -> &Self::Inner {
        unreachable!("There is no inner provider here")
    }

    async fn sign_user_operation<S: Signer>(
        &self,
        user_op: UserOperationRequest,
        // TODO: Passing in signer through method param for now.
        signer: &S,
    ) -> Result<Bytes, SmartAccountProviderError> {
        self.account
            .sign_user_op(user_op, signer)
            .await
            .map_err(SmartAccountProviderError::AccountError)
    }

    async fn estimate_user_operation_gas(
        &self,
        user_op: &UserOperationRequest,
    ) -> Result<UserOperationGasEstimate, SmartAccountProviderError> {
        let serialized_user_op = utils::serialize(user_op);
        let serialized_entry_point_address =
            utils::serialize(&self.account.entry_point().get_address());

        self.inner()
            .provider()
            .request(
                "eth_estimateUserOperationGas",
                [serialized_user_op, serialized_entry_point_address],
            )
            .await
            .map_err(SmartAccountProviderError::ProviderError)
    }

    async fn get_user_operation<T: Send + Sync + Into<UserOpHash>>(
        &self,
        user_op_hash: T,
    ) -> Result<Option<UserOperation>, SmartAccountProviderError> {
        let hash = user_op_hash.into();

        self.inner()
            .provider()
            .request("eth_getUserOperationByHash", [hash])
            .await
            .map_err(SmartAccountProviderError::ProviderError)
    }

    async fn get_user_operation_receipt<T: Send + Sync + Into<UserOpHash>>(
        &self,
        user_op_hash: T,
    ) -> Result<Option<UserOperationReceipt>, SmartAccountProviderError> {
        let hash = user_op_hash.into();

        self.inner()
            .provider()
            .request("eth_getUserOperationReceipt", [hash])
            .await
            .map_err(SmartAccountProviderError::ProviderError)
    }

    async fn get_supported_entry_points(&self) -> Result<Vec<String>, SmartAccountProviderError> {
        self.inner()
            .provider()
            .request("eth_supportedEntryPoints", ())
            .await
            .map_err(SmartAccountProviderError::ProviderError)
    }

    async fn get_chainid(&self) -> Result<U256, SmartAccountProviderError> {
        self.inner()
            .provider()
            .request("eth_chainId", ())
            .await
            .map_err(SmartAccountProviderError::ProviderError)
    }

    async fn estimate_gas(
        &self,
        tx: &TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<U256, SmartAccountProviderError> {
        let tx = utils::serialize(tx);
        // Some nodes (e.g. old Optimism clients) don't support a block ID being passed as a param,
        // so refrain from defaulting to BlockNumber::Latest.
        let params = if let Some(block_id) = block {
            vec![tx, utils::serialize(&block_id)]
        } else {
            vec![tx]
        };
        self.inner()
            .provider()
            .request("eth_estimateGas", params)
            .await
            .map_err(SmartAccountProviderError::ProviderError)
    }

    async fn get_code<T: Into<Address> + Send + Sync>(
        &self,
        at: T,
        block: Option<BlockId>,
    ) -> Result<Bytes,  SmartAccountProviderError> {
        let at = at.into();
        let at = utils::serialize(&at);
        let block = utils::serialize(&block.unwrap_or_else(|| BlockNumber::Latest.into()));
        self.inner()
            .provider()
            .request("eth_getCode", [at, block])
            .await
            .map_err(SmartAccountProviderError::ProviderError)
    }

    // TODO: Add paymaster methods
}

impl<P, A> AsRef<P> for SmartAccountProvider<P, A> {
    fn as_ref(&self) -> &P {
        &self.inner
    }
}

impl FromErr<SmartAccountProviderError> for SmartAccountProviderError {
    fn from(src: SmartAccountProviderError) -> Self {
        src
    }
}

#[derive(Debug, Error)]
/// An error thrown when making a call to the provider
pub enum SmartAccountProviderError {
    /// An internal error in the JSON RPC Client
    #[error(transparent)]
    JsonRpcClientError(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error(transparent)]
    ProviderError(#[from] ProviderError),

    #[error(transparent)]
    AccountError(#[from] AccountError),
}
