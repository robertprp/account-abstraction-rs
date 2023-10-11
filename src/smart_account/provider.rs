use async_trait::async_trait;
use ethers::{
    providers::{JsonRpcClient, ProviderError},
    types::{Block, BlockId, BlockNumber, Bytes, FeeHistory, TxHash, U256},
    utils,
};
use std::{fmt::Debug, ops::Add};

use crate::types::{
    AccountCall, FromErr, UserOpHash, UserOperation, UserOperationGasEstimate,
    UserOperationReceipt, UserOperationRequest,
};

use super::{AccountError, BaseAccount, EntryPoint, SmartAccountMiddleware, SmartAccountSigner};
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

    fn provider(&self) -> &P {
        &self.inner
    }
}

#[async_trait]
impl<P: JsonRpcClient, A: BaseAccount> SmartAccountMiddleware for SmartAccountProvider<P, A> {
    type Error = SmartAccountProviderError;
    type Provider = P;
    type Account = A;
    type Inner = Self;

    fn inner(&self) -> &Self::Inner {
        self
    }

    async fn send_user_operation<U: Into<UserOperationRequest> + Send + Sync, S: SmartAccountSigner>(
        &self,
        user_op: U,
        // TODO: Passing in signer through method param for now. Consider separate signer middleware.
        signer: &S,
    ) -> Result<UserOpHash, SmartAccountProviderError> {
        let mut user_op: UserOperationRequest = user_op.into();
        self.fill_user_operation(&mut user_op).await?;

        if user_op.signature.is_none() {
            let signature: Bytes = self.sign_user_operation(user_op.clone(), signer).await?;
            user_op.signature = Some(signature)
        }

        let serialized_user_op = utils::serialize(&user_op);
        let serialized_entry_point_address =
            utils::serialize(&self.account.entry_point().get_address());

        self.inner()
            .provider()
            .request(
                "eth_sendUserOperation",
                [serialized_user_op, serialized_entry_point_address],
            )
            .await
            .map_err(|e| SmartAccountProviderError::ProviderError(e.into()))
    }

