use alloy::{
    network::{Ethereum, Network},
    primitives::{Address, Bytes, ChainId, U256},
    providers::Provider,
    transports::Transport,
};
use async_trait::async_trait;
use std::fmt::Debug;
use thiserror::Error;

use crate::types::{ExecuteCall, UserOperation};

use super::{utils, EntryPoint, EntryPointError, SmartAccountSigner};

#[async_trait]
pub trait SmartAccount<P: Provider<T, N>, T: Transport + Clone, N: Network = Ethereum>:
    Sync + Send + Debug
{
    type P: Provider<T, N>; // ProviderLayer?
    type EntryPoint: EntryPoint;

    fn provider(&self) -> &Self::P;

    fn entry_point(&self) -> &Self::EntryPoint;

    fn chain_id(&self) -> ChainId;

    async fn get_account_address(&self) -> Result<Address, AccountError>;

    async fn get_nonce(&self) -> Result<U256, AccountError> {
        // TODO: Use cache trait to cache nonce, address, etc. Can also initialize

        let account_address: Address = self.get_account_address().await?;

        self.entry_point()
            .get_nonce(account_address)
            .await
            .map_err(AccountError::EntryPointError)
    }

    async fn get_init_code(&self) -> Result<Bytes, AccountError>;

    async fn is_account_deployed(&self) -> Result<bool, AccountError>;

    async fn encode_execute(&self, call: ExecuteCall) -> Result<Vec<u8>, AccountError>;

    async fn encode_execute_batch(&self, calls: Vec<ExecuteCall>) -> Result<Vec<u8>, AccountError>;

    async fn get_counterfactual_address(&self) -> Result<Address, AccountError> {
        let init_code: Bytes = self.get_init_code().await?;

        let address = self
            .entry_point()
            .get_sender_address(init_code)
            .await
            .map_err(AccountError::EntryPointError)?;

        if address == Address::ZERO {
            return Err(AccountError::InvalidInitCodeError);
        }

        Ok(address)
    }

    async fn get_user_op_hash<U: Into<UserOperation> + Send + Sync>(
        &self,
        user_op: U,
    ) -> Result<[u8; 32], AccountError> {
        let chain_id = U256::from(self.chain_id());
        let entry_point_address: Address = self.entry_point().get_address();

        Ok(utils::get_user_op_hash(
            &user_op.into(),
            entry_point_address,
            chain_id,
        ))
    }

    async fn sign_user_op_hash<S: SmartAccountSigner>(
        &self,
        user_op_hash: [u8; 32],
        signer: &S,
    ) -> Result<Bytes, AccountError>;

    async fn sign_user_op<U: Into<UserOperation> + Send + Sync, S: SmartAccountSigner>(
        &self,
        user_op: U,
        signer: &S,
    ) -> Result<Bytes, AccountError> {
        // can also be account.getEntryPoint().getUserOperationHash(request)

        let user_op_hash = self.get_user_op_hash(user_op).await?;
        let signature = self.sign_user_op_hash(user_op_hash, signer).await;

        signature
    }
}

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("decode error: {0}")]
    DecodeError(String),

    #[error("encode error: {0}")]
    EncodeError(String),

    #[error("revert error: {0}")]
    RevertError(String),

    #[error("contract error: {0}")]
    EntryPointError(EntryPointError),

    // #[error("provider error: {0}")]
    // ProviderError(ProviderError),
    #[error("nonce error")]
    NonceError,

    #[error("signer error")]
    SignerError,

    #[error("invalid init code error")]
    InvalidInitCodeError,
}
