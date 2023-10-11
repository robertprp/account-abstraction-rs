// use super::{base_account::BaseAccount, AccountError};

// use crate::{
//     paymaster::{Paymaster, PaymasterError},
//     types::{
//         request::{UserOpHash, UserOperationRequest},
//         AccountCall, FromErr, UserOperation, UserOperationGasEstimate, UserOperationReceipt,
//     },
// };

// use async_trait::async_trait;
// use ethers::{
//     providers::{Middleware, MiddlewareError, ProviderError},
//     signers::Signer,
//     types::{Bytes, U256},
//     utils,
// };
// use std::{fmt::Debug, ops::Add};
// use thiserror::Error;

// #[derive(Clone, Debug)]
// pub struct SmartAccountMiddleware<M, A> {
//     inner: M,
//     account: A,
// }

// impl<M, A> SmartAccountMiddleware<M, A>
// where
//     M: Middleware,
//     A: BaseAccount,
// {
//     pub fn new(inner: M, account: A) -> Self {
//         Self { inner, account }
//     }

//     pub fn account(&self) -> &A {
//         &self.account
//     }

//     pub async fn send_user_operation<U: Into<UserOperationRequest> + Send + Sync, S: Signer>(
//         &self,
//         user_op: U,
//         // TODO: Passing in signer through method param for now. Consider separate signer middleware.
//         signer: &S,
//     ) -> Result<UserOpHash, SmartAccountMiddlewareError<M>>
//     where
//         A: BaseAccount<Inner = M>,
//     {
//         let mut user_op: UserOperationRequest = user_op.into();
//         self.fill_user_operation(&mut user_op).await?;

//         if user_op.signature.is_none() {
//             let signature: Bytes = self.sign_user_operation(user_op.clone(), signer).await?;
//             user_op.signature = Some(signature)
//         }

//         let serialized_user_op = utils::serialize(&user_op);
//         let serialized_entry_point_address =
//             utils::serialize(&self.account.get_entry_point_address());

//         self.inner()
//             .provider()
//             .request(
//                 "eth_sendUserOperation",
//                 [serialized_user_op, serialized_entry_point_address],
//             )
//             .await
//             .map_err(SmartAccountMiddlewareError::ProviderError)
//     }

//     pub async fn fill_user_operation(
//         &self,
//         user_op: &mut UserOperationRequest,
//     ) -> Result<(), SmartAccountMiddlewareError<M>>
//     where
//         A: BaseAccount<Inner = M>,
//     {
//         if let Some(ref call) = user_op.call {
//             let call_data: Bytes = match call {
//                 AccountCall::Execute(execute_call) => {
//                     let call_data = self
//                         .account
//                         .encode_execute(execute_call.clone())
//                         .await
//                         .map_err(SmartAccountMiddlewareError::AccountError)?;

//                     call_data.into()
//                 }
//                 AccountCall::ExecuteBatch(execute_calls) => {
//                     let call_data = self
//                         .account
//                         .encode_execute_batch(execute_calls.clone())
//                         .await
//                         .map_err(SmartAccountMiddlewareError::AccountError)?;

//                     call_data.into()
//                 }
//             };

//             user_op.call_data = Some(call_data.into());
//         }

//         if user_op.nonce.is_none() {
//             let nonce = self.account.get_nonce().await.unwrap_or(U256::from(0));
//             user_op.set_nonce(nonce);
//         }

//         if user_op.sender.is_none() {
//             user_op.sender = Some(
//                 self.account
//                     .get_account_address()
//                     .await
//                     .map_err(SmartAccountMiddlewareError::AccountError)?,
//             );
//         }

//         if user_op.init_code.is_none() {
//             user_op.init_code = Some(
//                 self.account
//                     .get_init_code()
//                     .await
//                     .map_err(SmartAccountMiddlewareError::AccountError)?,
//             );
//         }