    async fn fill_user_operation(
        &self,
        user_op: &mut UserOperationRequest,
    ) -> Result<(), SmartAccountProviderError> {
        if let Some(ref call) = user_op.call {
            let call_data: Bytes = match call {
                AccountCall::Execute(execute_call) => {
                    let call_data = self
                        .account
                        .encode_execute(execute_call.clone())
                        .await
                        .map_err(SmartAccountProviderError::AccountError)?;

                    call_data.into()
                }
                AccountCall::ExecuteBatch(execute_calls) => {
                    let call_data = self
                        .account
                        .encode_execute_batch(execute_calls.clone())
                        .await
                        .map_err(SmartAccountProviderError::AccountError)?;

                    call_data.into()
                }
            };

            user_op.call_data = Some(call_data.into());
        }

        if user_op.nonce.is_none() {
            let nonce = self.account.get_nonce().await.unwrap_or(U256::from(0));
            user_op.set_nonce(nonce);
        }

        if user_op.sender.is_none() {
            user_op.sender = Some(
                self.account
                    .get_account_address()
                    .await
                    .map_err(SmartAccountProviderError::AccountError)?,
            );
        }

        if user_op.init_code.is_none() {
            user_op.init_code = Some(
                self.account
                    .get_init_code()
                    .await
                    .map_err(SmartAccountProviderError::AccountError)?,
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
                .map_err(SmartAccountProviderError::AccountError)?;
            user_op.verification_gas_limit =
                Some(self.account.get_verification_gas_limit().add(init_gas));
        }

        // TODO: Update
        // if let Some(paymaster_api) = self.account.get_paymaster() {
        //     let pre_verification_gas = self.account.get_pre_verification_gas(user_op.clone());

        //     user_op.pre_verification_gas = Some(pre_verification_gas);

        //     let paymaster_and_data = paymaster_api
        //         .get_paymaster_and_data(user_op.clone().into())
        //         .await
        //         .map_err(SmartAccountMiddlewareError::PaymasterError)?;

        //     user_op.paymaster_and_data = Some(paymaster_and_data);
        // } else {
        //     user_op.paymaster_and_data = Some(Bytes::new());
        // }

        if user_op.call_gas_limit.is_none()
            || user_op.verification_gas_limit.is_none()
            || user_op.pre_verification_gas.is_none()
        {
            let gas_estimate = self
                .estimate_user_operation_gas(&user_op.clone().with_defaults())
                .await?;
            user_op.call_gas_limit = Some(
                user_op
                    .call_gas_limit
                    .unwrap_or(gas_estimate.call_gas_limit.into()),
            );
            user_op.pre_verification_gas = Some(
                user_op
                    .pre_verification_gas
                    .unwrap_or(gas_estimate.pre_verification_gas.into()),
            );
            user_op.verification_gas_limit = Some(
                user_op
                    .verification_gas_limit
                    .unwrap_or(gas_estimate.verification_gas.into()),
            );
        }

        Ok(())
    }

    async fn sign_user_operation<S: SmartAccountSigner>(
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
            .map_err(|e| SmartAccountProviderError::ProviderError(e.into()))
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
            .map_err(|e| SmartAccountProviderError::ProviderError(e.into()))
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
            .map_err(|e| SmartAccountProviderError::ProviderError(e.into()))
    }

    async fn get_supported_entry_points(&self) -> Result<Vec<String>, SmartAccountProviderError> {
        self.inner()
            .provider()
            .request("eth_supportedEntryPoints", ())
            .await
            .map_err(|e| SmartAccountProviderError::ProviderError(e.into()))
    }

    async fn estimate_eip1559_fees(
        &self,
        estimator: Option<fn(U256, Vec<Vec<U256>>) -> (U256, U256)>,
    ) -> Result<(U256, U256), SmartAccountProviderError> {
        let base_fee_per_gas = self
            .get_block(BlockNumber::Latest)
            .await?
            .ok_or_else(|| ProviderError::CustomError("Latest block not found".into()))?
            .base_fee_per_gas
            .ok_or_else(|| ProviderError::CustomError("EIP-1559 not activated".into()))?;

        let fee_history = self
            .fee_history(
                utils::EIP1559_FEE_ESTIMATION_PAST_BLOCKS,
                BlockNumber::Latest,
                &[utils::EIP1559_FEE_ESTIMATION_REWARD_PERCENTILE],
            )
            .await?;

        // use the provided fee estimator function, or fallback to the default implementation.
        let (max_fee_per_gas, max_priority_fee_per_gas) = if let Some(es) = estimator {
            es(base_fee_per_gas, fee_history.reward)
        } else {
            utils::eip1559_default_estimator(base_fee_per_gas, fee_history.reward)
        };

        Ok((max_fee_per_gas, max_priority_fee_per_gas))
    }

    async fn get_block<T: Into<BlockId> + Send + Sync>(
        &self,
        block_hash_or_number: T,
    ) -> Result<Option<Block<TxHash>>, SmartAccountProviderError> {
        let include_txs = utils::serialize(&false);
        let id: BlockId = block_hash_or_number.into();

        Ok(match id {
            BlockId::Hash(hash) => {
                let hash = utils::serialize(&hash);
                self.inner()
                    .provider()
                    .request("eth_getBlockByHash", [hash, include_txs])
                    .await
                    .map_err(|e| SmartAccountProviderError::ProviderError(e.into()))?
            }
            BlockId::Number(num) => {
                let num = utils::serialize(&num);
                self.inner()
                    .provider()
                    .request("eth_getBlockByNumber", [num, include_txs])
                    .await
                    .map_err(|e| SmartAccountProviderError::ProviderError(e.into()))?
            }
        })
    }

    async fn fee_history<T: Into<U256> + Send + Sync>(
        &self,
        block_count: T,
        last_block: BlockNumber,
        reward_percentiles: &[f64],
    ) -> Result<FeeHistory, SmartAccountProviderError> {
        let block_count = block_count.into();
        let last_block = utils::serialize(&last_block);
        let reward_percentiles = utils::serialize(&reward_percentiles);

        // The blockCount param is expected to be an unsigned integer up to geth v1.10.6.
        // Geth v1.10.7 onwards, this has been updated to a hex encoded form. Failure to
        // decode the param from client side would fallback to the old API spec.
        match self
            .inner()
            .provider()
            .request::<_, FeeHistory>(
                "eth_feeHistory",
                [
                    utils::serialize(&block_count),
                    last_block.clone(),
                    reward_percentiles.clone(),
                ],
            )
            .await
        {
            success @ Ok(_) => success,
            err @ Err(_) => {
                let fallback = self
                    .inner()
                    .provider()
                    .request::<_, FeeHistory>(
                        "eth_feeHistory",
                        [
                            utils::serialize(&block_count.as_u64()),
                            last_block,
                            reward_percentiles,
                        ],
                    )
                    .await;

                if fallback.is_err() {
                    // if the older fallback also resulted in an error, we return the error from the
                    // initial attempt
                    return err.map_err(|e| SmartAccountProviderError::ProviderError(e.into()));
                }
                fallback
            }
        }
        .map_err(|e| SmartAccountProviderError::ProviderError(e.into()))
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
