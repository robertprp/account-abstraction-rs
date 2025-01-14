use alloy::primitives::{Address, Bytes, B256, U256};
use serde::{Deserialize, Serialize};

/// Details of a signed user operation
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct UserOperationResponse {
    /// The account initiating the UserOperation.
    #[serde(default)]
    pub sender: Address,

    /// Nonce
    #[serde(default)]
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

    /// User op signature
    #[serde(default)]
    pub signature: Bytes,

    #[serde(default, rename = "entryPoint")]
    /// Entry point address
    pub entry_point: Address,

    /// Block hash. None when pending.
    #[serde(default, rename = "blockHash")]
    pub block_hash: Option<B256>,

    /// Number of the block this user operation was included within.
    #[serde(default, rename = "blockNumber")]
    pub block_number: Option<u64>, // stackup return as integer whereas alchemy defines it has hex / U64.

    /// Transaction hash
    #[serde(default, rename = "transactionHash")]
    pub transaction_hash: Option<B256>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct UserOperationLog {
    address: Address,
    topics: Vec<String>,
    data: String,
    #[serde(rename = "blockNumber", default)]
    pub block_number: U256,
    #[serde(rename = "transactionHash", default)]
    pub transaction_hash: B256,
    #[serde(rename = "transactionIndex", default)]
    pub transaction_index: String,
    #[serde(rename = "blockHash", default)]
    pub block_hash: B256,
    #[serde(rename = "logIndex", default)]
    pub log_index: String,
    removed: bool,
}

/// Represents the gas estimation for a user operation.
/// Note: the alloy type is not used here because it contains an invalid field.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct UserOperationGasEstimation {
    /// The gas limit for the pre-verification.
    #[serde(rename = "preVerificationGas")]
    pub pre_verification_gas: U256,
    /// The gas limit for the verification.
    #[serde(rename = "verificationGasLimit")]
    pub verification_gas_limit: U256,
    /// The gas limit for the paymaster verification.
    #[serde(rename = "paymasterVerificationGasLimit")]
    pub paymaster_verification_gas_limit: Option<U256>,
    /// The gas limit for the call.
    #[serde(rename = "callGasLimit")]
    pub call_gas_limit: U256,
}
