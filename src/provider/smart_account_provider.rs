use crate::entry_point::EntryPointTrait;
use crate::signer::SmartAccountSigner;
use crate::smart_account::SmartAccount;
use crate::types::request::{UserOpHash, UserOperation, UserOperationRequest};
use crate::types::{AccountCall, UserOperationGasEstimation};
use alloy::providers::RootProvider;
use alloy::{
    network::Network,
    primitives::{Address, Bytes, U256},
    providers::Provider,
    rpc::types::UserOperationReceipt,
    transports::TransportError,
};
use async_trait::async_trait;
use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

#[async_trait]
pub trait SmartAccountProviderTrait<N>: Provider<N> + Debug
where
    N: Network,
{
    type Error: Error + Send + Sync + From<TransportError>;

    /// Sends a user operation to the bundler
    async fn send_user_operation<S>(
        &self,
        user_op: UserOperationRequest,
        signer: &S,
    ) -> Result<UserOpHash, Self::Error>
    where
        S: SmartAccountSigner + Send + Sync;

    /// Fills missing fields in a user operation
    async fn fill_user_operation(
        &self,
        user_op: &mut UserOperationRequest,
    ) -> Result<(), Self::Error>;

    /// Signs a user operation using the provided signer
    async fn sign_user_operation<S>(
        &self,
        user_op: UserOperationRequest,
        signer: &S,
    ) -> Result<Bytes, Self::Error>
    where
        S: SmartAccountSigner + Send + Sync;

    /// Estimates gas parameters for a user operation
    async fn estimate_user_operation_gas(
        &self,
        user_op: &UserOperationRequest,
    ) -> Result<UserOperationGasEstimation, Self::Error>;

    /// Retrieves a user operation by its hash
    async fn get_user_operation<H>(
        &self,
        user_op_hash: H,
    ) -> Result<Option<UserOperation>, Self::Error>
    where
        H: Send + Sync + Into<UserOpHash>;

    /// Retrieves the receipt of a user operation
    async fn get_user_operation_receipt<H>(
        &self,
        user_op_hash: H,
    ) -> Result<Option<UserOperationReceipt>, Self::Error>
    where
        H: Send + Sync + Into<UserOpHash>;

    /// Returns the list of supported entry points
    async fn get_supported_entry_points(&self) -> Result<Vec<Address>, Self::Error>;

    /// Gets paymaster data for a user operation
    async fn get_paymaster_and_data(
        &self,
        user_op: UserOperationRequest,
    ) -> Result<Bytes, Self::Error>;
}

#[derive(Clone, Debug)]
pub struct SmartAccountProvider<P, N, A> {
    inner: P,
    account: A,
    _network: std::marker::PhantomData<N>,
}

impl<P, N, A> SmartAccountProvider<P, N, A>
where
    P: Provider<N>,
    N: Network,
    A: SmartAccount<P, N>,
{
    pub fn new(inner: P, account: A) -> Self {
        Self {
            inner,
            account,
            _network: std::marker::PhantomData,
        }
    }
}

impl<P, N, A> Provider<N> for SmartAccountProvider<P, N, A>
where
    P: Provider<N>,
    N: Network,
    A: SmartAccount<P, N>,
{
    fn root(&self) -> &RootProvider<N> {
        self.inner.root()
    }
}

