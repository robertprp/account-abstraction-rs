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
    transports::{Transport, TransportError},
};
use async_trait::async_trait;
use std::error::Error;
use std::fmt::Debug;

#[async_trait]
pub trait SmartAccountProviderTrait<N, T>: Provider<T, N> + Debug
where
    N: Network,
    T: Transport + Clone,
{
    type Error: Error + Send + Sync + From<TransportError>;

    /// Sends a user operation to the bundler
    async fn send_user_operation<S>(
        &self,
        user_op: UserOperationRequest,
        signer: &S,
        entry_point: Address,
    ) -> Result<UserOpHash, Self::Error>
    where
        S: SmartAccountSigner + Send + Sync;

    /// Fills missing fields in a user operation
    async fn fill_user_operation(
        &self,
        user_op: &mut UserOperationRequest,
        entry_point: Address,
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
        entry_point: Address,
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
pub struct SmartAccountProvider<P, T, N, A> {
    inner: P,
    account: A,
    _transport: std::marker::PhantomData<T>,
    _network: std::marker::PhantomData<N>,
}

impl<P, T, N, A> SmartAccountProvider<P, T, N, A>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
    A: SmartAccount<P, T, N>,
{
    pub fn new(inner: P, account: A) -> Self {
        Self {
            inner,
            account,
            _transport: std::marker::PhantomData,
            _network: std::marker::PhantomData,
        }
    }
}

impl<P, T, N, A> Provider<T, N> for SmartAccountProvider<P, T, N, A>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
    A: SmartAccount<P, T, N>,
{
    fn root(&self) -> &RootProvider<T, N> {
        self.inner.root()
    }
}

#[async_trait]
impl<P, T, N, A> SmartAccountProviderTrait<N, T> for SmartAccountProvider<P, T, N, A>
where
    P: Provider<T, N> + Send + Sync + std::fmt::Debug,
    T: Transport + Clone + Send + Sync + std::fmt::Debug,
    N: Network + Send + Sync,
    A: SmartAccount<P, T, N>,
    Self: Provider<T, N>,
{
    type Error = SmartAccountError;

    async fn send_user_operation<S>(
        &self,
        mut user_op: UserOperationRequest,
        signer: &S,
        // TODO: Add entry point to init
        entry_point: Address,
    ) -> Result<UserOpHash, Self::Error>
    where
        S: SmartAccountSigner + Send + Sync,
    {
        self.fill_user_operation(&mut user_op, entry_point).await?;

        if user_op.signature.is_none() {
            let signature = self.sign_user_operation(user_op.clone(), signer).await?;
            user_op.signature = Some(signature);
        }

        let user_operation = user_op.build_user_operation().await.map_err(|e| {
            SmartAccountError::Provider(format!("Failed to build user operation: {}", e))
        })?;

        self.inner
            .client()
            .request("eth_sendUserOperation", (user_operation, entry_point))
            .await
            .map_err(SmartAccountError::from)
    }

    async fn fill_user_operation(
        &self,
        user_op: &mut UserOperationRequest,
        // TODO: Add entry point to init
        entry_point: Address,
    ) -> Result<(), Self::Error> {
        if user_op.call_data.is_none() {
            let call_data: Bytes = match user_op.call.clone() {
                AccountCall::Execute(execute_call) => {
                    let call_data = self
                        .account
                        .encode_execute(execute_call.clone())
                        .await
                        .map_err(|e| {
                            SmartAccountError::Provider(format!("Failed to encode execute: {}", e))
                        })?;

                    call_data.into()
                }
                AccountCall::ExecuteBatch(execute_calls) => {
                    let call_data = self
                        .account
                        .encode_execute_batch(execute_calls.clone())
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

            user_op.call_data = Some(call_data);
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
            let eip1559_fees = self.inner.estimate_eip1559_fees(None).await.map_err(|e| {
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

        if user_op.call_gas_limit.is_none()
            || user_op.verification_gas_limit.is_none()
            || user_op.pre_verification_gas.is_none()
        {
            let gas_estimate = self
                .estimate_user_operation_gas(&user_op.clone().with_defaults(), entry_point)
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
        entry_point: Address,
    ) -> Result<UserOperationGasEstimation, Self::Error> {
        self.inner
            .client()
            .request("eth_estimateUserOperationGas", (user_op, entry_point))
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

#[derive(Debug, thiserror::Error)]
pub enum SmartAccountError {
    #[error(transparent)]
    Transport(#[from] TransportError),
    #[error("Signer error: {0}")]
    Signer(String),
    #[error("Provider error: {0}")]
    Provider(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entry_point::{EntryPointError, EntryPointTrait},
        smart_account::AccountError,
        types::ExecuteCall,
    };
    use alloy::{
        network::{Ethereum, EthereumWallet},
        primitives::{Address, Bytes, ChainId, U256},
        providers::ProviderBuilder,
        signers::local::PrivateKeySigner,
        transports::http::{Client, Http},
    };
    use alloy_node_bindings::Anvil;
    use url::Url;

    #[derive(Debug)]
    struct MockSmartAccount;

    #[async_trait]
    impl<P> SmartAccount<P, Http<Client>, Ethereum> for MockSmartAccount
    where
        P: Provider<Http<Client>, Ethereum> + Clone + Debug + Send + Sync,
    {
        type P = P;
        type EntryPoint = MockEntryPoint;

        fn provider(&self) -> &Self::P {
            unimplemented!()
        }

        fn entry_point(&self) -> &Self::EntryPoint {
            unimplemented!()
        }

        fn get_factory_address(&self) -> Address {
            Address::ZERO
        }
        fn chain_id(&self) -> ChainId {
            84532
        }

        async fn get_account_address(&self) -> Result<Address, AccountError> {
            Ok(Address::ZERO)
        }

        async fn get_init_code(&self) -> Result<Bytes, AccountError> {
            Ok(Bytes::default())
        }

        async fn is_account_deployed(&self) -> Result<bool, AccountError> {
            Ok(false)
        }

        async fn encode_execute(&self, _call: ExecuteCall) -> Result<Vec<u8>, AccountError> {
            Ok(vec![])
        }

        async fn encode_execute_batch(
            &self,
            _calls: Vec<ExecuteCall>,
        ) -> Result<Vec<u8>, AccountError> {
            Ok(vec![])
        }

        async fn sign_user_op_hash<S: SmartAccountSigner>(
            &self,
            _user_op_hash: &[u8; 32],
            _signer: &S,
        ) -> Result<Bytes, AccountError> {
            Ok(Bytes::default())
        }
    }

    #[derive(Debug)]
    struct MockEntryPoint;

    #[async_trait]
    impl EntryPointTrait for MockEntryPoint {
        fn get_address(&self) -> Address {
            Address::ZERO
        }

        async fn get_sender_address(&self, _init_code: Bytes) -> Result<Address, EntryPointError> {
            Ok(Address::ZERO)
        }

        async fn get_nonce(&self, _address: Address) -> Result<U256, EntryPointError> {
            Ok(U256::ZERO)
        }
    }

    #[tokio::test]
    async fn test_get_accounts() {
        let anvil = Anvil::new().try_spawn().unwrap();

        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = EthereumWallet::from(signer);

        let rpc_url =
            Url::parse("https://base-sepolia.g.alchemy.com/v2/IVqOyg3PqHzBQJMqa_yZAfyonF9ne2Gx")
                .unwrap();

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url);

        let account = MockSmartAccount;
        let smart_account_provider = SmartAccountProvider::new(provider, account);

        let block_number = smart_account_provider.get_block_number().await;
        assert!(block_number.is_ok());
    }

    #[tokio::test]
    async fn test_get_supported_entry_points() {
        let anvil = Anvil::new().try_spawn().unwrap();

        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = EthereumWallet::from(signer);

        let rpc_url =
            Url::parse("https://base-sepolia.g.alchemy.com/v2/IVqOyg3PqHzBQJMqa_yZAfyonF9ne2Gx")
                .unwrap();

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url);

        let account = MockSmartAccount;
        let smart_account_provider = SmartAccountProvider::new(provider, account);

        let entry_points = smart_account_provider.get_supported_entry_points().await;
        assert!(entry_points.is_ok());
    }
}
