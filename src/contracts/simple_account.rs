pub use simple_account::*;
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
pub mod simple_account {
    #[rustfmt::skip]
    const __ABI: &str = "[\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"contract IEntryPoint\",\n          \"name\": \"anEntryPoint\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"constructor\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"previousAdmin\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"newAdmin\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"AdminChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"beacon\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"BeaconUpgraded\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint8\",\n          \"name\": \"version\",\n          \"type\": \"uint8\"\n        }\n      ],\n      \"name\": \"Initialized\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"contract IEntryPoint\",\n          \"name\": \"entryPoint\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"SimpleAccountInitialized\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"implementation\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"Upgraded\",\n      \"type\": \"event\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"addDeposit\",\n      \"outputs\": [],\n      \"stateMutability\": \"payable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"entryPoint\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract IEntryPoint\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"dest\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"value\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"func\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"execute\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address[]\",\n          \"name\": \"dest\",\n          \"type\": \"address[]\"\n        },\n        {\n          \"internalType\": \"bytes[]\",\n          \"name\": \"func\",\n          \"type\": \"bytes[]\"\n        }\n      ],\n      \"name\": \"executeBatch\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"getDeposit\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"anOwner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"initialize\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"nonce\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"owner\",\n      \"outputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"proxiableUUID\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"newImplementation\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"upgradeTo\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"newImplementation\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"data\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"upgradeToAndCall\",\n      \"outputs\": [],\n      \"stateMutability\": \"payable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"sender\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"nonce\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"initCode\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"callData\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"callGasLimit\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"verificationGasLimit\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"preVerificationGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"maxFeePerGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"maxPriorityFeePerGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"paymasterAndData\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"signature\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct UserOperation\",\n          \"name\": \"userOp\",\n          \"type\": \"tuple\"\n        },\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"userOpHash\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"aggregator\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"missingAccountFunds\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"validateUserOp\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"deadline\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address payable\",\n          \"name\": \"withdrawAddress\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"withdrawDepositTo\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"stateMutability\": \"payable\",\n      \"type\": \"receive\"\n    }\n  ]";
    ///The parsed JSON ABI of the contract.
    pub static SIMPLEACCOUNT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct SimpleAccount<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SimpleAccount<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SimpleAccount<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SimpleAccount<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SimpleAccount<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(SimpleAccount))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SimpleAccount<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SIMPLEACCOUNT_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `addDeposit` (0x4a58db19) function
        pub fn add_deposit(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 88, 219, 25], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `entryPoint` (0xb0d691fe) function
        pub fn entry_point(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([176, 214, 145, 254], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0xb61d27f6) function
        pub fn execute(
            &self,
            dest: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            func: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 29, 39, 246], (dest, value, func))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeBatch` (0x18dfb3c7) function
        pub fn execute_batch(
            &self,
            dest: ::std::vec::Vec<::ethers::core::types::Address>,
            func: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 223, 179, 199], (dest, func))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDeposit` (0xc399ec88) function
        pub fn get_deposit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([195, 153, 236, 136], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xc4d66de8) function
        pub fn initialize(
            &self,
            an_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], an_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonce` (0xaffed0e0) function
        pub fn nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 254, 208, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeTo` (0x3659cfe6) function
        pub fn upgrade_to(
            &self,
            new_implementation: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 89, 207, 230], new_implementation)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeToAndCall` (0x4f1ef286) function
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateUserOp` (0x0825d1fc) function
        pub fn validate_user_op(
            &self,
            user_op: UserOperation,
            user_op_hash: [u8; 32],
            aggregator: ::ethers::core::types::Address,
            missing_account_funds: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [8, 37, 209, 252],
                    (user_op, user_op_hash, aggregator, missing_account_funds),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawDepositTo` (0x4d44560d) function
        pub fn withdraw_deposit_to(
            &self,
            withdraw_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 68, 86, 13], (withdraw_address, amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AdminChanged` event
        pub fn admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AdminChangedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `BeaconUpgraded` event
        pub fn beacon_upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BeaconUpgradedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SimpleAccountInitialized` event
        pub fn simple_account_initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SimpleAccountInitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpgradedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SimpleAccountEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for SimpleAccount<M>
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
    #[ethevent(name = "AdminChanged", abi = "AdminChanged(address,address)")]
    pub struct AdminChangedFilter {
        pub previous_admin: ::ethers::core::types::Address,
        pub new_admin: ::ethers::core::types::Address,
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
    #[ethevent(name = "BeaconUpgraded", abi = "BeaconUpgraded(address)")]
    pub struct BeaconUpgradedFilter {
        #[ethevent(indexed)]
        pub beacon: ::ethers::core::types::Address,
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
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
    #[ethevent(
        name = "SimpleAccountInitialized",
        abi = "SimpleAccountInitialized(address,address)"
    )]
    pub struct SimpleAccountInitializedFilter {
        #[ethevent(indexed)]
        pub entry_point: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SimpleAccountEvents {
        AdminChangedFilter(AdminChangedFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        InitializedFilter(InitializedFilter),
        SimpleAccountInitializedFilter(SimpleAccountInitializedFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for SimpleAccountEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(SimpleAccountEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(SimpleAccountEvents::BeaconUpgradedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(SimpleAccountEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = SimpleAccountInitializedFilter::decode_log(log) {
                return Ok(SimpleAccountEvents::SimpleAccountInitializedFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(SimpleAccountEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SimpleAccountEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BeaconUpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimpleAccountInitializedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminChangedFilter> for SimpleAccountEvents {
        fn from(value: AdminChangedFilter) -> Self {
            Self::AdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<BeaconUpgradedFilter> for SimpleAccountEvents {
        fn from(value: BeaconUpgradedFilter) -> Self {
            Self::BeaconUpgradedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for SimpleAccountEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<SimpleAccountInitializedFilter> for SimpleAccountEvents {
        fn from(value: SimpleAccountInitializedFilter) -> Self {
            Self::SimpleAccountInitializedFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for SimpleAccountEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `addDeposit` function with signature `addDeposit()` and selector `0x4a58db19`
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
    #[ethcall(name = "addDeposit", abi = "addDeposit()")]
    pub struct AddDepositCall;
    ///Container type for all input parameters for the `entryPoint` function with signature `entryPoint()` and selector `0xb0d691fe`
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
    #[ethcall(name = "entryPoint", abi = "entryPoint()")]
    pub struct EntryPointCall;
    ///Container type for all input parameters for the `execute` function with signature `execute(address,uint256,bytes)` and selector `0xb61d27f6`
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
    #[ethcall(name = "execute", abi = "execute(address,uint256,bytes)")]
    pub struct ExecuteCall {
        pub dest: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub func: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `executeBatch` function with signature `executeBatch(address[],bytes[])` and selector `0x18dfb3c7`
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
    #[ethcall(name = "executeBatch", abi = "executeBatch(address[],bytes[])")]
    pub struct ExecuteBatchCall {
        pub dest: ::std::vec::Vec<::ethers::core::types::Address>,
        pub func: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `getDeposit` function with signature `getDeposit()` and selector `0xc399ec88`
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
    #[ethcall(name = "getDeposit", abi = "getDeposit()")]
    pub struct GetDepositCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address)` and selector `0xc4d66de8`
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
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub an_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `nonce` function with signature `nonce()` and selector `0xaffed0e0`
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
    #[ethcall(name = "nonce", abi = "nonce()")]
    pub struct NonceCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
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
    #[ethcall(name = "proxiableUUID", abi = "proxiableUUID()")]
    pub struct ProxiableUUIDCall;
    ///Container type for all input parameters for the `upgradeTo` function with signature `upgradeTo(address)` and selector `0x3659cfe6`
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
    #[ethcall(name = "upgradeTo", abi = "upgradeTo(address)")]
    pub struct UpgradeToCall {
        pub new_implementation: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `upgradeToAndCall` function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`
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
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `validateUserOp` function with signature `validateUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,address,uint256)` and selector `0x0825d1fc`
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
        abi = "validateUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,address,uint256)"
    )]
    pub struct ValidateUserOpCall {
        pub user_op: UserOperation,
        pub user_op_hash: [u8; 32],
        pub aggregator: ::ethers::core::types::Address,
        pub missing_account_funds: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawDepositTo` function with signature `withdrawDepositTo(address,uint256)` and selector `0x4d44560d`
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
    #[ethcall(name = "withdrawDepositTo", abi = "withdrawDepositTo(address,uint256)")]
    pub struct WithdrawDepositToCall {
        pub withdraw_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SimpleAccountCalls {
        AddDeposit(AddDepositCall),
        EntryPoint(EntryPointCall),
        Execute(ExecuteCall),
        ExecuteBatch(ExecuteBatchCall),
        GetDeposit(GetDepositCall),
        Initialize(InitializeCall),
        Nonce(NonceCall),
        Owner(OwnerCall),
        ProxiableUUID(ProxiableUUIDCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        ValidateUserOp(ValidateUserOpCall),
        WithdrawDepositTo(WithdrawDepositToCall),
    }
    impl ::ethers::core::abi::AbiDecode for SimpleAccountCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddDepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddDeposit(decoded));
            }
            if let Ok(decoded) = <EntryPointCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EntryPoint(decoded));
            }
            if let Ok(decoded) = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded) = <ExecuteBatchCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteBatch(decoded));
            }
            if let Ok(decoded) = <GetDepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetDeposit(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <NonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonce(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProxiableUUID(decoded));
            }
            if let Ok(decoded) = <UpgradeToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpgradeTo(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) =
                <ValidateUserOpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateUserOp(decoded));
            }
            if let Ok(decoded) =
                <WithdrawDepositToCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawDepositTo(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SimpleAccountCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddDeposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EntryPoint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteBatch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDeposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProxiableUUID(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeToAndCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateUserOp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawDepositTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SimpleAccountCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::EntryPoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteBatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateUserOp(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawDepositTo(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddDepositCall> for SimpleAccountCalls {
        fn from(value: AddDepositCall) -> Self {
            Self::AddDeposit(value)
        }
    }
    impl ::core::convert::From<EntryPointCall> for SimpleAccountCalls {
        fn from(value: EntryPointCall) -> Self {
            Self::EntryPoint(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for SimpleAccountCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ExecuteBatchCall> for SimpleAccountCalls {
        fn from(value: ExecuteBatchCall) -> Self {
            Self::ExecuteBatch(value)
        }
    }
    impl ::core::convert::From<GetDepositCall> for SimpleAccountCalls {
        fn from(value: GetDepositCall) -> Self {
            Self::GetDeposit(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for SimpleAccountCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<NonceCall> for SimpleAccountCalls {
        fn from(value: NonceCall) -> Self {
            Self::Nonce(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for SimpleAccountCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for SimpleAccountCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<UpgradeToCall> for SimpleAccountCalls {
        fn from(value: UpgradeToCall) -> Self {
            Self::UpgradeTo(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for SimpleAccountCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    impl ::core::convert::From<ValidateUserOpCall> for SimpleAccountCalls {
        fn from(value: ValidateUserOpCall) -> Self {
            Self::ValidateUserOp(value)
        }
    }
    impl ::core::convert::From<WithdrawDepositToCall> for SimpleAccountCalls {
        fn from(value: WithdrawDepositToCall) -> Self {
            Self::WithdrawDepositTo(value)
        }
    }
    ///Container type for all return fields from the `entryPoint` function with signature `entryPoint()` and selector `0xb0d691fe`
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
    pub struct EntryPointReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getDeposit` function with signature `getDeposit()` and selector `0xc399ec88`
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
    pub struct GetDepositReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `nonce` function with signature `nonce()` and selector `0xaffed0e0`
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
    pub struct NonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
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
    pub struct ProxiableUUIDReturn(pub [u8; 32]);
    ///Container type for all return fields from the `validateUserOp` function with signature `validateUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,address,uint256)` and selector `0x0825d1fc`
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
        pub deadline: ::ethers::core::types::U256,
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
