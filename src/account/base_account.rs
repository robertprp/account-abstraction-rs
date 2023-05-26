use crate::contracts::{EntryPoint, SenderAddressResult, UserOperation};
use crate::paymaster::{Paymaster, PaymasterError};
use crate::types::{ExecuteCall, FromErr};
use async_trait::async_trait;
use ethers::signers::Signer;
use ethers::{
    abi::AbiDecode,
    prelude::ContractError,
    providers::{JsonRpcClient, Middleware, Provider, ProviderError},
    types::{transaction::eip2718::TypedTransaction, Address, Bytes, TransactionRequest, U256},
};
use std::fmt::Debug;
use std::ops::Add;
use thiserror::Error;

use super::utils;

#[async_trait]
pub trait BaseAccount: Sync + Send + Debug {
    // type Error: Sync
    //     + Send
    //     + Error;
    //     + FromErr<<Self::Inner as Middleware>::Error>
    //     + FromErr<ProviderError>;

    type Paymaster: Paymaster;
    type Provider: JsonRpcClient;
    type Inner: Middleware<Provider = Self::Provider>;

    fn inner(&self) -> &Self::Inner;

    fn provider(&self) -> &Provider<Self::Provider> {
        self.inner().provider()
    }

    async fn get_account_address(&self) -> Result<Address, AccountError<Self::Inner>>;

    fn get_rpc_url(&self) -> &str;

    fn get_entry_point_address(&self) -> Address;

    fn get_entry_point(&self) -> EntryPoint<Self::Inner>;

    fn get_paymaster(&self) -> Option<Self::Paymaster>;

    fn get_verification_gas_limit(&self) -> U256 {
        U256::from(100000)
    }

    fn get_pre_verification_gas<U: Into<UserOperation> + Send + Sync>(&self, user_op: U) -> U256 {
        utils::calc_pre_verification_gas(user_op.into(), None)
    }

    async fn get_nonce(&self) -> Result<U256, AccountError<Self::Inner>> {
        // TODO: Use cache trait to cache nonce, address, etc. Can also initialize

        let entry_point = self.get_entry_point();
        let account_address = self.get_account_address().await?;

        entry_point
            .get_nonce(account_address, U256::from(0))
            .call()
            .await
            .map_err(AccountError::ContractError)
    }

    async fn get_account_init_code(&self) -> Result<Bytes, AccountError<Self::Inner>>;

    async fn get_init_code(&self) -> Result<Bytes, AccountError<Self::Inner>> {
        if !self.check_is_deployed().await? {
            return self.get_account_init_code().await;
        }

        Ok(Bytes::new())
    }

    async fn is_deployed(&self) -> bool;
    async fn set_is_deployed(&self, is_deployed: bool);

    async fn check_is_deployed(&self) -> Result<bool, AccountError<Self::Inner>> {
        if self.is_deployed().await {
            return Ok(self.is_deployed().await);
        }

        let sender_address_code = self
            .provider()
            .get_code(self.get_account_address().await?, None)
            .await
            .map_err(AccountError::ProviderError)?;

        if sender_address_code.len() > 2 {
            self.set_is_deployed(true).await;
        }

        Ok(self.is_deployed().await)
    }

    async fn encode_execute(&self, call: ExecuteCall)
        -> Result<Vec<u8>, AccountError<Self::Inner>>;

    async fn encode_execute_batch(
        &self,
        calls: Vec<ExecuteCall>,
    ) -> Result<Vec<u8>, AccountError<Self::Inner>>;

    async fn estimate_creation_gas(&self) -> Result<U256, AccountError<Self::Inner>> {
        let init_code = self.get_init_code().await?;

        if init_code.is_empty() {
            Ok(U256::zero())
        } else {
            let deployer_address = &init_code[0..20];
            let deployer_address = ethers::types::Address::from_slice(deployer_address);
            let deployer_call_data = &init_code[20..];

            let typed_tx: TypedTransaction = TransactionRequest::new()
                .to(deployer_address)
                .data(deployer_call_data.to_vec())
                .into();

            let gas_estimate = self
                .inner()
                .estimate_gas(&typed_tx, None)
                .await
                .map_err(FromErr::from)?;

            Ok(gas_estimate)
        }
    }

    async fn get_user_op_hash<U: Into<UserOperation> + Send + Sync>(
        &self,
        user_op: U,
    ) -> Result<[u8; 32], AccountError<Self::Inner>> {
        let chain_id = self.inner().get_chainid().await.map_err(FromErr::from)?;
        let entry_point_address = self.get_entry_point_address();

        Ok(utils::get_user_op_hash(
            user_op.into(),
            entry_point_address,
            chain_id,
        ))
    }

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

