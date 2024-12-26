use alloy::{
    primitives::{aliases::U192, Address, Bytes, U256},
    sol,
};
use async_trait::async_trait;
use std::fmt::Debug;
use thiserror::Error;
use alloy::{
    network::Network,
    providers::Provider,
    transports::Transport,
};
use crate::smart_account::EntryPointContract::PackedUserOperation;

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
        println!("init_code: {:?}", init_code);
        
        // TODO: instantiate contract on struct level
        let contract = EntryPointContract::new(self.address, self.provider.clone());

        let result = contract.getSenderAddress(init_code).call().await;
       
        match result {
            Ok(_) => Err(EntryPointError::RevertError(format!(
                "Get sender address must revert."
            ))),
            Err(e) => {
                println!("Error: {:?}", e);

                Ok(Address::ZERO)
                // if let Some(revert_err) = e.as_revert() {
                // let Ok(sender_address) = SenderAddressResult::decode(revert_err) else {
                //     return Err(EntryPointError::RevertError(format!(
                //         "Decode sender address result error."
                //     )));
                // };

                // Ok(sender_address.sender)
                // } else {
                //     Err(EntryPointError::RevertError(format!(
                //         "Get sender address must revert."
                //     )))
                // }
            }
        }

        // Ok(Address::from_slice(&call))
    }

    async fn get_nonce(&self, address: Address) -> Result<U256, EntryPointError> {
        
        let contract = EntryPointContract::new(self.address, self.provider.clone());
        
        let result = contract.getNonce(address, U192::ZERO).call().await;

        match result {
            Ok(result) => Ok(result.nonce),
            Err(e) => Err(EntryPointError::ContractError(format!("Error getting nonce: {:?}", e))),
        }
    }
}

// #[async_trait]
// impl<M> EntryPoint for EthersEntryPoint<M>
// // where
// //     M: Middleware + 'static,
// {
//     fn get_address(&self) -> Address {
//         self.address()
//     }

//     async fn get_sender_address(&self, init_code: Bytes) -> Result<Address, EntryPointError> {
//         let result = self.get_sender_address(init_code).call().await;

//         match result {
//             Ok(_) => Err(EntryPointError::RevertError(format!(
//                 "Get sender address must revert."
//             ))),
//             Err(e) => {
//                 if let Some(revert_err) = e.as_revert() {
//                     let Ok(sender_address) = SenderAddressResult::decode(revert_err) else {
//                         return Err(EntryPointError::RevertError(format!(
//                             "Decode sender address result error."
//                         )));
//                     };

//                     Ok(sender_address.sender)
//                 } else {
//                     Err(EntryPointError::RevertError(format!(
//                         "Get sender address must revert."
//                     )))
//                 }
//             }
//         }
//     }

//     async fn get_nonce(&self, address: Address) -> Result<U256, EntryPointError> {
//         self.get_nonce(address, U256::from(0))
//             .call()
//             .await
//             .map_err(|_| EntryPointError::ContractError(format!("Get sender address call failed.")))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{network::EthereumWallet, providers::ProviderBuilder, signers::local::PrivateKeySigner};
    use alloy_node_bindings::Anvil;
    use url::Url;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_get_sender_address() {
        let anvil = Anvil::new().try_spawn().unwrap();

        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = EthereumWallet::from(signer);
        
        let rpc_url = Url::parse("https://base-sepolia.g.alchemy.com/v2/IVqOyg3PqHzBQJMqa_yZAfyonF9ne2Gx").unwrap();//anvil.endpoint_url();
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url);

        let entry_point = EntryPointContractWrapper::new(
            Address::from_str("0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789").unwrap(),
            provider,
        );

        let init_code = Bytes::default();
        let result = entry_point.get_sender_address(init_code).await;

        println!("Result: {:?}", result);
        // assert!(result.is_ok());
        // assert_eq!(result.unwrap(), Address::ZERO);
    }
}

#[derive(Debug, Error)]
pub enum EntryPointError {
    #[error("contract error: {0}")]
    ContractError(String),

    #[error("revert error: {0}")]
    RevertError(String),
}
