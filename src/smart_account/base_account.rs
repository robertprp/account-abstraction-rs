use crate::contracts::UserOperation;
use crate::paymaster::PaymasterError;
use crate::types::ExecuteCall;
use async_trait::async_trait;
use ethers::signers::Signer;
use ethers::{
    providers::{JsonRpcClient, Provider},
    types::{transaction::eip2718::TypedTransaction, Address, Bytes, TransactionRequest, U256},
};
use std::fmt::Debug;
use thiserror::Error;

use super::{utils, EntryPoint, EntryPointError, SmartAccountMiddleware};

#[async_trait]
pub trait BaseAccount: Sync + Send + Debug {
    // TODO: move paymaster handling to middleware
    // type Paymaster: Paymaster;
    type EntryPoint: EntryPoint;
    type Provider: JsonRpcClient;
    type Inner: SmartAccountMiddleware<Provider = Self::Provider>;

    fn inner(&self) -> &Self::Inner;

    fn provider(&self) -> &Provider<Self::Provider> {
        self.inner().provider()
    }

    fn entry_point(&self) -> &Self::EntryPoint;

    async fn get_account_address(&self) -> Result<Address, AccountError>;

    // fn get_paymaster(&self) -> Option<Self::Paymaster>;

    fn get_verification_gas_limit(&self) -> U256 {
        U256::from(110000)
    }

    fn get_pre_verification_gas<U: Into<UserOperation> + Send + Sync>(&self, user_op: U) -> U256 {
        utils::calc_pre_verification_gas(user_op.into(), None)
    }

    async fn get_nonce(&self) -> Result<U256, AccountError> {
        // TODO: Use cache trait to cache nonce, address, etc. Can also initialize

        let account_address = self.get_account_address().await?;

        self.entry_point()
            .get_nonce(account_address)
            .await
            .map_err(AccountError::EntryPointError)
    }

    async fn get_account_init_code(&self) -> Result<Bytes, AccountError>;

    async fn get_init_code(&self) -> Result<Bytes, AccountError> {
        if !self.check_is_deployed().await? {
            return self.get_account_init_code().await;
        }

        Ok(Bytes::new())
    }

    async fn is_deployed(&self) -> bool;
    async fn set_is_deployed(&self, is_deployed: bool);

    async fn check_is_deployed(&self) -> Result<bool, AccountError> {
        if self.is_deployed().await {
            return Ok(self.is_deployed().await);
        }

        let sender_address_code = self
            .inner()
            .get_code(self.get_account_address().await?, None)
            .await
            .map_err(|e| AccountError::SmartAccountMiddlewareError(e.to_string()))?;

        if sender_address_code.len() > 2 {
            self.set_is_deployed(true).await;
        }

        Ok(self.is_deployed().await)
    }

    async fn encode_execute(&self, call: ExecuteCall) -> Result<Vec<u8>, AccountError>;

    async fn encode_execute_batch(&self, calls: Vec<ExecuteCall>) -> Result<Vec<u8>, AccountError>;

