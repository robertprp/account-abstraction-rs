use alloy::{hex, primitives::{Address, Bytes, B256, U256}};
use serde::Serialize;

#[derive(Clone, Serialize, PartialEq, Eq, Debug)]
pub struct UserOperationRequest {
    #[serde(skip_serializing)]
    pub call: AccountCall,

    /// The account making the operation
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sender: Option<Address>,

    /// Anti-replay parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<U256>,

    /// Account factory, only for new accounts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub factory: Option<Address>,

    /// Data for account factory
    #[serde(
        rename = "factoryData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factory_data: Option<Bytes>,

    /// The data to pass to the sender during the main execution call
    #[serde(rename = "callData", default, skip_serializing_if = "Option::is_none")]
    pub call_data: Option<Bytes>,

    /// The amount of gas to allocate the main execution call
    #[serde(
        rename = "callGasLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub call_gas_limit: Option<U256>,

    /// The amount of gas to allocate for the verification step
    #[serde(
        rename = "verificationGasLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_gas_limit: Option<U256>,

    /// Extra gas to pay the bundler
    #[serde(
        rename = "preVerificationGas",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_verification_gas: Option<U256>,

    /// Maximum fee per gas (similar to EIP-1559 max_fee_per_gas)
    #[serde(
        rename = "maxFeePerGas",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_fee_per_gas: Option<U256>,

    /// Maximum priority fee per gas (similar to EIP-1559 max_priority_fee_per_gas)
    #[serde(
        rename = "maxPriorityFeePerGas",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_priority_fee_per_gas: Option<U256>,

    /// Address of paymaster contract
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paymaster: Option<Address>,

    /// Gas limit for paymaster validation
    #[serde(
        rename = "paymasterVerificationGasLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub paymaster_verification_gas_limit: Option<U256>,

    /// Gas limit for paymaster post-operation
    #[serde(
        rename = "paymasterPostOpGasLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub paymaster_post_op_gas_limit: Option<U256>,

    /// Data for paymaster
    #[serde(
        rename = "paymasterData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub paymaster_data: Option<Bytes>,

    /// Data passed into the account to verify authorization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Bytes>,

    /// EIP-7702 authorization data
    #[serde(
        rename = "eip7702Auth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub eip7702_auth: Option<Eip7702Auth>,
}

impl UserOperationRequest {
    pub fn new(call: AccountCall) -> Self {
        Self {
            sender: None,
            nonce: None,
            factory: None,
            factory_data: None,
            call_data: None,
            call_gas_limit: None,
            verification_gas_limit: None,
            pre_verification_gas: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            paymaster: None,
            paymaster_verification_gas_limit: None,
            paymaster_post_op_gas_limit: None,
            paymaster_data: None,
            signature: None,
            eip7702_auth: None,
            call,
        }
    }

