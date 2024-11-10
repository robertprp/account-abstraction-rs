use alloy::{primitives::{keccak256, Address, Bytes, U256}, sol, sol_types::SolValue};

use crate::types::{PackedUserOperation, UserOperation};

pub fn get_user_op_hash(
    user_op: &UserOperation,
    entry_point: Address,
    chain_id: U256,
) -> [u8; 32] {
    // First get the hash of the encoded user operation
    let user_op_encoded = encode_user_op(user_op, true);
    let user_op_hash = keccak256(user_op_encoded);

    // Create and encode the data
    let data = (
        user_op_hash,
        entry_point,
        chain_id,
    );

    // Return the keccak256 hash of the encoded data
    keccak256(data.abi_encode()).into()
}

pub fn encode_user_op(user_op: &UserOperation, for_signature: bool) -> Bytes {
    let packed_user_op = pack_user_op(user_op);
    
    if for_signature {
        let encoded = UserOpForSignature {
            sender: packed_user_op.sender,
            nonce: packed_user_op.nonce,
            initCodeHash: keccak256(packed_user_op.initCode),
            callDataHash: keccak256(packed_user_op.callData),
            accountGasLimits: packed_user_op.accountGasLimits,
            preVerificationGas: packed_user_op.preVerificationGas,
            gasFees: packed_user_op.gasFees,
            paymasterAndDataHash: keccak256(packed_user_op.paymasterAndData),
        }.abi_encode();

        encoded.into()
    } else {
        let encoded = UserOpForGas {
            sender: packed_user_op.sender,
            nonce: packed_user_op.nonce,
            initCode: packed_user_op.initCode,
            callData: packed_user_op.callData,
            accountGasLimits: packed_user_op.accountGasLimits,
            preVerificationGas: packed_user_op.preVerificationGas,
            gasFees: packed_user_op.gasFees,
            paymasterAndData: packed_user_op.paymasterAndData,
            signature: packed_user_op.signature,
        }.abi_encode();

        encoded.into()
    }
}

// pub fn encode_user_op(user_op: &UserOperation, for_signature: bool) -> Bytes {
//     let packed_user_op = pack_user_op(user_op);
    
//     let encoded_packed_user_op: Bytes = packed_user_op.abi_encode().into();

//     encoded_packed_user_op
// }

pub fn pack_user_op(user_op: &UserOperation) -> PackedUserOperation {
    // Pack gas limits
    let account_gas_limits = pack_account_gas_limits(
        user_op.verification_gas_limit,
        user_op.call_gas_limit,
    );
    
    // Pack gas fees
    let gas_fees = pack_account_gas_limits(
        user_op.max_priority_fee_per_gas,
        user_op.max_fee_per_gas,
    );
    
    // Handle paymaster data
    let paymaster_and_data = if !user_op.paymaster.is_zero() {
        pack_paymaster_data(
            user_op.paymaster,
            user_op.paymaster_verification_gas_limit,
            user_op.paymaster_post_op_gas_limit,
            user_op.paymaster_data.clone(),
        )
    } else {
        Bytes::default() // Empty bytes, equivalent to '0x'
    };

    PackedUserOperation {
        sender: user_op.sender,
        nonce: user_op.nonce,
        initCode: user_op.init_code.clone(),
        callData: user_op.call_data.clone(),
        accountGasLimits: account_gas_limits.into(),
        preVerificationGas: user_op.pre_verification_gas,
        gasFees: gas_fees.into(),
        paymasterAndData: paymaster_and_data,
        signature: user_op.signature.clone(),
    }
}

fn pack_account_gas_limits(verification_gas_limit: U256, call_gas_limit: U256) -> [u8; 32] {
    let mut result = [0u8; 32];
    
    // Convert verification_gas_limit to bytes and pad to 16 bytes
    let ver_gas_bytes: [u8; 32] = verification_gas_limit.to_be_bytes();
    result[..16].copy_from_slice(&ver_gas_bytes[16..32]); // Take last 16 bytes
    
    // Convert call_gas_limit to bytes and pad to 16 bytes
    let call_gas_bytes: [u8; 32] = call_gas_limit.to_be_bytes();
    result[16..].copy_from_slice(&call_gas_bytes[16..32]); // Take last 16 bytes
    
    result
}

