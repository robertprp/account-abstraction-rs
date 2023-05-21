use crate::types::request::UserOperationRequest;
use ethers::types::{Address, Bytes, U256};

mod entry_point;
pub use entry_point::{EntryPoint, EntryPointCalls, SenderAddressResult};

mod simple_account_factory;
pub use simple_account_factory::*;

mod simple_account;
pub use simple_account::{NonceCall, SimpleAccount, SimpleAccountCalls, ExecuteCall};

#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct UserOperation {
    pub sender: ::ethers::core::types::Address,
    pub nonce: ::ethers::core::types::U256,
    pub init_code: ::ethers::core::types::Bytes,
    pub call_data: ::ethers::core::types::Bytes,
    pub call_gas_limit: ::ethers::core::types::U256,
    pub verification_gas_limit: ::ethers::core::types::U256,
    pub pre_verification_gas: ::ethers::core::types::U256,
    pub max_fee_per_gas: ::ethers::core::types::U256,
    pub max_priority_fee_per_gas: ::ethers::core::types::U256,
    pub paymaster_and_data: ::ethers::core::types::Bytes,
    pub signature: ::ethers::core::types::Bytes,
}

impl From<UserOperation> for entry_point::UserOperation {
    fn from(request: UserOperation) -> Self {
        Self {
            sender: request.sender,
            nonce: request.nonce,
            init_code: request.init_code,
            call_data: request.call_data,
            call_gas_limit: request.call_gas_limit,
            verification_gas_limit: request.verification_gas_limit,
            pre_verification_gas: request.pre_verification_gas,
            max_fee_per_gas: request.max_fee_per_gas,
            max_priority_fee_per_gas: request.max_priority_fee_per_gas,
            paymaster_and_data: request.paymaster_and_data,
            signature: request.signature,
        }
    }
}

impl From<UserOperation> for simple_account::UserOperation {
    fn from(request: UserOperation) -> Self {
        Self {
            sender: request.sender,
            nonce: request.nonce,
            init_code: request.init_code,
            call_data: request.call_data,
            call_gas_limit: request.call_gas_limit,
            verification_gas_limit: request.verification_gas_limit,
            pre_verification_gas: request.pre_verification_gas,
            max_fee_per_gas: request.max_fee_per_gas,
            max_priority_fee_per_gas: request.max_priority_fee_per_gas,
            paymaster_and_data: request.paymaster_and_data,
            signature: request.signature,
        }
    }
}

impl From<UserOperationRequest> for UserOperation {
    fn from(request: UserOperationRequest) -> Self {
        Self {
            sender: request.sender.unwrap_or_else(|| Address::zero()),
            nonce: request.nonce.unwrap_or_else(|| U256::zero()),
            init_code: request.init_code.unwrap_or_else(|| Bytes::new()),
            call_data: request.call_data.unwrap_or_else(|| Bytes::new()),
            call_gas_limit: request.call_gas_limit.unwrap_or_else(|| U256::zero()),
            verification_gas_limit: request
                .verification_gas_limit
                .unwrap_or_else(|| U256::from(21000)),
            pre_verification_gas: request.pre_verification_gas.unwrap_or_else(|| U256::zero()),
            max_fee_per_gas: request.max_fee_per_gas.unwrap_or_else(|| U256::zero()),
            max_priority_fee_per_gas: request
                .max_priority_fee_per_gas
                .unwrap_or_else(|| U256::zero()),
            paymaster_and_data: request.paymaster_and_data.unwrap_or_else(|| Bytes::new()),
            signature: request.signature.unwrap_or_else(|| Bytes::new()),
        }
    }
}
