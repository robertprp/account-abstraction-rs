use crate::types::{request::UserOperation, UserOperationResponse};
use alloy::{
    network::Network,
    primitives::{Address, Bytes},
    providers::Provider,
    rpc::types::{UserOperationGasEstimation, UserOperationReceipt},
    transports::{Transport, TransportResult},
};
use async_trait::async_trait;

/// This is an adapted version of 
/// https://github.com/alloy-rs/alloy/blob/main/crates/provider/src/ext/erc4337.rs 
/// which has minor errors and is incomplete.
///
/// ERC-4337 Account Abstraction API
///
/// This module provides support for the `eth_sendUserOperation` RPC method
/// as defined in ERC-4337.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Erc4337Api<N, T>: Send + Sync {
    /// Sends a user operation to the bundler, as defined in ERC-4337.
    async fn send_user_operation(
        &self,
        user_op: UserOperation,
        entry_point: Address,
    ) -> TransportResult<UserOperationResponse>;

    /// Returns the list of supported entry points.
    async fn supported_entry_points(&self) -> TransportResult<Vec<Address>>;

    /// Returns the receipt for any user operation.
    async fn get_user_operation_receipt(
        &self,
        user_op_hash: Bytes,
    ) -> TransportResult<UserOperationReceipt>;

    /// Estimates the gas for a user operation.
    async fn estimate_user_operation_gas(
        &self,
        user_op: UserOperation,
        entry_point: Address,
    ) -> TransportResult<UserOperationGasEstimation>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<N, T, P> Erc4337Api<N, T> for P
where
    N: Network,
    T: Transport + Clone,
    P: Provider<T, N>,
{
    async fn send_user_operation(
        &self,
        user_op: UserOperation,
        entry_point: Address,
    ) -> TransportResult<UserOperationResponse> {
        self.client()
            .request("eth_sendUserOperation", (user_op, entry_point))
            .await
    }

    async fn supported_entry_points(&self) -> TransportResult<Vec<Address>> {
        self.client().request("eth_supportedEntryPoints", ()).await
    }

    async fn get_user_operation_receipt(
        &self,
        user_op_hash: Bytes,
    ) -> TransportResult<UserOperationReceipt> {
        self.client()
            .request("eth_getUserOperationReceipt", (user_op_hash,))
            .await
    }

    async fn estimate_user_operation_gas(
        &self,
        user_op: UserOperation,
        entry_point: Address,
    ) -> TransportResult<UserOperationGasEstimation> {
        self.client()
            .request("eth_estimateUserOperationGas", (user_op, entry_point))
            .await
    }
}