fn pack_paymaster_data(
    paymaster: Address,
    paymaster_verification_gas_limit: U256,
    post_op_gas_limit: U256,
    paymaster_data: Bytes,
) -> Bytes {
    let mut result = Vec::with_capacity(20 + 32 + paymaster_data.len());
    
    // Add paymaster address (20 bytes)
    result.extend_from_slice(paymaster.as_slice());
    
    // Add verification gas limit (16 bytes)
    let ver_gas_bytes: [u8; 32] = paymaster_verification_gas_limit.to_be_bytes();
    result.extend_from_slice(&ver_gas_bytes[16..32]); // Take last 16 bytes
    
    // Add post op gas limit (16 bytes)
    let post_gas_bytes: [u8; 32] = post_op_gas_limit.to_be_bytes();
    result.extend_from_slice(&post_gas_bytes[16..32]); // Take last 16 bytes
    
    // Add paymaster data
    result.extend_from_slice(&paymaster_data);
    
    Bytes::from(result)
}

sol! {
    struct UserOpForSignature {
        address sender;
        uint256 nonce;
        bytes32 initCodeHash;
        bytes32 callDataHash;
        bytes32 accountGasLimits;
        uint256 preVerificationGas;
        bytes32 gasFees;
        bytes32 paymasterAndDataHash;
    }
}

sol! {
    struct UserOpForGas {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_get_user_op_hash() {
        // Create a test UserOperation
        let user_op = UserOperation {
            sender: Address::from_slice(&[1u8; 20]),
            nonce: U256::from(1),
            init_code: Bytes::from(vec![2u8; 32]),
            call_data: Bytes::from(vec![3u8; 32]),
            verification_gas_limit: U256::from(100000),
            call_gas_limit: U256::from(200000),
            pre_verification_gas: U256::from(300000),
            max_fee_per_gas: U256::from(400000),
            max_priority_fee_per_gas: U256::from(500000),
            paymaster: Address::from_slice(&[4u8; 20]),
            paymaster_verification_gas_limit: U256::from(600000),
            paymaster_post_op_gas_limit: U256::from(700000),
            paymaster_data: Bytes::from(vec![5u8; 32]),
            signature: Bytes::from(vec![6u8; 65]),
        };

        // Test values
        let entry_point = Address::from_str("0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789").unwrap();
        let chain_id = U256::from(1);

        let hash = get_user_op_hash(&user_op, entry_point, chain_id);
        
        // The hash should be 32 bytes
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_pack_account_gas_limits() {
        let verification_gas = U256::from(100000);
        let call_gas = U256::from(200000);
        
        let result = pack_account_gas_limits(verification_gas, call_gas);
        
        // Create full 32-byte arrays with padding
        let mut ver_bytes = [0u8; 32];
        let mut call_bytes = [0u8; 32];
        ver_bytes[16..].copy_from_slice(&result[..16]);
        call_bytes[16..].copy_from_slice(&result[16..]);
        
        assert_eq!(U256::from_be_bytes(ver_bytes), verification_gas);
        assert_eq!(U256::from_be_bytes(call_bytes), call_gas);
    }

    #[test]
    fn test_pack_paymaster_data() {
        let paymaster = Address::from_str("0x1234567890123456789012345678901234567890").unwrap();
        let verification_gas = U256::from(100000);
        let post_op_gas = U256::from(200000);
        let paymaster_data = Bytes::from(vec![1, 2, 3, 4]);
        
        let result = pack_paymaster_data(paymaster, verification_gas, post_op_gas, paymaster_data);
        
        // Check length (20 + 16 + 16 + 4 = 56 bytes)
        assert_eq!(result.len(), 56);
        
        // Check paymaster address (first 20 bytes)
        assert_eq!(&result[..20], paymaster.as_slice());
        
        // Verify gas limits can be recovered
        let ver_gas_slice: [u8; 16] = result[20..36].try_into().unwrap();
        let mut full_bytes = [0u8; 32];
        full_bytes[16..].copy_from_slice(&ver_gas_slice);
        assert_eq!(U256::from_be_bytes(full_bytes), verification_gas);
        
        // Check paymaster data at the end
        assert_eq!(&result[52..], &[1, 2, 3, 4]);
    }
}
