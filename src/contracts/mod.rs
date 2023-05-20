use crate::types::request::UserOperationRequest;
use ethers::types::{Address, Bytes, U256};

mod entry_point;
pub use entry_point::*;

mod simple_account_factory;
pub use simple_account_factory::*;

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
