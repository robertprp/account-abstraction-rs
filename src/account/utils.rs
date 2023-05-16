use ethers::{utils::keccak256, types::U256, abi::Address};
use ethers::{
    abi::{AbiEncode},
};
use crate::contracts::UserOperation;

pub fn hash_user_op(user_op: UserOperation) -> [u8; 32] {
    let user_op_without_sig = UserOperationWithoutSignature::from(user_op);
    let user_op_hash = keccak256(user_op_without_sig.encode());
    user_op_hash
}

pub fn get_user_op_hash(user_op: UserOperation, entry_point_address: Address, chain_id: U256) -> [u8; 32] {
    let user_op_hash = hash_user_op(user_op);
    let hash_input = UserOperationHashInput { user_op_hash, entry_point_address, chain_id };
    keccak256(hash_input.encode())
}

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
struct UserOperationHashInput {
    user_op_hash: [u8; 32],
    entry_point_address: Address,
    chain_id: U256,
}

#[derive(Debug, Clone, ::ethers::contract::EthAbiType, ::ethers::contract::EthAbiCodec)]
struct UserOperationWithoutSignature {
    sender: Address,
    nonce: U256,
    init_code: [u8; 32],
    call_data: [u8; 32],
    call_gas_limit: U256,
    verification_gas_limit: U256,
    pre_verification_gas: U256,
    max_fee_per_gas: U256,
    max_priority_fee_per_gas: U256,
    paymaster_and_data: [u8; 32],
}

impl From<UserOperation> for UserOperationWithoutSignature {
    fn from(op: UserOperation) -> Self {
        UserOperationWithoutSignature {
            sender: op.sender,
            nonce: op.nonce,
            init_code: keccak256(op.init_code),
            call_data: keccak256(op.call_data),
            call_gas_limit: op.call_gas_limit,
            verification_gas_limit: op.verification_gas_limit,
            pre_verification_gas: op.pre_verification_gas,
            max_fee_per_gas: op.max_fee_per_gas,
            max_priority_fee_per_gas: op.max_priority_fee_per_gas,
            paymaster_and_data: keccak256(op.paymaster_and_data),
        }
    }
}