    async fn estimate_creation_gas(&self) -> Result<U256, AccountError> {
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
                .map_err(|e| AccountError::SmartAccountMiddlewareError(e.to_string()))?;

            Ok(gas_estimate)
        }
    }

    async fn get_user_op_hash<U: Into<UserOperation> + Send + Sync>(
        &self,
        user_op: U,
    ) -> Result<[u8; 32], AccountError> {
        let chain_id = self
            .inner()
            .get_chainid()
            .await
            .map_err(|e| AccountError::SmartAccountMiddlewareError(e.to_string()))?;

        let entry_point_address = self.entry_point().get_address();

        Ok(utils::get_user_op_hash(
            user_op.into(),
            entry_point_address,
            chain_id,
        ))
    }

    async fn get_counterfactual_address(&self) -> Result<Address, AccountError> {
        let init_code = self.get_account_init_code().await?;

        self.entry_point()
            .get_sender_address(init_code)
            .await
            .map_err(AccountError::EntryPointError)
    }

    // TODO: `Signer` produces an ECDSA signature. Will need to add our own Signer type
    async fn sign_user_op_hash<S: Signer>(
        &self,
        user_op_hash: [u8; 32],
        signer: &S,
    ) -> Result<Bytes, AccountError>;

    // TODO: `Signer` produces an ECDSA signature. Will need to add our own Signer type
    async fn sign_user_op<U: Into<UserOperation> + Send + Sync, S: Signer>(
        &self,
        user_op: U,
        signer: &S,
    ) -> Result<Bytes, AccountError> {
        let user_op_hash = self.get_user_op_hash(user_op).await?;
        let signature = self.sign_user_op_hash(user_op_hash, signer).await;

        signature
    }

    // Helpers

    // async fn encode_user_op_call_data_and_gas_limit(
    //     &self,
    //     target: Address,
    //     value: Option<U256>,
    //     data: Bytes,
    //     gas_limit: Option<U256>,
    // ) -> Result<(Bytes, U256), AccountError> {
    //     let value = value.unwrap_or(U256::zero());
    //     let call_data = self
    //         .encode_execute(ExecuteCall::new(target, value, data))
    //         .await?;

    //     let call_gas_limit = match gas_limit {
    //         Some(limit) => limit,
    //         None => {
    //             let tx_request = TransactionRequest::new()
    //                 .from(self.get_entry_point_address())
    //                 .to(self.get_account_address().await?)
    //                 .data(call_data.clone());

    //             let typed_tx: TypedTransaction = tx_request.into();

    //             self.inner()
    //                 .estimate_gas(&typed_tx, None)
    //                 .await
    //                 .map_err(FromErr::from)?
    //         }
    //     };

    //     Ok((call_data.into(), call_gas_limit))
    // }

    // async fn create_unsigned_user_op(
    //     &self,
    //     info: TransactionDetailsForUserOp,
    // ) -> Result<UserOperation, AccountError> {
    //     let (call_data, call_gas_limit) = self
    //         .encode_user_op_call_data_and_gas_limit(
    //             info.target,
    //             info.value,
    //             info.data.into(),
    //             info.gas_limit,
    //         )
    //         .await?;
    //     let init_code = self.get_init_code().await?;
    //     let init_gas = self.estimate_creation_gas().await?;
    //     let verification_gas_limit = self.get_verification_gas_limit().add(init_gas);
    //     let (max_fee_per_gas, max_priority_fee_per_gas) = {
    //         if let (Some(max_fee_per_gas), Some(max_priority_fee_per_gas)) =
    //             (info.max_fee_per_gas, info.max_priority_fee_per_gas)
    //         {
    //             (max_fee_per_gas, max_priority_fee_per_gas)
    //         } else {
    //             let gas_estimate = self
    //                 .provider()
    //                 .estimate_eip1559_fees(None)
    //                 .await
    //                 .map_err(|e| AccountError::ProviderError(e))?;
    //             gas_estimate
    //         }
    //     };

    //     let mut partial_user_op = UserOperation {
    //         sender: self.get_account_address().await?,
    //         nonce: self.get_nonce().await?,
    //         init_code,
    //         call_data,
    //         call_gas_limit,
    //         verification_gas_limit,
    //         pre_verification_gas: U256::zero(), // This will be set later
    //         max_fee_per_gas,
    //         max_priority_fee_per_gas,
    //         paymaster_and_data: Bytes::default(),
    //         signature: Bytes::default(),
    //     };

    //     if let Some(paymaster_api) = self.get_paymaster() {
    //         let pre_verification_gas = self.get_pre_verification_gas(partial_user_op.clone());
    //         partial_user_op.pre_verification_gas = pre_verification_gas;
    //         let paymaster_and_data = paymaster_api
    //             .get_paymaster_and_data(partial_user_op.clone())
    //             .await
    //             .map_err(|e| AccountError::PaymasterError(e))?;
    //         partial_user_op.paymaster_and_data = paymaster_and_data;
    //     } else {
    //         partial_user_op.paymaster_and_data = Bytes::new();
    //     }

    //     partial_user_op.pre_verification_gas =
    //         self.get_pre_verification_gas(partial_user_op.clone());
    //     Ok(partial_user_op)
    // }

    // async fn sign_transaction_info<S: Signer>(
    //     &self,
    //     transaction: TransactionDetailsForUserOp,
    //     signer: &S,
    // ) -> Result<Bytes, AccountError> {
    //     let user_op = self.create_unsigned_user_op(transaction).await?;
    //     self.sign_user_op(user_op, signer).await
    // }
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
pub enum AccountError {
    #[error("decode error: {0}")]
    DecodeError(String),

