pub use safe_signer_launch_pad::*;
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
pub mod safe_signer_launch_pad {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"entryPoint\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"SUPPORTED_ENTRYPOINT\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"_initHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"value\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"singleton\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"signerFactory\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"signerData\",\"type\":\"bytes\"},{\"internalType\":\"address\",\"name\":\"setupTo\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"setupData\",\"type\":\"bytes\"},{\"internalType\":\"address\",\"name\":\"fallbackHandler\",\"type\":\"address\"}],\"name\":\"getInitHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"initHash\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"userOpHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint48\",\"name\":\"validAfter\",\"type\":\"uint48\"},{\"internalType\":\"uint48\",\"name\":\"validUntil\",\"type\":\"uint48\"}],\"name\":\"getOperationHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"operationHash\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"singleton\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"signerFactory\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"signerData\",\"type\":\"bytes\"},{\"internalType\":\"address\",\"name\":\"setupTo\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"setupData\",\"type\":\"bytes\"},{\"internalType\":\"address\",\"name\":\"fallbackHandler\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\"}],\"name\":\"initializeThenUserOp\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"initHash\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"preInit\",\"type\":\"bytes\"}],\"name\":\"preValidationSetup\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"initCode\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\"},{\"internalType\":\"uint256\",\"name\":\"callGasLimit\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"verificationGasLimit\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"preVerificationGas\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"maxFeePerGas\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"maxPriorityFeePerGas\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"paymasterAndData\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"}],\"internalType\":\"struct UserOperation\",\"name\":\"userOp\",\"type\":\"tuple\"},{\"internalType\":\"bytes32\",\"name\":\"userOpHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"missingAccountFunds\",\"type\":\"uint256\"}],\"name\":\"validateUserOp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"validationData\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"stateMutability\":\"payable\",\"type\":\"receive\"}]";
    ///The parsed JSON ABI of the contract.
    pub static SAFESIGNERLAUNCHPAD_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct SafeSignerLaunchPad<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SafeSignerLaunchPad<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SafeSignerLaunchPad<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SafeSignerLaunchPad<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SafeSignerLaunchPad<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(SafeSignerLaunchPad))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SafeSignerLaunchPad<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SAFESIGNERLAUNCHPAD_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `SUPPORTED_ENTRYPOINT` (0x137e051e) function
        pub fn supported_entrypoint(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([19, 126, 5, 30], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_initHash` (0xc67e2d2a) function
        pub fn init_hash(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([198, 126, 45, 42], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInitHash` (0x6cbc5b97) function
        pub fn get_init_hash(
            &self,
            singleton: ::ethers::core::types::Address,
            signer_factory: ::ethers::core::types::Address,
            signer_data: ::ethers::core::types::Bytes,
            setup_to: ::ethers::core::types::Address,
            setup_data: ::ethers::core::types::Bytes,
            fallback_handler: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [108, 188, 91, 151],
                    (
                        singleton,
                        signer_factory,
                        signer_data,
                        setup_to,
                        setup_data,
                        fallback_handler,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperationHash` (0xc0d13687) function
        pub fn get_operation_hash(
            &self,
            user_op_hash: [u8; 32],
            valid_after: u64,
            valid_until: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [192, 209, 54, 135],
                    (user_op_hash, valid_after, valid_until),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initializeThenUserOp` (0x1e48143b) function
        pub fn initialize_then_user_op(
            &self,
            singleton: ::ethers::core::types::Address,
            signer_factory: ::ethers::core::types::Address,
            signer_data: ::ethers::core::types::Bytes,
            setup_to: ::ethers::core::types::Address,
            setup_data: ::ethers::core::types::Bytes,
            fallback_handler: ::ethers::core::types::Address,
            call_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [30, 72, 20, 59],
                    (
                        singleton,
                        signer_factory,
                        signer_data,
                        setup_to,
                        setup_data,
                        fallback_handler,
                        call_data,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `preValidationSetup` (0x4fff40e1) function
        pub fn pre_validation_setup(
            &self,
            init_hash: [u8; 32],
            to: ::ethers::core::types::Address,
            pre_init: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 255, 64, 225], (init_hash, to, pre_init))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateUserOp` (0x3a871cdd) function
        pub fn validate_user_op(
            &self,
            user_op: UserOperation,
            user_op_hash: [u8; 32],
            missing_account_funds: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [58, 135, 28, 221],
                    (user_op, user_op_hash, missing_account_funds),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for SafeSignerLaunchPad<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `SUPPORTED_ENTRYPOINT` function with signature `SUPPORTED_ENTRYPOINT()` and selector `0x137e051e`
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
    #[ethcall(name = "SUPPORTED_ENTRYPOINT", abi = "SUPPORTED_ENTRYPOINT()")]
    pub struct SupportedEntrypointCall;
    ///Container type for all input parameters for the `_initHash` function with signature `_initHash()` and selector `0xc67e2d2a`
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
    #[ethcall(name = "_initHash", abi = "_initHash()")]
    pub struct InitHashCall;
    ///Container type for all input parameters for the `getInitHash` function with signature `getInitHash(address,address,bytes,address,bytes,address)` and selector `0x6cbc5b97`
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
        name = "getInitHash",
        abi = "getInitHash(address,address,bytes,address,bytes,address)"
    )]
    pub struct GetInitHashCall {
        pub singleton: ::ethers::core::types::Address,
        pub signer_factory: ::ethers::core::types::Address,
        pub signer_data: ::ethers::core::types::Bytes,
        pub setup_to: ::ethers::core::types::Address,
        pub setup_data: ::ethers::core::types::Bytes,
        pub fallback_handler: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getOperationHash` function with signature `getOperationHash(bytes32,uint48,uint48)` and selector `0xc0d13687`
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
        name = "getOperationHash",
        abi = "getOperationHash(bytes32,uint48,uint48)"
    )]
    pub struct GetOperationHashCall {
        pub user_op_hash: [u8; 32],
        pub valid_after: u64,
        pub valid_until: u64,
    }
    ///Container type for all input parameters for the `initializeThenUserOp` function with signature `initializeThenUserOp(address,address,bytes,address,bytes,address,bytes)` and selector `0x1e48143b`
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
        name = "initializeThenUserOp",
        abi = "initializeThenUserOp(address,address,bytes,address,bytes,address,bytes)"
    )]
    pub struct InitializeThenUserOpCall {
        pub singleton: ::ethers::core::types::Address,
        pub signer_factory: ::ethers::core::types::Address,
        pub signer_data: ::ethers::core::types::Bytes,
        pub setup_to: ::ethers::core::types::Address,
        pub setup_data: ::ethers::core::types::Bytes,
        pub fallback_handler: ::ethers::core::types::Address,
        pub call_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `preValidationSetup` function with signature `preValidationSetup(bytes32,address,bytes)` and selector `0x4fff40e1`
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
        name = "preValidationSetup",
        abi = "preValidationSetup(bytes32,address,bytes)"
    )]
    pub struct PreValidationSetupCall {
        pub init_hash: [u8; 32],
        pub to: ::ethers::core::types::Address,
        pub pre_init: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `validateUserOp` function with signature `validateUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)` and selector `0x3a871cdd`
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
        name = "validateUserOp",
        abi = "validateUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)"
    )]
    pub struct ValidateUserOpCall {
        pub user_op: UserOperation,
        pub user_op_hash: [u8; 32],
        pub missing_account_funds: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SafeSignerLaunchPadCalls {
        SupportedEntrypoint(SupportedEntrypointCall),
        InitHash(InitHashCall),
        GetInitHash(GetInitHashCall),
        GetOperationHash(GetOperationHashCall),
        InitializeThenUserOp(InitializeThenUserOpCall),
        PreValidationSetup(PreValidationSetupCall),
        ValidateUserOp(ValidateUserOpCall),
    }
    impl ::ethers::core::abi::AbiDecode for SafeSignerLaunchPadCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <SupportedEntrypointCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SupportedEntrypoint(decoded));
            }
            if let Ok(decoded) = <InitHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InitHash(decoded));
            }
            if let Ok(decoded) = <GetInitHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetInitHash(decoded));
            }
            if let Ok(decoded) =
                <GetOperationHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOperationHash(decoded));
            }
            if let Ok(decoded) =
                <InitializeThenUserOpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InitializeThenUserOp(decoded));
            }
            if let Ok(decoded) =
                <PreValidationSetupCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PreValidationSetup(decoded));
            }
            if let Ok(decoded) =
                <ValidateUserOpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateUserOp(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SafeSignerLaunchPadCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::SupportedEntrypoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetInitHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOperationHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InitializeThenUserOp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreValidationSetup(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateUserOp(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SafeSignerLaunchPadCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SupportedEntrypoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInitHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperationHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializeThenUserOp(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreValidationSetup(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateUserOp(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<SupportedEntrypointCall> for SafeSignerLaunchPadCalls {
        fn from(value: SupportedEntrypointCall) -> Self {
            Self::SupportedEntrypoint(value)
        }
    }
    impl ::core::convert::From<InitHashCall> for SafeSignerLaunchPadCalls {
        fn from(value: InitHashCall) -> Self {
            Self::InitHash(value)
        }
    }
    impl ::core::convert::From<GetInitHashCall> for SafeSignerLaunchPadCalls {
        fn from(value: GetInitHashCall) -> Self {
            Self::GetInitHash(value)
        }
    }
    impl ::core::convert::From<GetOperationHashCall> for SafeSignerLaunchPadCalls {
        fn from(value: GetOperationHashCall) -> Self {
            Self::GetOperationHash(value)
        }
    }
    impl ::core::convert::From<InitializeThenUserOpCall> for SafeSignerLaunchPadCalls {
        fn from(value: InitializeThenUserOpCall) -> Self {
            Self::InitializeThenUserOp(value)
        }
    }
    impl ::core::convert::From<PreValidationSetupCall> for SafeSignerLaunchPadCalls {
        fn from(value: PreValidationSetupCall) -> Self {
            Self::PreValidationSetup(value)
        }
    }
    impl ::core::convert::From<ValidateUserOpCall> for SafeSignerLaunchPadCalls {
        fn from(value: ValidateUserOpCall) -> Self {
            Self::ValidateUserOp(value)
        }
    }
    ///Container type for all return fields from the `SUPPORTED_ENTRYPOINT` function with signature `SUPPORTED_ENTRYPOINT()` and selector `0x137e051e`
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
    pub struct SupportedEntrypointReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `_initHash` function with signature `_initHash()` and selector `0xc67e2d2a`
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
    pub struct InitHashReturn {
        pub value: [u8; 32],
    }
    ///Container type for all return fields from the `getInitHash` function with signature `getInitHash(address,address,bytes,address,bytes,address)` and selector `0x6cbc5b97`
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
    pub struct GetInitHashReturn {
        pub init_hash: [u8; 32],
    }
    ///Container type for all return fields from the `getOperationHash` function with signature `getOperationHash(bytes32,uint48,uint48)` and selector `0xc0d13687`
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
    pub struct GetOperationHashReturn {
        pub operation_hash: [u8; 32],
    }
    ///Container type for all return fields from the `validateUserOp` function with signature `validateUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)` and selector `0x3a871cdd`
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
    pub struct ValidateUserOpReturn {
        pub validation_data: ::ethers::core::types::U256,
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
