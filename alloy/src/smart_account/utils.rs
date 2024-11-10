use alloy::primitives::{Address, Bytes, U256};

use crate::types::{PackedUserOperation, UserOperation};

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

pub fn pack_account_gas_limits(verification_gas_limit: U256, call_gas_limit: U256) -> [u8; 32] {
    let mut result = [0u8; 32];
    
    // Convert verification_gas_limit to bytes and pad to 16 bytes
    let ver_gas_bytes: [u8; 32] = verification_gas_limit.to_be_bytes();
    result[..16].copy_from_slice(&ver_gas_bytes[16..32]); // Take last 16 bytes
    
    // Convert call_gas_limit to bytes and pad to 16 bytes
    let call_gas_bytes: [u8; 32] = call_gas_limit.to_be_bytes();
    result[16..].copy_from_slice(&call_gas_bytes[16..32]); // Take last 16 bytes
    
    result
}

pub fn pack_paymaster_data(
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

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
