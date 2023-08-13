use super::{AccountError, BaseAccount};

use crate::contracts::{zerodev_kernel_account, zerodev_kernel_factory};
use crate::contracts::{EntryPoint, UserOperation};
use crate::paymaster::{Paymaster, PaymasterError};
use crate::types::ExecuteCall;

use async_trait::async_trait;
use ethers::abi::AbiEncode;
use ethers::providers::Http;
use ethers::signers::Signer;
use ethers::{
    providers::Provider,
    types::{Address, Bytes, U256},
};
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::RwLock;

const ENTRY_POINT_ADDRESS: &str = "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789";
const FACTORY_ADDRESS: &str = "0x4E4946298614FC299B50c947289F4aD0572CB9ce";
const DEFAULT_SESSION_KEY_PLUGIN: &str = "0x6E2631aF80bF7a9cEE83F590eE496bCc2E40626D";

#[derive(Debug)]
struct ZeroDevKernelAccount {
    inner: Arc<Provider<Http>>,
    owner: Address,
    account_address: RwLock<Option<Address>>,
    is_deployed: RwLock<bool>,
    rpc_url: String,
}

impl ZeroDevKernelAccount {
    fn new(
        inner: Arc<Provider<Http>>,
        owner: Address,
        account_address: RwLock<Option<Address>>,
        is_deployed: RwLock<bool>,
        rpc_url: String,
    ) -> Self {
        Self {
            inner,
            owner,
            account_address,
            is_deployed,
            rpc_url,
        }
    }
}

#[async_trait]
impl BaseAccount for ZeroDevKernelAccount {
    type Paymaster = EmptyPaymaster;
    type Provider = Http;
    type Inner = Provider<Http>;

    fn inner(&self) -> &Self::Inner {
        &self.inner
    }

    // TODO: Add
    // async approvePlugin(plugin: Contract, validUntil: BigNumber, validAfter: BigNumber, data: string): Promise<string>

    async fn get_account_address(&self) -> Result<Address, AccountError<Self::Inner>> {
        let Some(account_address) = *self.account_address.read().await else {
            let address = self.get_counterfactual_address().await?;
            *self.account_address.write().await = Some(address);
            return Ok(address)
        };

        Ok(account_address)
    }

    fn get_rpc_url(&self) -> &str {
        self.rpc_url.as_str()
    }

    fn get_entry_point_address(&self) -> Address {
        ENTRY_POINT_ADDRESS.parse().unwrap()
    }

    fn get_entry_point(&self) -> EntryPoint<Self::Inner> {
        let address: Address = self.get_entry_point_address();
        EntryPoint::new(address, self.inner.clone())
    }

    fn get_paymaster(&self) -> Option<Self::Paymaster> {
        None
    }

    async fn get_account_init_code(&self) -> Result<Bytes, AccountError<Self::Inner>> {
        let factory_address: Address = FACTORY_ADDRESS.parse().unwrap();

        let owner: Address = self.owner;

        // TODO: Add optional index
        let index = U256::from(0);

        let call = zerodev_kernel_factory::ZeroDevKernelFactoryCalls::CreateAccount(
            zerodev_kernel_factory::CreateAccountCall { owner, index },
        );

        let mut result: Vec<u8> = Vec::new();

        result.extend_from_slice(factory_address.as_bytes());
        result.extend_from_slice(&call.encode());

        let result_bytes = Bytes::from(result);

        Ok(result_bytes)
    }

    async fn is_deployed(&self) -> bool {
        *self.is_deployed.read().await
    }

    async fn set_is_deployed(&self, is_deployed: bool) {
        *self.is_deployed.write().await = is_deployed;
    }

    async fn encode_execute(
        &self,
        call: ExecuteCall,
    ) -> Result<Vec<u8>, AccountError<Self::Inner>> {
        let call = zerodev_kernel_account::ZeroDevKernelAccountCalls::Execute(
            zerodev_kernel_account::zero_dev_kernel_account::ExecuteCall {
                to: call.target,
                value: call.value,
                data: call.data,
                operation: u8::from(1),
            },
        );

        Ok(call.encode())
    }

    async fn encode_execute_batch(
        &self,
        calls: Vec<ExecuteCall>,
    ) -> Result<Vec<u8>, AccountError<Self::Inner>> {
        let targets: Vec<Address> = calls.iter().map(|call| call.target).collect();
        let data: Vec<Bytes> = calls.iter().map(|call| call.data.clone()).collect();
        let multi_call = SimpleAccountCalls::ExecuteBatch(ExecuteBatchCall {
            dest: targets,
            func: data,
        });

        Ok(multi_call.encode())
    }

    async fn sign_user_op_hash<S: Signer>(
        &self,
        user_op_hash: [u8; 32],
        signer: &S,
    ) -> Result<Bytes, AccountError<Self::Inner>> {
        let Ok(signed_hash) = signer.sign_message(&user_op_hash).await else {
            return Err(AccountError::SignerError);
        };

        Ok(signed_hash.to_vec().into())
    }
}

#[derive(Debug)]
struct EmptyPaymaster;

#[async_trait]
impl Paymaster for EmptyPaymaster {
    async fn get_paymaster_and_data(
        &self,
        _user_op: UserOperation,
    ) -> Result<Bytes, PaymasterError> {
        Ok(Bytes::new())
    }
}
