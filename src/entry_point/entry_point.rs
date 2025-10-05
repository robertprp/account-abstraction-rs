use crate::entry_point::EntryPointContract::PackedUserOperation;
use crate::types::UserOperation;
use crate::utils;

use alloy::contract::Error;
use alloy::transports::RpcError;
use alloy::{network::Network, providers::Provider};
use alloy::{
    primitives::{aliases::U192, Address, Bytes, U256},
    sol,
};
use async_trait::async_trait;
use std::fmt::Debug;
use thiserror::Error;

#[async_trait]
pub trait EntryPointTrait: Sync + Send + Debug {
    fn get_address(&self) -> Address;
    async fn get_sender_address(&self, init_code: Bytes) -> Result<Address, EntryPointError>;
    async fn get_nonce(&self, address: Address, key: U192) -> Result<U256, EntryPointError>;
    async fn get_user_op_hash(
        &self,
        user_op: &UserOperation,
        chain_id: U256,
    ) -> Result<[u8; 32], EntryPointError>;
}

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    EntryPointContract,
    "src/abi/EntryPoint_0_7.json"
);

#[derive(Clone, Debug)]
pub struct EntryPointContractWrapper<P: Provider<N>, N: Network> {
    address: Address,
    provider: P,
    _network: std::marker::PhantomData<N>,
}

impl<P: Provider<N> + Clone + Debug, N: Network> EntryPointContractWrapper<P, N> {
    pub fn new(address: Address, provider: P) -> Self {
        Self {
            address,
            provider,
            _network: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<P: Provider<N> + Clone + Debug, N: Network> EntryPointTrait
    for EntryPointContractWrapper<P, N>
{
    fn get_address(&self) -> Address {
        self.address
    }

    async fn get_sender_address(&self, init_code: Bytes) -> Result<Address, EntryPointError> {
        let contract = EntryPointContract::new(self.address, self.provider.clone());

        let result = contract.getSenderAddress(init_code).call().await;

        match result {
            Ok(_) => Err(EntryPointError::RevertError(
                "Get sender address must revert.".to_string(),
            )),
            Err(e) => match e {
                Error::TransportError(RpcError::ErrorResp(error_payload)) => error_payload
                    .as_revert_data()
                    .filter(|data| data.len() >= 36)
                    .map(|data| Address::from_slice(&data[16..36]))
                    .ok_or(EntryPointError::RevertError(
                        error_payload.message.clone().to_string(),
                    )),
                _ => Err(EntryPointError::RevertError(
                    "Invalid revert error format".into(),
                )),
            },
        }
    }

    async fn get_nonce(&self, address: Address, key: U192) -> Result<U256, EntryPointError> {
        let contract = EntryPointContract::new(self.address, self.provider.clone());

        let nonce = contract
            .getNonce(address, key)
            .call()
            .await
            .map_err(|e| EntryPointError::ContractError(format!("Error getting nonce: {:?}", e)))?;

        Ok(nonce)
    }

    async fn get_user_op_hash(
        &self,
        user_op: &UserOperation,
        chain_id: U256,
    ) -> Result<[u8; 32], EntryPointError> {
        Ok(utils::get_user_op_hash(&user_op, self.address, chain_id))
    }
}

#[derive(Debug, Error)]
pub enum EntryPointError {
    #[error("contract error: {0}")]
    ContractError(String),

    #[error("revert error: {0}")]
    RevertError(String),
}
