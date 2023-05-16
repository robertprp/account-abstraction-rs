use async_trait::async_trait;
use ethers::{
    abi::{AbiDecode},
    providers::{JsonRpcClient, Middleware, Provider, ProviderError},
    types::{Address, Bytes, U256}, prelude::{ContractError},
};
use thiserror::Error;
use std::{fmt::Debug};
use crate::contracts::{EntryPoint, SenderAddressResult, UserOperation};

#[async_trait]
pub trait BaseAccount: Sync + Send + Debug {
    // type Error: Sync
    //     + Send
    //     + Error;
    //     + FromErr<<Self::Inner as Middleware>::Error>
    //     + FromErr<ProviderError>;

    type Provider: JsonRpcClient;
    type Inner: Middleware<Provider = Self::Provider>;

    fn inner(&self) -> &Self::Inner;

    fn get_account_address(&self) -> Address;

    fn get_rpc_url(&self) -> &str;

    fn get_entry_point(&self) -> EntryPoint<Self::Inner>;

    fn get_verification_gas_limit(&self) -> U256 {
        U256::from(100000)
    }

    fn provider(&self) -> &Provider<Self::Provider> {
        self.inner().provider()
    }

    async fn get_account_init_code(&self) -> Result<Bytes, AccountError<Self::Inner>>;

    async fn get_nonce(&self) -> Result<U256, AccountError<Self::Inner>> {
        self.inner()
            .get_transaction_count(self.get_account_address(), None)
            .await
            .map_err(|e| AccountError::MiddlewareError(e))
    }

    async fn encode_execute(&self) -> Result<Vec<u8>, AccountError<Self::Inner>>;

    async fn get_user_op_hash(&self, user_op: UserOperation) -> Result<[u8; 32], AccountError<Self::Inner>> {
        let entry_point = self.get_entry_point();

        entry_point.get_user_op_hash(user_op).call().await.map_err(|e| AccountError::ContractError(e))
    }

    async fn sign_user_op_hash(&self) -> Result<(), AccountError<Self::Inner>>;

    async fn get_counterfactual_address(&self) -> Result<Address, AccountError<Self::Inner>> {
        let init_code = self.get_account_init_code().await?;
        let entry_point = self.get_entry_point();

        let result = entry_point.get_sender_address(init_code).call().await;

        match result {
            Ok(_) => Err(AccountError::RevertError(format!(
                "Get sender address must revert."
            ))),
            Err(e) => {
                if let Some(revert_err) = e.as_revert() {
                    let Ok(sender_address) = SenderAddressResult::decode(revert_err) else {
                        return Err(AccountError::DecodeError(format!(
                            "Decode sender address result error."
                        )))
                    };

                    Ok(sender_address.sender)
                } else {
                    Err(AccountError::RevertError(format!(
                        "Get sender address must revert."
                    )))
                }
            }
        }
    }
}

pub trait FromErr<T> {
    fn from(src: T) -> Self;
}

#[derive(Debug, Error)]
pub enum AccountError<M: Middleware> {
    #[error("decode error: {0}")]
    DecodeError(String),

    #[error("revert error: {0}")]
    RevertError(String),
    
    #[error("contract error: {0}")]
    ContractError(ContractError<M>),
    
    #[error("contract error: {0}")]
    MiddlewareError(M::Error),
    
    #[error("nonce error")]
    NonceError,
    
    #[error("contract error: {0}")]
    ProviderError(ProviderError),
}
