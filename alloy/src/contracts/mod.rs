use alloy::{
    primitives::{Address, Bytes, U256},
    sol,
};

sol! {
    /**
     * User Operation struct
     * @param sender                - The sender account of this request.
     * @param nonce                 - Unique value the sender uses to verify it is not a replay.
     * @param initCode              - If set, the account contract will be created by this constructor/
     * @param callData              - The method call to execute on this account.
     * @param accountGasLimits      - Packed gas limits for validateUserOp and gas limit passed to the callData method call.
     * @param preVerificationGas    - Gas not calculated by the handleOps method, but added to the gas paid.
     *                                Covers batch overhead.
     * @param gasFees               - packed gas fields maxPriorityFeePerGas and maxFeePerGas - Same as EIP-1559 gas parameters.
     * @param paymasterAndData      - If set, this field holds the paymaster address, verification gas limit, postOp gas limit and paymaster-specific extra data
     *                                The paymaster will pay for the transaction instead of the sender.
     * @param signature             - Sender-verified signature over the entire request, the EntryPoint address and the chain ID.
     */
    struct PackedUserOperation {
        address sender;
        uint256 nonce;
        bytes initCode;
        bytes callData;
        bytes32 accountGasLimits;
        uint256 preVerificationGas;
        bytes32 gasFees;
        bytes paymasterAndData;
        bytes signature;
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct UserOperation {
    pub sender: Address,
    pub nonce: U256,
    pub init_code: Bytes,
    pub call_data: Bytes,
    pub call_gas_limit: U256,
    pub verification_gas_limit: U256,
    pub pre_verification_gas: U256,
    pub max_fee_per_gas: U256,
    pub max_priority_fee_per_gas: U256,
    pub paymaster: Address,
    pub paymaster_verification_gas_limit: U256,
    pub paymaster_post_op_gas_limit: U256,
    pub paymaster_data: Bytes,
    pub signature: Bytes,
}
// pub use self::PackedUserOperation;

// #[derive(
//     Clone,
//     alloy_dyn_abi::AbiEncode,
//     // alloy::primitives::AbiEncode,
//     // alloy::contract::EthAbiCodec,
//     Default,
//     Debug,
//     PartialEq,
//     Eq,
//     Hash,
// )]
// pub struct UserOperation {
//     pub sender: ::alloy::primitives::Address,
//     pub nonce: ::alloy::primitives::U256,
//     pub init_code: ::alloy::primitives::Bytes,
//     pub call_data: ::alloy::primitives::Bytes,
//     pub call_gas_limit: ::alloy::primitives::U256,
//     pub verification_gas_limit: ::alloy::primitives::U256,
//     pub pre_verification_gas: ::alloy::primitives::U256,
//     pub max_fee_per_gas: ::alloy::primitives::U256,
//     pub max_priority_fee_per_gas: ::alloy::primitives::U256,
//     pub paymaster_and_data: ::alloy::primitives::Bytes,
//     pub signature: ::alloy::primitives::Bytes,
// }
