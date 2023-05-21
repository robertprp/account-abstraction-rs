use super::{AccountError, BaseAccount};

use crate::contracts::{
    CreateAccountCall, SimpleAccount as SimpleAccountContract, SimpleAccountFactoryCalls,
};
use crate::contracts::{EntryPoint, ExecuteCall, SimpleAccountCalls, UserOperation};
use crate::paymaster::{Paymaster, PaymasterError};

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

#[derive(Debug)]
struct SimpleAccount<S: Signer> {
    inner: Arc<Provider<Http>>,
    signer: S,
    account_address: RwLock<Option<Address>>,
    is_deployed: RwLock<bool>,
}

#[async_trait]
impl<S> BaseAccount for SimpleAccount<S>
where
    S: Signer,
{
    type Paymaster = EmptyPaymaster;
    type Provider = Http;
    type Inner = Provider<Http>;

    fn inner(&self) -> &Self::Inner {
        &self.inner
    }

    async fn get_account_address(&self) -> Result<Address, AccountError<Self::Inner>> {
        let Some(account_address) = *self.account_address.read().await else {
            return self.get_counterfactual_address().await
        };

        Ok(account_address)
    }

    fn get_rpc_url(&self) -> &str {
        unimplemented!()
    }

    fn get_entry_point_address(&self) -> Address {
        "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789"
            .parse()
            .unwrap()
    }

    fn get_entry_point(&self) -> EntryPoint<Self::Inner> {
        let address: Address = self.get_entry_point_address();
        EntryPoint::new(address, self.inner.clone())
    }

    fn get_paymaster(&self) -> Option<Self::Paymaster> {
        None
    }

    async fn get_account_init_code(&self) -> Result<Bytes, AccountError<Self::Inner>> {
        let factory_address: Address = "0x9406Cc6185a346906296840746125a0E44976454"
            .parse()
            .unwrap();

        let owner: Address = "".parse().unwrap();

        let index = U256::from(1);

        let call =
            SimpleAccountFactoryCalls::CreateAccount(CreateAccountCall { owner, salt: index });

        let mut result: Vec<u8> = Vec::new();

        result.extend_from_slice(factory_address.as_bytes());
        result.extend_from_slice(&call.encode());

        let result_bytes = Bytes::from(result);

        Ok(result_bytes)
    }

    async fn get_nonce(&self) -> Result<U256, AccountError<Self::Inner>> {
        let account_address = self.get_account_address().await?;
        let simple_account = SimpleAccountContract::new(account_address, self.inner.clone());
        let nonce = simple_account
            .nonce()
            .call()
            .await
            .map_err(AccountError::ContractError)?;

        Ok(nonce)
    }

    async fn is_deployed(&self) -> bool {
        *self.is_deployed.read().await
    }

    async fn set_is_deployed(&self, is_deployed: bool) {
        *self.is_deployed.write().await = is_deployed;
    }

    async fn encode_execute(
        &self,
        target: Address,
        value: U256,
        data: Bytes,
    ) -> Result<Vec<u8>, AccountError<Self::Inner>> {
        let call =
            SimpleAccountCalls::Execute(ExecuteCall { dest: target, value, func: data });

        Ok(call.encode())
    }

    async fn sign_user_op_hash(
        &self,
        user_op_hash: [u8; 32],
    ) -> Result<Bytes, AccountError<Self::Inner>> {
        let Ok(signed_hash) = self.signer.sign_message(&user_op_hash).await else {
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
        user_op: UserOperation,
    ) -> Result<Bytes, PaymasterError> {
        Ok(Bytes::new())
    }
}
