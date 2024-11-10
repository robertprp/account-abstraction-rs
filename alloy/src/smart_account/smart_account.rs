use std::fmt::Debug;
use alloy::{network::{Ethereum, Network}, providers::Provider, transports::Transport};
use async_trait::async_trait;

use super::EntryPoint;

#[async_trait]
pub trait SmartAccount<P: Provider<T, N>, T: Transport + Clone, N: Network = Ethereum>: Sync + Send + Debug {
    type P: Provider<T, N>; // ProviderLayer?
    type EntryPoint: EntryPoint;

    fn provider(&self) -> &Self::P;

    fn entry_point(&self) -> &Self::EntryPoint;
}