use crate::signer::SmartAccountSigner;
use crate::types::request::{UserOpHash, UserOperation, UserOperationRequest};
use alloy::providers::RootProvider;
use alloy::{
    network::Network,
    primitives::{Address, Bytes},
    providers::Provider,
    rpc::types::{UserOperationGasEstimation, UserOperationReceipt},
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
pub struct SmartAccountProvider<P, T, N> {
    inner: P,
    _transport: std::marker::PhantomData<T>,
    _network: std::marker::PhantomData<N>,
}

impl<P, T, N> SmartAccountProvider<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    pub fn new(inner: P) -> Self {
        Self {
            inner,
            _transport: std::marker::PhantomData,
            _network: std::marker::PhantomData,
        }
    }
}

impl<P, T, N> Provider<T, N> for SmartAccountProvider<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    fn root(&self) -> &RootProvider<T, N> {
        self.inner.root()
    }
}

#[async_trait]
impl<P, T, N> SmartAccountProviderTrait<N, T> for SmartAccountProvider<P, T, N>
where
    P: Provider<T, N> + Send + Sync + std::fmt::Debug,
    T: Transport + Clone + Send + Sync + std::fmt::Debug,
    N: Network + Send + Sync,
    Self: Provider<T, N>,
{
    type Error = SmartAccountError;

    async fn send_user_operation<S>(
        &self,
        user_op: UserOperationRequest,
        signer: &S,
        entry_point: Address,
    ) -> Result<UserOpHash, Self::Error>
    where
        S: SmartAccountSigner + Send + Sync,
    {
        let signed_op = self.sign_user_operation(user_op, signer).await?;
        self.inner
            .client()
            .request("eth_sendUserOperation", (signed_op, entry_point))
            .await
            .map_err(SmartAccountError::from)
    }

    async fn fill_user_operation(
        &self,
        _user_op: &mut UserOperationRequest,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn sign_user_operation<S>(
        &self,
        _user_op: UserOperationRequest,
        _signer: &S,
    ) -> Result<Bytes, Self::Error>
    where
        S: SmartAccountSigner + Send + Sync,
    {
        Ok(Bytes::default())
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
    use alloy::{
        network::EthereumWallet, providers::ProviderBuilder, signers::local::PrivateKeySigner,
    };
    use alloy_node_bindings::Anvil;
    use url::Url;

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

        let smart_account_provider = SmartAccountProvider::new(provider);

        let block_number = smart_account_provider.get_block_number().await;

        assert!(block_number.is_ok());
        assert!(block_number.unwrap() > 0);
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

        let smart_account_provider = SmartAccountProvider::new(provider);

        let entry_points = smart_account_provider.get_supported_entry_points().await;

        assert!(entry_points.is_ok());
    }
}
