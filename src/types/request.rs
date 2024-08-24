// use ethers::types::{Address, Bytes, H256, U256};
use alloy::primitives::{Address, Bytes, B256, U256};
use serde::Serialize;

/// Parameters for sending a user operation
#[derive(Clone, Default, Serialize, PartialEq, Eq, Debug)]
pub struct UserOperationRequest {
    /// Sender address
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sender: Option<Address>,

    /// Nonce
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<U256>,

    /// Init code
    #[serde(rename = "initCode", default, skip_serializing_if = "Option::is_none")]
    pub init_code: Option<Bytes>,

    /// Call data
    #[serde(rename = "callData", default, skip_serializing_if = "Option::is_none")]
    pub call_data: Option<Bytes>,

    /// Call gas limit
    #[serde(
        rename = "callGasLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub call_gas_limit: Option<U256>,

    /// Verification gas limit
    #[serde(
        rename = "verificationGasLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_gas_limit: Option<U256>,

    /// Pre-verification gas limit
    #[serde(
        rename = "preVerificationGas",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_verification_gas: Option<U256>,

    /// Represents the maximum amount that a user is willing
    /// to pay for their tx (inclusive of baseFeePerGas and maxPriorityFeePerGas).
    #[serde(
        rename = "maxFeePerGas",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_fee_per_gas: Option<U256>,

    /// Represents the maximum tx fee that will go to the miner as part of the user's
    #[serde(
        rename = "maxPriorityFeePerGas",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_priority_fee_per_gas: Option<U256>,

    /// Sender address
    #[serde(
        rename = "paymasterAndData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub paymaster_and_data: Option<Bytes>,

    /// Sender address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Bytes>,

    /// Helper properties for encodeExecute

    #[serde(skip_serializing)]
    pub call: Option<AccountCall>,
}

impl UserOperationRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_defaults(self) -> Self {
        Self {
            sender: Some(self.sender.unwrap_or_else(|| Address::zero())),
            nonce: Some(self.nonce.unwrap_or_else(|| U256::zero())),
            init_code: Some(self.init_code.unwrap_or_else(|| Bytes::from(vec![0u8; 0]))),
            call_data: Some(self.call_data.unwrap_or_else(|| Bytes::from(vec![0u8; 0]))),
            call_gas_limit: Some(self.call_gas_limit.unwrap_or_else(|| U256::zero())),
            verification_gas_limit: Some(self.verification_gas_limit.unwrap_or_else(|| U256::zero())),
            pre_verification_gas: Some(self.pre_verification_gas.unwrap_or_else(|| U256::zero())),
            max_fee_per_gas: Some(self.max_fee_per_gas.unwrap_or_else(|| U256::zero())),
            max_priority_fee_per_gas: Some(self.max_priority_fee_per_gas.unwrap_or_else(|| U256::zero())),
            paymaster_and_data: Some(self.paymaster_and_data.unwrap_or_else(|| Bytes::from(vec![0u8; 0]))),
            // Dummy signature
            signature: Some(self.signature.unwrap_or_else(|| "0xb6905c3cc524616247f41b6de20bced7d4437a6513a5cf1c90ab6a28415eb6993e1236443130bce47d1580ab289bcc8f32270acb4ae7e46a17fe158f473fd4991c".parse().unwrap())),
            call: self.call
        }
    }

    #[must_use]
    pub fn sender<T: Into<Address>>(mut self, sender: T) -> Self {
        self.sender = Some(sender.into());
        self
    }

    #[must_use]
    pub fn nonce<T: Into<U256>>(mut self, nonce: T) -> Self {
        self.nonce = Some(nonce.into());
        self
    }

    pub fn set_nonce<T: Into<U256>>(&mut self, nonce: T) -> &mut Self {
        self.nonce = Some(nonce.into());
        self
    }

    #[must_use]
    pub fn init_code<T: Into<Bytes>>(mut self, init_code: T) -> Self {
        self.init_code = Some(init_code.into());
        self
    }

    #[must_use]
    pub fn call_data<T: Into<Bytes>>(mut self, call_data: T) -> Self {
        self.call_data = Some(call_data.into());
        self
    }

    #[must_use]
    pub fn call_gas_limit<T: Into<U256>>(mut self, call_gas_limit: T) -> Self {
        self.call_gas_limit = Some(call_gas_limit.into());
        self
    }

    #[must_use]
    pub fn verification_gas_limit<T: Into<U256>>(mut self, verification_gas_limit: T) -> Self {
        self.verification_gas_limit = Some(verification_gas_limit.into());
        self
    }

    #[must_use]
    pub fn pre_verification_gas<T: Into<U256>>(mut self, pre_verification_gas: T) -> Self {
        self.pre_verification_gas = Some(pre_verification_gas.into());
        self
    }

    #[must_use]
    pub fn max_fee_per_gas<T: Into<U256>>(mut self, max_fee_per_gas: T) -> Self {
        self.max_fee_per_gas = Some(max_fee_per_gas.into());
        self
    }

    #[must_use]
    pub fn max_priority_fee_per_gas<T: Into<U256>>(mut self, max_priority_fee_per_gas: T) -> Self {
        self.max_priority_fee_per_gas = Some(max_priority_fee_per_gas.into());
        self
    }

    #[must_use]
    pub fn paymaster_and_data<T: Into<Bytes>>(mut self, paymaster_and_data: T) -> Self {
        self.paymaster_and_data = Some(paymaster_and_data.into());
        self
    }

    #[must_use]
    pub fn signature<T: Into<Bytes>>(mut self, signature: T) -> Self {
        self.signature = Some(signature.into());
        self
    }

    #[must_use]
    pub fn call<T: Into<AccountCall>>(mut self, call: T) -> Self {
        self.call = Some(call.into());
        self
    }

    #[must_use]
    pub fn execute<T: Into<Address>, V: Into<U256>, D: Into<Bytes>>(
        mut self,
        target: T,
        value: V,
        data: D,
    ) -> Self {
        self.call = Some(AccountCall::Execute(ExecuteCall::new(
            target.into(),
            value.into(),
            data.into(),
        )));
        self
    }

    #[must_use]
    pub fn execute_batch(mut self, calls: Vec<ExecuteCall>) -> Self {
        self.call = Some(AccountCall::ExecuteBatch(calls));
        self
    }
}

pub type UserOpHash = B256;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AccountCall {
    Execute(ExecuteCall),
    ExecuteBatch(Vec<ExecuteCall>),
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct ExecuteCall {
    pub target: Address,
    pub value: U256,
    pub data: Bytes,
}

impl ExecuteCall {
    pub fn new<T: Into<Address>, V: Into<U256>, B: Into<Bytes>>(
        target: T,
        value: V,
        data: B,
    ) -> Self {
        Self {
            target: target.into(),
            value: value.into(),
            data: data.into(),
        }
    }
}