    #[error("revert error: {0}")]
    RevertError(String),

    #[error("contract error: {0}")]
    EntryPointError(EntryPointError),

    #[error("middleware error: {0}")]
    SmartAccountMiddlewareError(String),

    #[error("nonce error")]
    NonceError,

    #[error("paymaster error: {0}")]
    PaymasterError(PaymasterError),

    #[error("signer error")]
    SignerError,
}

#[cfg(test)]
mod tests {
    // use crate::contracts::simple_account_factory::CreateAccountCall;
    // use crate::smart_account::SmartAccountProvider;

    // use super::*;
    // use crate::contracts::{EntryPoint, SimpleAccountFactoryCalls};
    // use async_trait::async_trait;
    // use ethers::abi::AbiEncode;
    // use ethers::prelude::{Http, Provider, ProviderError};
    // use ethers::types::{Address, Bytes, H256, U256};
    // use std::sync::Arc;
    // use std::{assert_eq, println};

    // #[derive(Debug)]
    // struct MockBaseAccount {
    //     inner: Arc<SmartAccountProvider<Http, MockBaseAccount>>,
    // }

    // #[async_trait]
    // impl BaseAccount for MockBaseAccount {
    //     // type Paymaster = MockPaymaster;
    //     type Provider = Http;
    //     type Inner = SmartAccountProvider<Http, MockBaseAccount>;

    //     fn inner(&self) -> &Self::Inner {
    //         &self.inner
    //     }

    //     async fn get_account_address(&self) -> Result<Address, AccountError> {
    //         Ok("0x12fd82c9b1a44979838a19dfa5153bd093b0e75e"
    //             .parse()
    //             .unwrap())
    //     }

    //     // fn get_rpc_url(&self) -> &str {
    //     //     unimplemented!()
    //     // }

    //     fn get_entry_point_address(&self) -> Address {
    //         "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789" //"0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789"
    //             .parse()
    //             .unwrap()
    //     }

    //     fn get_entry_point(&self) -> EntryPoint<Self::Inner> {
    //         let address: Address = self.get_entry_point_address();
    //         EntryPoint::new(address, self.inner.clone())
    //     }

    //     // fn get_paymaster(&self) -> Option<Self::Paymaster> {
    //     //     unimplemented!()
    //     // }

    //     async fn get_account_init_code(&self) -> Result<Bytes, AccountError> {
    //         let factory_address: Address = "0x9406Cc6185a346906296840746125a0E44976454"
    //             .parse()
    //             .unwrap();

    //         let owner: Address = "0xde3e943a1c2211cfb087dc6654af2a9728b15536"
    //             .parse()
    //             .unwrap();

    //         let index = U256::from(1);

    //         let call =
    //             SimpleAccountFactoryCalls::CreateAccount(CreateAccountCall { owner, salt: index });

    //         println!("encoded call: {:?}", call.clone().encode_hex());

    //         let mut result: Vec<u8> = Vec::new();

    //         result.extend_from_slice(factory_address.as_bytes());
    //         result.extend_from_slice(&call.encode());

    //         let result_bytes = Bytes::from(result);

    //         println!("encoded calldata: {:?}", result_bytes.clone().encode_hex());

    //         Ok(result_bytes)
    //     }

    //     async fn get_nonce(&self) -> Result<U256, AccountError> {
    //         unimplemented!() // You will need to provide an actual implementation.
    //     }

    //     async fn is_deployed(&self) -> bool {
    //         false
    //     }

    //     async fn set_is_deployed(&self, _is_deployed: bool) {}

    //     async fn encode_execute(
    //         &self,
    //         _call: ExecuteCall,
    //     ) -> Result<Vec<u8>, AccountError> {
    //         unimplemented!() // You will need to provide an actual implementation.
    //     }

