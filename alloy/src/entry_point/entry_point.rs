use crate::entry_point::EntryPointContract::PackedUserOperation;
use alloy::contract::Error;
use alloy::transports::RpcError;
use alloy::{network::Network, providers::Provider, transports::Transport};
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
    async fn get_nonce(&self, address: Address) -> Result<U256, EntryPointError>;
}

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    EntryPointContract,
    "src/abi/EntryPoint_0_7.json"
);

#[derive(Debug)]
pub struct EntryPointContractWrapper<P: Provider<T, N>, T: Transport + Clone, N: Network> {
    address: Address,
    provider: P,
    _transport: std::marker::PhantomData<T>,
    _network: std::marker::PhantomData<N>,
}

impl<P: Provider<T, N> + Clone + Debug, T: Transport + Clone + Debug, N: Network>
    EntryPointContractWrapper<P, T, N>
{
    pub fn new(address: Address, provider: P) -> Self {
        Self {
            address,
            provider,
            _transport: std::marker::PhantomData,
            _network: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<P: Provider<T, N> + Clone + Debug, T: Transport + Clone + Debug, N: Network> EntryPointTrait
    for EntryPointContractWrapper<P, T, N>
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
                        "Invalid revert error format".into(),
                    )),
                _ => Err(EntryPointError::RevertError(
                    "Invalid revert error format".into(),
                )),
            },
        }
    }

    async fn get_nonce(&self, address: Address) -> Result<U256, EntryPointError> {
        let contract = EntryPointContract::new(self.address, self.provider.clone());

        let nonce = contract
            .getNonce(address, U192::ZERO)
            .call()
            .await
            .map_err(|e| EntryPointError::ContractError(format!("Error getting nonce: {:?}", e)))?
            .nonce;

        Ok(nonce)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        network::EthereumWallet, providers::ProviderBuilder, signers::local::PrivateKeySigner,
    };
    use alloy_node_bindings::Anvil;
    use std::str::FromStr;
    use url::Url;

    #[tokio::test]
    async fn test_get_sender_address() {
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

        let entry_point = EntryPointContractWrapper::new(
            Address::from_str("0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789").unwrap(),
            provider,
        );

        let init_code = Bytes::from_str("0x9406cc6185a346906296840746125a0e449764545fbfb9cf0000000000000000000000002c7536e3605d9c16a7a3d7b1898e529396a65c230000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let result = entry_point.get_sender_address(init_code).await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "0x982ffac966b962bddf89d3b26fee91da6f68df13"
                .parse::<Address>()
                .unwrap()
        );
    }
}

#[derive(Debug, Error)]
pub enum EntryPointError {
    #[error("contract error: {0}")]
    ContractError(String),

    #[error("revert error: {0}")]
    RevertError(String),
}
