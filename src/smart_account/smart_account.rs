use alloy::{
    network::{Ethereum, Network},
    primitives::{Address, Bytes, ChainId, U256},
    providers::Provider,
    transports::Transport,
};
use async_trait::async_trait;
use std::fmt::Debug;
use thiserror::Error;

use crate::entry_point::{EntryPointError, EntryPointTrait};
use crate::signer::SmartAccountSigner;
use crate::types::{ExecuteCall, UserOperation};

#[async_trait]
pub trait SmartAccount<P: Provider<T, N>, T: Transport + Clone, N: Network = Ethereum>:
    Sync + Send + Debug
{
    type P: Provider<T, N>;
    type EntryPoint: EntryPointTrait;

    fn provider(&self) -> &Self::P;

    fn entry_point(&self) -> &Self::EntryPoint;

    fn chain_id(&self) -> ChainId;

    fn get_factory_address(&self) -> Address;

    async fn get_factory_data(&self) -> Bytes {
        let init_code = self.get_init_code().await.unwrap_or_default();
        if init_code.len() <= 20 {
            return Bytes::default();
        }

        Bytes::from(init_code[20..].to_vec())
    }

    async fn get_account_address(&self) -> Result<Address, AccountError>;

    async fn get_nonce(&self) -> Result<U256, AccountError> {
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

    // async fn estimate_creation_gas(&self) -> Result<U256, AccountError> {
    //     let init_code: Bytes = self.get_init_code().await?;

    //     if init_code.is_empty() {
    //         Ok(U256::zero())
    //     } else {
    //         let deployer_address = &init_code[0..20];
    //         let deployer_address = Address::from_slice(deployer_address);
    //         let deployer_call_data = &init_code[20..];

    //         let typed_tx: TypedTransaction = TransactionRequest::new()
    //             .to(deployer_address)
    //             .data(deployer_call_data.to_vec())
    //             .into();

    //         let gas_estimate: U256 = self
    //             .provider()
    //             .estimate_gas(&typed_tx, None)
    //             .await
    //             .map_err(AccountError::ProviderError)?;

    //         Ok(gas_estimate)
    //     }
    // }

    async fn get_user_op_hash<U: Into<UserOperation> + Send + Sync>(
        &self,
        user_op: U,
    ) -> Result<[u8; 32], AccountError> {
        let chain_id = U256::from(self.chain_id());
        let user_op_hash = self
            .entry_point()
            .get_user_op_hash(&user_op.into(), chain_id)
            .await
            .map_err(AccountError::EntryPointError)?;

        Ok(user_op_hash)
    }

    async fn sign_user_op_hash<S: SmartAccountSigner>(
        &self,
        user_op_hash: &[u8; 32],
        signer: &S,
    ) -> Result<Bytes, AccountError>;

    async fn sign_user_op<U: Into<UserOperation> + Send + Sync, S: SmartAccountSigner>(
        &self,
        user_op: U,
        signer: &S,
    ) -> Result<Bytes, AccountError> {
        let user_op_hash = self.get_user_op_hash(user_op).await?;
        let signature = self.sign_user_op_hash(&user_op_hash, signer).await;

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

    #[error("rpc error: {0}")]
    RpcError(String),

    #[error("nonce error")]
    NonceError,

    #[error("signer error: {0}")]
    SignerError(String),

    #[error("invalid init code error")]
    InvalidInitCodeError,
}