    //     async fn encode_execute_batch(
    //         &self,
    //         _calls: Vec<ExecuteCall>,
    //     ) -> Result<Vec<u8>, AccountError> {
    //         unimplemented!() // You will need to provide an actual implementation.
    //     }

    //     async fn sign_user_op_hash<S: Signer>(
    //         &self,
    //         _user_op_hash: [u8; 32],
    //         _signer: &S,
    //     ) -> Result<Bytes, AccountError> {
    //         unimplemented!() // You will need to provide an actual implementation.
    //     }
    // }

    // impl MockBaseAccount {
    //     async fn get_onchain_user_op_hash(
    //         &self,
    //         user_op: UserOperation,
    //     ) -> Result<[u8; 32], AccountError<<MockBaseAccount as BaseAccount>::Inner>> {
    //         let entry_point = self.get_entry_point();

    //         entry_point
    //             .get_user_op_hash(user_op.into())
    //             .call()
    //             .await
    //             .map_err(|e| AccountError::ContractError(e))
    //     }
    // }

    // #[derive(Debug)]
    // struct MockPaymaster;

    // #[async_trait]
    // impl Paymaster for MockPaymaster {
    //     async fn get_paymaster_and_data(
    //         &self,
    //         _user_op: UserOperation,
    //     ) -> Result<Bytes, PaymasterError> {
    //         Ok(Bytes::new())
    //     }
    // }

    // const RPC_URL: &str = "https://eth-mainnet.g.alchemy.com/v2/lRcdJTfR_zjZSef3yutTGE6OIY9YFx1E";

    // #[tokio::test]
    // async fn test_get_counterfactual_address() {
    //     let provider = Provider::<Http>::try_from(RPC_URL).unwrap();

    //     let account = MockBaseAccount {
    //         inner: Arc::new(provider),
    //     };

    //     let result = account.get_counterfactual_address().await;

    //     println!("{:?}", result);
    // }

    // #[tokio::test]
    // async fn test_user_op_hash() {
    //     let owner: Address = "0xde3e943a1c2211cfb087dc6654af2a9728b15536"
    //         .parse()
    //         .unwrap();

    //     let provider = Provider::<Http>::try_from(RPC_URL).unwrap();

    //     let account = MockBaseAccount {
    //         inner: Arc::new(provider),
    //     };
    //     println!(
    //         "init code {:?}",
    //         account.get_account_init_code().await.unwrap()
    //     );
    //     let user_op = UserOperation {
    //         sender: owner,
    //         nonce: U256::from(1),
    //         init_code: account.get_account_init_code().await.unwrap(),
    //         call_data: Bytes::from(vec![]),
    //         call_gas_limit: U256::from(0),
    //         verification_gas_limit: U256::from(0),
    //         pre_verification_gas: U256::from(0),
    //         max_fee_per_gas: U256::from(0),
    //         max_priority_fee_per_gas: U256::from(0),
    //         paymaster_and_data: Bytes::from(vec![]),
    //         signature: Bytes::from(vec![]),
    //     };

    //     let onchain_hash = account
    //         .get_onchain_user_op_hash(user_op.clone())
    //         .await
    //         .unwrap();
    //     let offchain_hash = account.get_user_op_hash(user_op.clone()).await.unwrap();

    //     println!("onchain {:?}", H256::from(onchain_hash));
    //     println!("offchain {:?}", H256::from(offchain_hash));

    //     assert!(onchain_hash == offchain_hash)
    // }

    // #[tokio::test]
    // async fn test_estimate_creation_gas() {
    //     let provider = Provider::<Http>::try_from(RPC_URL).unwrap();

    //     let account = MockBaseAccount {
    //         inner: Arc::new(provider),
    //     };

    //     let creation_gas = account.estimate_creation_gas().await.unwrap();

    //     assert_eq!(creation_gas, U256::from(291723))
    // }

    // impl FromErr<ProviderError> for ProviderError {
    //     fn from(src: ProviderError) -> Self {
    //         src
    //     }
    // }
}
