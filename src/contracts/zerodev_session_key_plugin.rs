pub use zero_dev_session_key_plugin::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod zero_dev_session_key_plugin {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"key\",\"type\":\"address\"}],\"name\":\"SessionKeyRevoked\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_packed\",\"type\":\"bytes\"}],\"name\":\"parseDataAndSignature\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_key\",\"type\":\"address\"}],\"name\":\"revokeSessionKey\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_key\",\"type\":\"address\"}],\"name\":\"revoked\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"initCode\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\"},{\"internalType\":\"uint256\",\"name\":\"callGasLimit\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"verificationGasLimit\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"preVerificationGas\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"maxFeePerGas\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"maxPriorityFeePerGas\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"paymasterAndData\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"}],\"internalType\":\"struct UserOperation\",\"name\":\"userOp\",\"type\":\"tuple\"},{\"internalType\":\"bytes32\",\"name\":\"userOpHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"missingAccountFunds\",\"type\":\"uint256\"}],\"name\":\"validatePluginData\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"validated\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]";
    ///The parsed JSON ABI of the contract.
    pub static ZERODEVSESSIONKEYPLUGIN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct ZeroDevSessionKeyPlugin<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ZeroDevSessionKeyPlugin<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ZeroDevSessionKeyPlugin<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ZeroDevSessionKeyPlugin<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ZeroDevSessionKeyPlugin<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ZeroDevSessionKeyPlugin))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ZeroDevSessionKeyPlugin<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ZERODEVSESSIONKEYPLUGIN_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `parseDataAndSignature` (0x970aa9ad) function
        pub fn parse_data_and_signature(
            &self,
            packed: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Bytes, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([151, 10, 169, 173], packed)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeSessionKey` (0x84f4fc6a) function
        pub fn revoke_session_key(
            &self,
            key: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 244, 252, 106], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revoked` (0xfa01dc06) function
        pub fn revoked(
            &self,
            key: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 1, 220, 6], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validatePluginData` (0x9e2045ce) function
        pub fn validate_plugin_data(
            &self,
            user_op: UserOperation,
            user_op_hash: [u8; 32],
            missing_account_funds: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [158, 32, 69, 206],
                    (user_op, user_op_hash, missing_account_funds),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `SessionKeyRevoked` event
        pub fn session_key_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SessionKeyRevokedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SessionKeyRevokedFilter>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for ZeroDevSessionKeyPlugin<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SessionKeyRevoked", abi = "SessionKeyRevoked(address)")]
    pub struct SessionKeyRevokedFilter {
        #[ethevent(indexed)]
        pub key: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `parseDataAndSignature` function with signature `parseDataAndSignature(bytes)` and selector `0x970aa9ad`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "parseDataAndSignature", abi = "parseDataAndSignature(bytes)")]
    pub struct ParseDataAndSignatureCall {
        pub packed: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `revokeSessionKey` function with signature `revokeSessionKey(address)` and selector `0x84f4fc6a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "revokeSessionKey", abi = "revokeSessionKey(address)")]
    pub struct RevokeSessionKeyCall {
        pub key: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revoked` function with signature `revoked(address)` and selector `0xfa01dc06`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "revoked", abi = "revoked(address)")]
    pub struct RevokedCall {
        pub key: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `validatePluginData` function with signature `validatePluginData((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)` and selector `0x9e2045ce`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "validatePluginData",
        abi = "validatePluginData((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)"
    )]
    pub struct ValidatePluginDataCall {
        pub user_op: UserOperation,
        pub user_op_hash: [u8; 32],
        pub missing_account_funds: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ZeroDevSessionKeyPluginCalls {
        ParseDataAndSignature(ParseDataAndSignatureCall),
        RevokeSessionKey(RevokeSessionKeyCall),
        Revoked(RevokedCall),
        ValidatePluginData(ValidatePluginDataCall),
    }
    impl ::ethers::core::abi::AbiDecode for ZeroDevSessionKeyPluginCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ParseDataAndSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ParseDataAndSignature(decoded));
            }
            if let Ok(decoded) =
                <RevokeSessionKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevokeSessionKey(decoded));
            }
            if let Ok(decoded) = <RevokedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Revoked(decoded));
            }
            if let Ok(decoded) =
                <ValidatePluginDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidatePluginData(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ZeroDevSessionKeyPluginCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ParseDataAndSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeSessionKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Revoked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidatePluginData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ZeroDevSessionKeyPluginCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ParseDataAndSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeSessionKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::Revoked(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidatePluginData(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ParseDataAndSignatureCall> for ZeroDevSessionKeyPluginCalls {
        fn from(value: ParseDataAndSignatureCall) -> Self {
            Self::ParseDataAndSignature(value)
        }
    }
    impl ::core::convert::From<RevokeSessionKeyCall> for ZeroDevSessionKeyPluginCalls {
        fn from(value: RevokeSessionKeyCall) -> Self {
            Self::RevokeSessionKey(value)
        }
    }
    impl ::core::convert::From<RevokedCall> for ZeroDevSessionKeyPluginCalls {
        fn from(value: RevokedCall) -> Self {
            Self::Revoked(value)
        }
    }
    impl ::core::convert::From<ValidatePluginDataCall> for ZeroDevSessionKeyPluginCalls {
        fn from(value: ValidatePluginDataCall) -> Self {
            Self::ValidatePluginData(value)
        }
    }
    ///Container type for all return fields from the `parseDataAndSignature` function with signature `parseDataAndSignature(bytes)` and selector `0x970aa9ad`
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
    pub struct ParseDataAndSignatureReturn {
        pub data: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `revoked` function with signature `revoked(address)` and selector `0xfa01dc06`
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
    pub struct RevokedReturn(pub bool);
    ///Container type for all return fields from the `validatePluginData` function with signature `validatePluginData((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)` and selector `0x9e2045ce`
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
    pub struct ValidatePluginDataReturn {
        pub validated: bool,
    }
    ///`UserOperation(address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)`
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
    pub struct UserOperation {
        pub sender: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub init_code: ::ethers::core::types::Bytes,
        pub call_data: ::ethers::core::types::Bytes,
        pub call_gas_limit: ::ethers::core::types::U256,
        pub verification_gas_limit: ::ethers::core::types::U256,
        pub pre_verification_gas: ::ethers::core::types::U256,
        pub max_fee_per_gas: ::ethers::core::types::U256,
        pub max_priority_fee_per_gas: ::ethers::core::types::U256,
        pub paymaster_and_data: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
    }
}