    pub fn with_defaults(self) -> Self {
        Self {
            call: self.call,
            sender: Some(self.sender.unwrap_or_else(|| Address::ZERO)),
            nonce: Some(self.nonce.unwrap_or_else(|| U256::ZERO)),
            factory: self.factory,
            factory_data: self.factory_data,
            call_data: Some(self.call_data.unwrap_or_else(|| Bytes::from(vec![0u8; 0]))),
            call_gas_limit: Some(self.call_gas_limit.unwrap_or_else(|| U256::ZERO)),
            verification_gas_limit: Some(self.verification_gas_limit.unwrap_or_else(|| U256::ZERO)),
            pre_verification_gas: Some(self.pre_verification_gas.unwrap_or_else(|| U256::ZERO)),
            max_fee_per_gas: Some(self.max_fee_per_gas.unwrap_or_else(|| U256::ZERO)),
            max_priority_fee_per_gas: Some(self.max_priority_fee_per_gas.unwrap_or_else(|| U256::ZERO)),
            paymaster: self.paymaster,
            paymaster_verification_gas_limit: self.paymaster_verification_gas_limit,
            paymaster_post_op_gas_limit: self.paymaster_post_op_gas_limit,
            paymaster_data: self.paymaster_data,
            signature: Some(self.signature.unwrap_or_else(|| "0xfffffffffffffffffffffffffffffff0000000000000000000000000000000007aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1c".parse().unwrap())),
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
    pub fn factory<T: Into<Address>>(mut self, factory: T) -> Self {
        self.factory = Some(factory.into());
        self
    }

    #[must_use]
    pub fn factory_data<T: Into<Bytes>>(mut self, factory_data: T) -> Self {
        self.factory_data = Some(factory_data.into());
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
    pub fn paymaster<T: Into<Address>>(mut self, paymaster: T) -> Self {
        self.paymaster = Some(paymaster.into());
        self
    }

    #[must_use]
    pub fn paymaster_verification_gas_limit<T: Into<U256>>(mut self, gas_limit: T) -> Self {
        self.paymaster_verification_gas_limit = Some(gas_limit.into());
        self
    }

    #[must_use]
    pub fn paymaster_post_op_gas_limit<T: Into<U256>>(mut self, gas_limit: T) -> Self {
        self.paymaster_post_op_gas_limit = Some(gas_limit.into());
        self
    }

    #[must_use]
    pub fn paymaster_data<T: Into<Bytes>>(mut self, data: T) -> Self {
        self.paymaster_data = Some(data.into());
        self
    }

    #[must_use]
    pub fn signature<T: Into<Bytes>>(mut self, signature: T) -> Self {
        self.signature = Some(signature.into());
        self
    }

    #[must_use]
    pub fn eip7702_auth(mut self, auth: Eip7702Auth) -> Self {
        self.eip7702_auth = Some(auth);
        self
    }

    pub async fn build_user_operation(self) -> Result<UserOperation, &'static str> {
        Ok(UserOperation {
            sender: self
                .sender
                .ok_or("Missing 'sender' field for UserOperation")?,
            nonce: self
                .nonce
                .ok_or("Missing 'nonce' field for UserOperation")?,
            factory: self.factory,
            factory_data: self.factory_data,
            call_data: self
                .call_data
                .ok_or("Missing 'call_data' field for UserOperation")?,
            verification_gas_limit: self
                .verification_gas_limit
                .ok_or("Missing 'verification_gas_limit' field for UserOperation")?,
            call_gas_limit: self
                .call_gas_limit
                .ok_or("Missing 'call_gas_limit' field for UserOperation")?,
            pre_verification_gas: self
                .pre_verification_gas
                .ok_or("Missing 'pre_verification_gas' field for UserOperation")?,
            max_fee_per_gas: self
                .max_fee_per_gas
                .ok_or("Missing 'max_fee_per_gas' field for UserOperation")?,
            max_priority_fee_per_gas: self
                .max_priority_fee_per_gas
                .ok_or("Missing 'max_priority_fee_per_gas' field for UserOperation")?,
            paymaster: self.paymaster,
            paymaster_verification_gas_limit: self.paymaster_verification_gas_limit,
            paymaster_post_op_gas_limit: self.paymaster_post_op_gas_limit,
            paymaster_data: self.paymaster_data,
            signature: self
                .signature
                .ok_or("Missing 'signature' field for UserOperation")?,
            eip7702_auth: self.eip7702_auth,
        })
    }
}

impl From<UserOperationRequest> for UserOperation {
    fn from(request: UserOperationRequest) -> Self {
        Self {
            sender: request.sender.unwrap_or_else(|| Address::ZERO),
            nonce: request.nonce.unwrap_or_else(|| U256::ZERO),
            factory: request.factory,
            factory_data: request.factory_data,
            call_data: request.call_data.unwrap_or_else(|| Bytes::new()),
            verification_gas_limit: request.verification_gas_limit.unwrap_or_else(|| U256::ZERO),
            call_gas_limit: request.call_gas_limit.unwrap_or_else(|| U256::ZERO),
            pre_verification_gas: request.pre_verification_gas.unwrap_or_else(|| U256::ZERO),
            max_fee_per_gas: request.max_fee_per_gas.unwrap_or_else(|| U256::ZERO),
            max_priority_fee_per_gas: request
                .max_priority_fee_per_gas
                .unwrap_or_else(|| U256::ZERO),
            paymaster: request.paymaster,
            paymaster_verification_gas_limit: request.paymaster_verification_gas_limit,
            paymaster_post_op_gas_limit: request.paymaster_post_op_gas_limit,
            paymaster_data: request.paymaster_data,
            signature: request.signature.unwrap_or_else(|| Bytes::new()),
        }
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

/// [`UserOperation`] in the spec: Entry Point V0.7
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserOperation {
    /// The account making the operation.
    pub sender: Address,
    /// Prevents message replay attacks and serves as a randomizing element for initial user
    /// registration.
    pub nonce: U256,
    /// Deployer contract address: Required exclusively for deploying new accounts that don't yet
    /// exist on the blockchain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub factory: Option<Address>,
    /// Factory data for the account creation process, applicable only when using a deployer
    /// contract.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub factory_data: Option<Bytes>,
    /// The call data.
    pub call_data: Bytes,
    /// The gas limit for the call.
    pub call_gas_limit: U256,
    /// The gas limit for the verification.
    pub verification_gas_limit: U256,
    /// Prepaid gas fee: Covers the bundler's costs for initial transaction validation and data
    /// transmission.
    pub pre_verification_gas: U256,
    /// The maximum fee per gas.
    pub max_fee_per_gas: U256,
    /// The maximum priority fee per gas.
    pub max_priority_fee_per_gas: U256,
    /// Paymaster contract address: Needed if a third party is covering transaction costs; left
    /// blank for self-funded accounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paymaster: Option<Address>,
    /// The gas limit for the paymaster verification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paymaster_verification_gas_limit: Option<U256>,
    /// The gas limit for the paymaster post-operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paymaster_post_op_gas_limit: Option<U256>,
    /// The paymaster data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paymaster_data: Option<Bytes>,
    /// The signature of the transaction.
    pub signature: Bytes,
    /// EIP-7702 authentication data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eip7702_auth: Option<Eip7702Auth>,
}

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eip7702Auth {
    pub chain_id: U256,
    pub nonce: U256,
    pub address: Address,
    pub r: B256,
    pub s: B256,
    pub y_parity: U256,
}
