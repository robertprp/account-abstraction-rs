use ethers::types::{Address, Bytes, H256, U256, U64};
use serde::{Deserialize, Serialize};

/// Details of a signed user operation
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct UserOperation {
    /// Sender address
    #[serde(default = "ethers::types::Address::zero")]
    pub sender: Address,

    /// Nonce
    pub nonce: U256,

    /// Init code
    #[serde(rename = "initCode", default)]
    pub init_code: Bytes,

    /// Call data
    #[serde(rename = "callData", default)]
    pub call_data: Bytes,

    /// Call gas limit
    #[serde(rename = "callGasLimit", default)]
    pub call_gas_limit: U256,

    /// Verification gas limit
    #[serde(rename = "verificationGasLimit", default)]
    pub verification_gas_limit: U256,

    /// Pre-verification gas limit
    #[serde(rename = "preVerificationGas", default)]
    pub pre_verification_gas: U256,

    /// Represents the maximum amount that a user is willing
    /// to pay for their tx (inclusive of baseFeePerGas and maxPriorityFeePerGas).
    #[serde(rename = "maxFeePerGas", default)]
    pub max_fee_per_gas: U256,

    /// Represents the maximum tx fee that will go to the miner as part of the user's
    #[serde(rename = "maxPriorityFeePerGas", default)]
    pub max_priority_fee_per_gas: U256,

    /// Paymaster data
    #[serde(rename = "paymasterAndData", default)]
    pub paymaster_and_data: Bytes,

    /// Sender address
    pub signature: String,

    #[serde(default = "ethers::types::Address::zero", rename = "entryPoint")]
    /// Entry point address
    pub entry_point: Address,

    /// Hash of the block this user operation was included within.
    #[serde(rename = "blockHash")]
    pub block_hash: H256,

    /// Number of the block this user operation was included within.
    #[serde(rename = "blockNumber")]
    pub block_number: U64,

    /// Sender address
    #[serde(default, rename = "transactionHash")]
    pub transaction_hash: H256,
}