//         if user_op.max_fee_per_gas.is_none() || user_op.max_priority_fee_per_gas.is_none() {
//             let (max_fee_per_gas, max_priority_fee_per_gas) =
//                 self.estimate_eip1559_fees(None).await?;

//             if user_op.max_priority_fee_per_gas.is_none() {
//                 user_op.max_priority_fee_per_gas = Some(max_priority_fee_per_gas);
//             }

//             if user_op.max_fee_per_gas.is_none() {
//                 user_op.max_fee_per_gas = Some(max_fee_per_gas);
//             }
//         }

//         if user_op.pre_verification_gas.is_none() {
//             let init_gas = self
//                 .account
//                 .estimate_creation_gas()
//                 .await
//                 .map_err(SmartAccountMiddlewareError::AccountError)?;
//             user_op.verification_gas_limit =
//                 Some(self.account.get_verification_gas_limit().add(init_gas));
//         }

//         // TODO: Update
//         // if let Some(paymaster_api) = self.account.get_paymaster() {
//         //     let pre_verification_gas = self.account.get_pre_verification_gas(user_op.clone());

//         //     user_op.pre_verification_gas = Some(pre_verification_gas);

//         //     let paymaster_and_data = paymaster_api
//         //         .get_paymaster_and_data(user_op.clone().into())
//         //         .await
//         //         .map_err(SmartAccountMiddlewareError::PaymasterError)?;

//         //     user_op.paymaster_and_data = Some(paymaster_and_data);
//         // } else {
//         //     user_op.paymaster_and_data = Some(Bytes::new());
//         // }

//         if user_op.call_gas_limit.is_none()
//             || user_op.verification_gas_limit.is_none()
//             || user_op.pre_verification_gas.is_none()
//         {
//             let gas_estimate = self
//                 .estimate_user_operation_gas(&user_op.clone().with_defaults())
//                 .await?;
//             user_op.call_gas_limit = Some(
//                 user_op
//                     .call_gas_limit
//                     .unwrap_or(gas_estimate.call_gas_limit.into()),
//             );
//             user_op.pre_verification_gas = Some(
//                 user_op
//                     .pre_verification_gas
//                     .unwrap_or(gas_estimate.pre_verification_gas.into()),
//             );
//             user_op.verification_gas_limit = Some(
//                 user_op
//                     .verification_gas_limit
//                     .unwrap_or(gas_estimate.verification_gas.into()),
//             );
//         }

//         Ok(())
//     }

//     pub async fn sign_user_operation<S: Signer>(
//         &self,
//         user_op: UserOperationRequest,
//         // TODO: Passing in signer through method param for now. Consider separate signer middleware.
//         signer: &S,
//     ) -> Result<Bytes, SmartAccountMiddlewareError<M>>
//     where
//         A: BaseAccount<Inner = M>,
//     {
//         self.account
//             .sign_user_op(user_op, signer)
//             .await
//             .map_err(SmartAccountMiddlewareError::AccountError)
//     }

//     pub async fn estimate_user_operation_gas(
//         &self,
//         user_op: &UserOperationRequest,
//     ) -> Result<UserOperationGasEstimate, SmartAccountMiddlewareError<M>>
//     where
//         A: BaseAccount<Inner = M>,
//     {
//         let serialized_user_op = utils::serialize(user_op);
//         let serialized_entry_point_address =
//             utils::serialize(&self.account.get_entry_point_address());
//         println!("serialized_user_op {:?}", serialized_user_op);
//         self.inner()
//             .provider()
//             .request(
//                 "eth_estimateUserOperationGas",
//                 [serialized_user_op, serialized_entry_point_address],
//             )
//             .await
//             .map_err(SmartAccountMiddlewareError::ProviderError)
//     }

//     pub async fn get_user_operation<T: Send + Sync + Into<UserOpHash>>(
//         &self,
//         user_op_hash: T,
//     ) -> Result<Option<UserOperation>, SmartAccountMiddlewareError<M>> {
//         let hash = user_op_hash.into();