    async fn sign_user_op_hash<S: Signer>(
        &self,
        user_op_hash: [u8; 32],
        signer: &S,
    ) -> Result<Bytes, AccountError<Self::Inner>>;

    async fn sign_user_op<U: Into<UserOperation> + Send + Sync, S: Signer>(
        &self,
        user_op: U,
        signer: &S,
    ) -> Result<Bytes, AccountError<Self::Inner>> {
        let user_op_hash = self.get_user_op_hash(user_op).await?;
        let signature = self.sign_user_op_hash(user_op_hash, signer).await;

        signature
    }

    // Helpers

    async fn encode_user_op_call_data_and_gas_limit(
        &self,
        target: Address,
        value: Option<U256>,
        data: Bytes,
        gas_limit: Option<U256>,
    ) -> Result<(Bytes, U256), AccountError<Self::Inner>> {
        let value = value.unwrap_or(U256::zero());
        let call_data = self
            .encode_execute(ExecuteCall::new(target, value, data.into()))
            .await?;

        let call_gas_limit = match gas_limit {
            Some(limit) => limit,
            None => {
                let tx_request = TransactionRequest::new()
                    .from(self.get_entry_point_address())
                    .to(self.get_account_address().await?)
                    .data(call_data.clone());

                let typed_tx: TypedTransaction = tx_request.into();

                self.inner()
                    .estimate_gas(&typed_tx, None)
                    .await
                    .map_err(FromErr::from)?
            }
        };

        Ok((call_data.into(), call_gas_limit))
    }

    async fn create_unsigned_user_op(
        &self,
        info: TransactionDetailsForUserOp,
    ) -> Result<UserOperation, AccountError<Self::Inner>> {
        let (call_data, call_gas_limit) = self
            .encode_user_op_call_data_and_gas_limit(
                info.target,
                info.value,
                info.data.into(),
                info.gas_limit,
            )
            .await?;
        let init_code = self.get_init_code().await?;
        let init_gas = self.estimate_creation_gas().await?;
        let verification_gas_limit = self.get_verification_gas_limit().add(init_gas);
        let (max_fee_per_gas, max_priority_fee_per_gas) = {
            if let (Some(max_fee_per_gas), Some(max_priority_fee_per_gas)) =
                (info.max_fee_per_gas, info.max_priority_fee_per_gas)
            {
                (max_fee_per_gas, max_priority_fee_per_gas)
            } else {
                let gas_estimate = self
                    .provider()
                    .estimate_eip1559_fees(None)
                    .await
                    .map_err(|e| AccountError::ProviderError(e))?;
                gas_estimate
            }
        };

        let mut partial_user_op = UserOperation {
            sender: self.get_account_address().await?,
            nonce: self.get_nonce().await?,
            init_code,
            call_data,
            call_gas_limit,
            verification_gas_limit,
            pre_verification_gas: U256::zero(), // This will be set later
            max_fee_per_gas,
            max_priority_fee_per_gas,
            paymaster_and_data: Bytes::default(),
            signature: Bytes::default(),
        };

        if let Some(paymaster_api) = self.get_paymaster() {
            let pre_verification_gas = self.get_pre_verification_gas(partial_user_op.clone());
            partial_user_op.pre_verification_gas = pre_verification_gas;
            let paymaster_and_data = paymaster_api
                .get_paymaster_and_data(partial_user_op.clone())
                .await
                .map_err(|e| AccountError::PaymasterError(e))?;
            partial_user_op.paymaster_and_data = paymaster_and_data;
        } else {
            partial_user_op.paymaster_and_data = Bytes::new();
        }

        partial_user_op.pre_verification_gas =
            self.get_pre_verification_gas(partial_user_op.clone());
        Ok(partial_user_op)
    }

    async fn sign_transaction_info<S: Signer>(
        &self,
        transaction: TransactionDetailsForUserOp,
        signer: &S,
    ) -> Result<Bytes, AccountError<Self::Inner>> {
        let user_op = self.create_unsigned_user_op(transaction).await?;
        self.sign_user_op(user_op, signer).await
    }
}

pub struct TransactionDetailsForUserOp {
    pub target: Address,
    pub data: Vec<u8>,
    pub value: Option<U256>,
    pub gas_limit: Option<U256>,
    pub max_fee_per_gas: Option<U256>,
    pub max_priority_fee_per_gas: Option<U256>,
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

    #[error("paymaster error: {0}")]
    PaymasterError(PaymasterError),

    #[error("signer error")]
    SignerError,
}

impl<M: Middleware> FromErr<M::Error> for AccountError<M> {
    fn from(src: M::Error) -> Self {
        AccountError::MiddlewareError(src)
    }
}

}
