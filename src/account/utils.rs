use crate::contracts::UserOperation;
use ethers::abi::AbiEncode;
use ethers::types::Bytes;
use ethers::{abi::Address, types::U256, utils::keccak256};

pub fn pack_user_op(user_op: UserOperation, include_signature: bool) -> Vec<u8> {
    if include_signature {
        user_op.encode()
    } else {
        let user_op_without_sig = UserOperationWithoutSignature::from(user_op);
        user_op_without_sig.encode()
    }
}

pub fn hash_user_op(user_op: UserOperation) -> [u8; 32] {
    let user_op_hash = keccak256(pack_user_op(user_op, false));
    user_op_hash
}

pub fn get_user_op_hash(
    user_op: UserOperation,
    entry_point_address: Address,
    chain_id: U256,
) -> [u8; 32] {
    let user_op_hash = hash_user_op(user_op);
    let hash_input = UserOperationHashInput {
        user_op_hash,
        entry_point_address,
        chain_id,
    };
    keccak256(hash_input.encode())
}

pub fn calc_pre_verification_gas(user_op: UserOperation, overheads: Option<GasOverheads>) -> U256 {
    let overheads = overheads.unwrap_or_else(|| GasOverheads::default());
    let dummy_signature = vec![1; 65];
    let mut user_op = user_op;

    // Set dummy values in case the UserOp is incomplete.
    user_op.pre_verification_gas = U256::from(21000);
    user_op.signature = Bytes::from(dummy_signature);

    let packed = pack_user_op(user_op, true);

    let call_data_cost = packed
        .iter()
        .map(|&x| {
            if x == 0 {
                overheads.zero_byte
            } else {
                overheads.non_zero_byte
            }
        })
        .fold(U256::zero(), |acc, x| acc + x);

    let ret = call_data_cost
        + overheads.fixed / overheads.bundle_size
        + overheads.per_user_op
        + overheads.per_user_op_word * U256::from(packed.len());

    ret
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

pub struct GasOverheads {
    /// Fixed overhead for the entire handleOp bundle.
    pub fixed: U256,
    /// Per userOp overhead, added on top of the above fixed per-bundle.
    pub per_user_op: U256,
    /// Overhead for userOp word (32 bytes) block.
    pub per_user_op_word: U256,
    /// Zero byte cost, for calldata gas cost calculations.
    pub zero_byte: U256,
    /// Non-zero byte cost, for calldata gas cost calculations.
    pub non_zero_byte: U256,
    /// Expected bundle size, to split per-bundle overhead between all ops.
    pub bundle_size: U256,
    /// Expected length of the userOp signature.
    pub sig_size: U256,
}

impl GasOverheads {
    fn default() -> Self {
        GasOverheads {
            fixed: U256::from(21000),
            per_user_op: U256::from(18300),
            per_user_op_word: U256::from(4),
            zero_byte: U256::from(4),
            non_zero_byte: U256::from(16),
            bundle_size: U256::from(1),
            sig_size: U256::from(65),
        }
    }
}
