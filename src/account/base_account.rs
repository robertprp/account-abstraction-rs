use async_trait::async_trait;
use ethers::{
    abi::{AbiDecode, AbiEncode},
    prelude::{ContractRevert, EthError},
    providers::{JsonRpcClient, Middleware, Provider, ProviderError},
    types::{Address, Bytes, U256},
    utils::hex::ToHex,
};
use std::{error::Error, fmt::Debug, println};

use crate::contracts::EntryPoint;

#[async_trait]
pub trait BaseAccount: Sync + Send + Debug {
    type Error: Sync
        + Send
        + Error
        + FromErr<<Self::Inner as Middleware>::Error>
        + FromErr<ProviderError>;
    type Provider: JsonRpcClient;
    type Inner: Middleware<Provider = Self::Provider>;

    fn inner(&self) -> &Self::Inner;

    fn get_account_address(&self) -> Address;

    fn get_rpc_url(&self) -> &str;

    fn get_entry_point(&self) -> EntryPoint<Self::Inner>;

    fn provider(&self) -> &Provider<Self::Provider> {
        self.inner().provider()
    }

    async fn get_account_init_code(&self) -> Result<Bytes, Self::Error>;

    async fn get_nonce(&self) -> Result<U256, Self::Error> {
        self.inner()
            .get_transaction_count(self.get_account_address(), None)
            .await
            .map_err(FromErr::from)
    }

    async fn encode_execute(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;

    async fn sign_user_op_hash(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn get_counterfactual_address(&self) -> Result<Address, Self::Error> {
        let init_code = self.get_account_init_code().await?;
        let entry_point = self.get_entry_point();

        let result = entry_point.get_sender_address(init_code).call().await;

        println!("{:?}", result);
        match result {
            Ok(_) => Err(ProviderError::CustomError(String::from(
                "Get sender address must revert",
            )))
            .map_err(FromErr::from),
            Err(e) => {
                if let Some(revert_err) = e.as_revert() {
                    let Ok(sender_address) = SenderAddressResult::decode(revert_err) else {
                        return Err(ProviderError::CustomError(String::from("Decode sender address result error"))).map_err(FromErr::from)
                    };

                    Ok(sender_address.sender)
                } else {
                    Err(ProviderError::CustomError(String::from(
                        "Get sender address must revert",
                    )))
                    .map_err(FromErr::from)
                }
            }
        }
    }
}

pub trait FromErr<T> {
    fn from(src: T) -> Self;
}