//         self.inner()
//             .provider()
//             .request("eth_getUserOperationByHash", [hash])
//             .await
//             .map_err(SmartAccountMiddlewareError::ProviderError)
//     }

//     pub async fn get_user_operation_receipt<T: Send + Sync + Into<UserOpHash>>(
//         &self,
//         user_op_hash: T,
//     ) -> Result<Option<UserOperationReceipt>, SmartAccountMiddlewareError<M>> {
//         let hash = user_op_hash.into();

//         self.inner()
//             .provider()
//             .request("eth_getUserOperationReceipt", [hash])
//             .await
//             .map_err(SmartAccountMiddlewareError::ProviderError)
//     }

//     pub async fn get_supported_entry_points(
//         &self,
//     ) -> Result<Vec<String>, SmartAccountMiddlewareError<M>> {
//         self.inner()
//             .provider()
//             .request("eth_supportedEntryPoints", ())
//             .await
//             .map_err(SmartAccountMiddlewareError::ProviderError)
//     }
// }

// #[async_trait]
// impl<M, A> Middleware for SmartAccountMiddleware<M, A>
// where
//     M: Middleware,
//     A: BaseAccount,
// {
//     type Error = SmartAccountMiddlewareError<M>;
//     type Provider = M::Provider;
//     type Inner = M;

//     fn inner(&self) -> &M {
//         &self.inner
//     }
// }

// #[derive(Error, Debug)]
// /// Thrown when an error happens at the smart account
// pub enum SmartAccountMiddlewareError<M: Middleware> {
//     /// Thrown when an internal middleware errors
//     #[error(transparent)]
//     MiddlewareError(M::Error),

//     #[error("account error {0}")]
//     AccountError(AccountError<M>),

//     #[error("account error {0}")]
//     PaymasterError(PaymasterError),

//     #[error("provider error {0}")]
//     ProviderError(ProviderError),

//     #[error("invalid input error {0}")]
//     InvalidInputError(String),
// }

// impl<M: Middleware> MiddlewareError for SmartAccountMiddlewareError<M> {
//     type Inner = M::Error;

//     fn from_err(src: M::Error) -> Self {
//         SmartAccountMiddlewareError::MiddlewareError(src)
//     }

//     fn as_inner(&self) -> Option<&Self::Inner> {
//         match self {
//             SmartAccountMiddlewareError::MiddlewareError(e) => Some(e),
//             _ => None,
//         }
//     }
// }

// impl<M: Middleware> FromErr<M::Error> for SmartAccountMiddlewareError<M> {
//     fn from(src: M::Error) -> SmartAccountMiddlewareError<M> {
//         SmartAccountMiddlewareError::MiddlewareError(src)
//     }
// }

use async_trait::async_trait;
use ethers::{
    providers::{JsonRpcClient, Provider},
    types::{Block, BlockId, BlockNumber, Bytes, FeeHistory, TxHash, U256},
};
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

    async fn send_user_operation<U: Into<UserOperationRequest> + Send + Sync, S: SmartAccountSigner>(
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

    async fn estimate_eip1559_fees(
        &self,
        estimator: Option<fn(U256, Vec<Vec<U256>>) -> (U256, U256)>,
    ) -> Result<(U256, U256), Self::Error> {
        self.inner()
            .estimate_eip1559_fees(estimator)
            .await
            .map_err(FromErr::from)
    }

    async fn get_block<T: Into<BlockId> + Send + Sync>(
        &self,
        block_hash_or_number: T,
    ) -> Result<Option<Block<TxHash>>, Self::Error> {
        self.inner()
            .get_block(block_hash_or_number)
            .await
            .map_err(FromErr::from)
    }

    async fn fee_history<T: Into<U256> + Send + Sync>(
        &self,
        block_count: T,
        last_block: BlockNumber,
        reward_percentiles: &[f64],
    ) -> Result<FeeHistory, Self::Error> {
        self.inner()
            .fee_history(block_count, last_block, reward_percentiles)
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