#[async_trait]
impl<P, N, A> SmartAccountProviderTrait<N> for SmartAccountProvider<P, N, A>
where
    P: Provider<N> + Send + Sync + std::fmt::Debug,
    N: Network + Send + Sync,
    A: SmartAccount<P, N>,
    Self: Provider<N>,
{
    type Error = SmartAccountError;

    async fn send_user_operation<S>(
        &self,
        mut user_op: UserOperationRequest,
        signer: &S,
    ) -> Result<UserOpHash, Self::Error>
    where
        S: SmartAccountSigner + Send + Sync,
    {
        let entry_point_address = self.account.entry_point().get_address();

        self.fill_user_operation(&mut user_op).await?;

        if user_op.signature.is_none() {
            let signature = self.sign_user_operation(user_op.clone(), signer).await?;
            user_op.signature = Some(signature);
        }

        let user_operation = user_op.build_user_operation().await.map_err(|e| {
            SmartAccountError::Provider(format!("Failed to build user operation: {}", e))
        })?;

        self.inner
            .client()
            .request(
                "eth_sendUserOperation",
                (user_operation, entry_point_address),
            )
            .await
            .map_err(SmartAccountError::from)
    }

    async fn fill_user_operation(
        &self,
        user_op: &mut UserOperationRequest,
    ) -> Result<(), Self::Error> {
        if user_op.call_data.is_none() {
            let Some(call_data) = user_op.call.clone() else {
                return Err(SmartAccountError::Provider(
                    "No call data provided".to_string(),
                ));
            };

            let encoded_call_data = match call_data {
                AccountCall::Execute(execute_call) => {
                    let call_data =
                        self.account
                            .encode_execute(execute_call)
                            .await
                            .map_err(|e| {
                                SmartAccountError::Provider(format!(
                                    "Failed to encode execute: {}",
                                    e
                                ))
                            })?;

                    call_data.into()
                }
                AccountCall::ExecuteBatch(execute_calls) => {
                    let call_data = self
                        .account
                        .encode_execute_batch(execute_calls)
                        .await
                        .map_err(|e| {
                            SmartAccountError::Provider(format!(
                                "Failed to encode execute batch: {}",
                                e
                            ))
                        })?;

                    call_data.into()
                }
            };

            user_op.call_data = Some(encoded_call_data);
        }

        if user_op.nonce.is_none() {
            let nonce =
                self.account.get_nonce().await.map_err(|e| {
                    SmartAccountError::Provider(format!("Failed to get nonce: {}", e))
                })?;
            user_op.nonce = Some(nonce);
        }

        if user_op.sender.is_none() {
            user_op.sender = Some(self.account.get_account_address().await.map_err(|e| {
                SmartAccountError::Provider(format!("Failed to get account address: {}", e))
            })?);
        }

        if user_op.factory.is_none()
            && !self.account.is_account_deployed().await.map_err(|e| {
                SmartAccountError::Provider(format!(
                    "Failed to check if account is deployed: {}",
                    e
                ))
            })?
        {
            user_op.factory = Some(self.account.get_factory_address());
            user_op.factory_data = Some(self.account.get_factory_data().await);
        }

        if user_op.max_fee_per_gas.is_none() || user_op.max_priority_fee_per_gas.is_none() {
            let eip1559_fees = self.inner.estimate_eip1559_fees().await.map_err(|e| {
                SmartAccountError::Provider(format!("Failed to estimate EIP1559 fees: {}", e))
            })?;

            if user_op.max_priority_fee_per_gas.is_none() {
                user_op.max_priority_fee_per_gas =
                    Some(U256::from(eip1559_fees.max_priority_fee_per_gas));
            }

            if user_op.max_fee_per_gas.is_none() {
                user_op.max_fee_per_gas = Some(U256::from(eip1559_fees.max_fee_per_gas));
            }
        }

        // if user_op.pre_verification_gas.is_none() {
        //     let init_gas: U256 = self
        //         .account
        //         .estimate_creation_gas()
        //         .await
        //         .map_err(SmartAccountProviderError::AccountError)?;
        //     user_op.verification_gas_limit =
        //         Some(self.account.get_verification_gas_limit().add(init_gas));
        // }

        // // TODO: Only add factory fields if account not deployed
        // if user_op.factory.is_none() {
        //     user_op.factory = Some(self.account.get_factory_address());
        // }

        // if user_op.factory_data.is_none() {
        //     user_op.factory_data = Some(self.account.get_factory_data().await);
        // }

        if user_op.call_gas_limit.is_none()
            || user_op.verification_gas_limit.is_none()
            || user_op.pre_verification_gas.is_none()
        {
            let gas_estimate = self
                .estimate_user_operation_gas(&user_op.clone().with_gas_estimate_defaults(
                    self.account.get_dummy_signature(),
                    Some(Address::from_str("0x69007702764179f14f51cdce752f4f775d74e139").unwrap()),
                ))
                .await?;

            if user_op.call_gas_limit.is_none() {
                user_op.call_gas_limit = Some(gas_estimate.call_gas_limit);
            }

            if user_op.verification_gas_limit.is_none() {
                user_op.verification_gas_limit = Some(gas_estimate.verification_gas_limit);
            }

            if user_op.pre_verification_gas.is_none() {
                user_op.pre_verification_gas = Some(gas_estimate.pre_verification_gas);
            }

            if user_op.paymaster_verification_gas_limit.is_none()
                && !user_op.paymaster_data.is_none()
            {
                user_op.paymaster_verification_gas_limit =
                    gas_estimate.paymaster_verification_gas_limit;
            }
        }

        if user_op.paymaster_data.is_none() {
            let paymaster_data = self.get_paymaster_and_data(user_op.clone()).await?;
            user_op.paymaster_data = Some(paymaster_data);
        }

        Ok(())
    }

    async fn sign_user_operation<S>(
        &self,
        user_op: UserOperationRequest,
        signer: &S,
    ) -> Result<Bytes, Self::Error>
    where
        S: SmartAccountSigner + Send + Sync,
    {
        self.account
            .sign_user_op(user_op, signer)
            .await
            .map_err(|e| {
                SmartAccountError::Provider(format!("Failed to sign user operation: {}", e))
            })
    }

    async fn estimate_user_operation_gas(
        &self,
        user_op: &UserOperationRequest,
    ) -> Result<UserOperationGasEstimation, Self::Error> {
        let entry_point_address = self.account.entry_point().get_address();

        self.inner
            .client()
            .request(
                "eth_estimateUserOperationGas",
                (user_op, entry_point_address),
            )
            .await
            .map_err(SmartAccountError::from)
    }

    async fn get_user_operation<H>(
        &self,
        user_op_hash: H,
    ) -> Result<Option<UserOperation>, Self::Error>
    where
        H: Send + Sync + Into<UserOpHash>,
    {
        self.inner
            .client()
            .request("eth_getUserOperation", (user_op_hash.into(),))
            .await
            .map_err(SmartAccountError::from)
    }

    async fn get_user_operation_receipt<H>(
        &self,
        user_op_hash: H,
    ) -> Result<Option<UserOperationReceipt>, Self::Error>
    where
        H: Send + Sync + Into<UserOpHash>,
    {
        self.inner
            .client()
            .request("eth_getUserOperationReceipt", (user_op_hash.into(),))
            .await
            .map_err(SmartAccountError::from)
    }

    async fn get_supported_entry_points(&self) -> Result<Vec<Address>, Self::Error> {
        self.inner
            .client()
            .request("eth_supportedEntryPoints", ())
            .await
            .map_err(SmartAccountError::from)
    }

    // TODO: Move this to separate paymaster trait
    async fn get_paymaster_and_data(
        &self,
        _user_op: UserOperationRequest,
    ) -> Result<Bytes, Self::Error> {
        Ok(Bytes::default())
    }
}

// impl<P, N, A> SmartAccountProvider<P, N, A>
// where
//                 (, user_op, entry_point_address),
#[derive(Debug, thiserror::Error)]
pub enum SmartAccountError {
    #[error(transparent)]
    Transport(#[from] TransportError),
    #[error("Signer error: {0}")]
    Signer(String),
    #[error("Provider error: {0}")]
    Provider(String),
}
