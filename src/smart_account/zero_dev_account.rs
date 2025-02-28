use alloy::{
    network::Ethereum,
    primitives::{Address, Bytes, ChainId, U256},
    providers::Provider,
    sol,
    sol_types::SolInterface,
    transports::http::{Client, Http},
};
use async_trait::async_trait;
use ZeroDevKernelAccountContract::{executeCall, ZeroDevKernelAccountContractCalls};
use ZeroDevKernelFactoryContract::{createAccountCall, ZeroDevKernelFactoryContractCalls};
use std::sync::{Arc, RwLock};

use crate::{
    entry_point::EntryPointContractWrapper,
    signer::SmartAccountSigner,
    types::ExecuteCall,
};

use super::{AccountError, SmartAccount};

/// Constants for ZeroDev
const ENTRY_POINT_ADDRESS: &str = "0x0000000071727De22E5E9d8BAf0edAc6f37da032";
const FACTORY_ADDRESS: &str = "0x4E4946298614FC299B50c947289F4aD0572CB9ce";
const _DEFAULT_SESSION_KEY_PLUGIN: &str = "0x6E2631aF80bF7a9cEE83F590eE496bCc2E40626D";
const _MULTISEND_ADDR: &str = "0x8ae01fcf7c655655ff2c6ef907b8b4718ab4e17c";

//
// Generate Alloy interfaces for the ZeroDev contracts
//
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ZeroDevKernelFactoryContract,
    "src/abi/zero_dev/ZeroDevKernelFactory.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ZeroDevKernelAccountContract,
    "src/abi/zero_dev/ZeroDevKernelAccount.json"
);

/// An Alloy implementation of a ZeroDev kernel account.
#[derive(Debug)]
pub struct ZeroDevKernelAccount<P: Provider<Http<Client>, Ethereum>> {
    provider: Arc<P>,
    owner: Address,
    account_address: RwLock<Option<Address>>,
    is_deployed: RwLock<bool>,
    entry_point: Arc<EntryPointContractWrapper<P, Http<Client>, Ethereum>>,
    chain_id: ChainId,
}

impl<P> ZeroDevKernelAccount<P>
where
    P: Provider<Http<Client>, Ethereum> + Clone + std::fmt::Debug,
{
    pub fn new(
        provider: Arc<P>,
        owner: Address,
        account_address: Option<Address>,
        is_deployed: bool,
        chain_id: ChainId,
    ) -> Self {
        let entry_point = Arc::new(EntryPointContractWrapper::new(
            ENTRY_POINT_ADDRESS.parse().unwrap(),
            (*provider).clone(),
        ));

        Self {
            provider,
            owner,
            account_address: RwLock::new(account_address),
            is_deployed: RwLock::new(is_deployed),
            entry_point,
            chain_id,
        }
    }

    /// Allows updating the deployment status.
    pub fn set_is_deployed(&self, deployed: bool) {
        *self.is_deployed.write().unwrap() = deployed;
    }
}

#[async_trait]
impl<P> SmartAccount<P, Http<Client>, Ethereum> for ZeroDevKernelAccount<P>
where
    P: Provider<Http<Client>, Ethereum> + Clone + std::fmt::Debug + Send + Sync,
{
    type P = P;
    type EntryPoint = EntryPointContractWrapper<P, Http<Client>, Ethereum>;

    fn provider(&self) -> &Self::P {
        &self.provider
    }

    fn entry_point(&self) -> &Self::EntryPoint {
        &self.entry_point
    }

    fn chain_id(&self) -> ChainId {
        self.chain_id
    }

    /// Returns the factory address (from which init code will be derived).
    fn get_factory_address(&self) -> Address {
        FACTORY_ADDRESS.parse().unwrap()
    }

    /// Returns the account address; if not known, computes it via counterfactual methods.
    async fn get_account_address(&self) -> Result<Address, AccountError> {
        if let Some(addr) = *self.account_address.read().unwrap() {
            return Ok(addr);
        }
        let addr = self.get_counterfactual_address().await?;
        *self.account_address.write().unwrap() = Some(addr);
        Ok(addr)
    }

    /// Constructs the initialization code needed to deploy the account.
    async fn get_init_code(&self) -> Result<Bytes, AccountError> {
        let index = U256::ZERO;
        let create_account_call = ZeroDevKernelFactoryContractCalls::createAccount(
            createAccountCall {
                _owner: self.owner,
                _index: index,
            },
        );
        let factory_address: Address = FACTORY_ADDRESS.parse().unwrap();
        let mut init_code = Vec::new();
        init_code.extend_from_slice(factory_address.as_slice());
        init_code.extend_from_slice(&create_account_call.abi_encode());
        Ok(Bytes::from(init_code))
    }

    /// Checks if the account is deployed by using the stored flag.
    async fn is_account_deployed(&self) -> Result<bool, AccountError> {
        Ok(*self.is_deployed.read().unwrap())
    }

    /// Encodes an execute call against the ZeroDev account contract.
    async fn encode_execute(&self, call: ExecuteCall) -> Result<Vec<u8>, AccountError> {
        let exec_call = ZeroDevKernelAccountContractCalls::execute(
            executeCall {
                to: call.target,
                value: call.value,
                data: call.data,
                operation: 1.into(),
            },
        );
        Ok(exec_call.abi_encode().into())
    }

    /// Encodes a batch execution call.
    async fn encode_execute_batch(
        &self,
        _calls: Vec<ExecuteCall>,
    ) -> Result<Vec<u8>, AccountError> {
        unimplemented!()
    }

    /// Signs the given user operation hash.
    async fn sign_user_op_hash<S: SmartAccountSigner>(
        &self,
        user_op_hash: &[u8; 32],
        signer: &S,
    ) -> Result<Bytes, AccountError> {
        signer
            .sign_message(user_op_hash)
            .await
            .map_err(|e| AccountError::SignerError(format!("Failed to sign user op hash: {}", e)))
    }
}
