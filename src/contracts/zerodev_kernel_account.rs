pub use zero_dev_kernel_account::*;
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
pub mod zero_dev_kernel_account {
    #[rustfmt::skip]
    const __ABI: &str = "[\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"contract IEntryPoint\",\n          \"name\": \"_entryPoint\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"constructor\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"AlreadyInitialized\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"DisabledMode\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"NotAuthorizedCaller\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"NotEntryPoint\",\n      \"type\": \"error\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"oldValidator\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"newValidator\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"DefaultValidatorChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"bytes4\",\n          \"name\": \"selector\",\n          \"type\": \"bytes4\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"executor\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"validator\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"ExecutionChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"newImplementation\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"Upgraded\",\n      \"type\": \"event\"\n    },\n    {\n      \"stateMutability\": \"payable\",\n      \"type\": \"fallback\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"_disableFlag\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"name\": \"disableMode\",\n      \"outputs\": [],\n      \"stateMutability\": \"payable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"eip712Domain\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes1\",\n          \"name\": \"fields\",\n          \"type\": \"bytes1\"\n        },\n        {\n          \"internalType\": \"string\",\n          \"name\": \"name\",\n          \"type\": \"string\"\n        },\n        {\n          \"internalType\": \"string\",\n          \"name\": \"version\",\n          \"type\": \"string\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"chainId\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"verifyingContract\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"salt\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"uint256[]\",\n          \"name\": \"extensions\",\n          \"type\": \"uint256[]\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"entryPoint\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract IEntryPoint\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"value\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"data\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"internalType\": \"enum Operation\",\n          \"name\": \"operation\",\n          \"type\": \"uint8\"\n        }\n      ],\n      \"name\": \"execute\",\n      \"outputs\": [],\n      \"stateMutability\": \"payable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"getDefaultValidator\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract IKernelValidator\",\n          \"name\": \"validator\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"getDisabledMode\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"disabled\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"_selector\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"name\": \"getExecution\",\n      \"outputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint48\",\n              \"name\": \"validAfter\",\n              \"type\": \"uint48\"\n            },\n            {\n              \"internalType\": \"uint48\",\n              \"name\": \"validUntil\",\n              \"type\": \"uint48\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"executor\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"contract IKernelValidator\",\n              \"name\": \"validator\",\n              \"type\": \"address\"\n            }\n          ],\n          \"internalType\": \"struct ExecutionDetail\",\n          \"name\": \"\",\n          \"type\": \"tuple\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"getLastDisabledTime\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint48\",\n          \"name\": \"\",\n          \"type\": \"uint48\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint192\",\n          \"name\": \"key\",\n          \"type\": \"uint192\"\n        }\n      ],\n      \"name\": \"getNonce\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"getNonce\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"contract IKernelValidator\",\n          \"name\": \"_defaultValidator\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"_data\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"initialize\",\n      \"outputs\": [],\n      \"stateMutability\": \"payable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"hash\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"signature\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"isValidSignature\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"name\",\n      \"outputs\": [\n        {\n          \"internalType\": \"string\",\n          \"name\": \"\",\n          \"type\": \"string\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256[]\",\n          \"name\": \"\",\n          \"type\": \"uint256[]\"\n        },\n        {\n          \"internalType\": \"uint256[]\",\n          \"name\": \"\",\n          \"type\": \"uint256[]\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"onERC1155BatchReceived\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"stateMutability\": \"pure\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"onERC1155Received\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"stateMutability\": \"pure\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"onERC721Received\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"stateMutability\": \"pure\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"contract IKernelValidator\",\n          \"name\": \"_defaultValidator\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"_data\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"setDefaultValidator\",\n      \"outputs\": [],\n      \"stateMutability\": \"payable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"_selector\",\n          \"type\": \"bytes4\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"_executor\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"contract IKernelValidator\",\n          \"name\": \"_validator\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint48\",\n          \"name\": \"_validUntil\",\n          \"type\": \"uint48\"\n        },\n        {\n          \"internalType\": \"uint48\",\n          \"name\": \"_validAfter\",\n          \"type\": \"uint48\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"_enableData\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"setExecution\",\n      \"outputs\": [],\n      \"stateMutability\": \"payable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"_newImplementation\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"upgradeTo\",\n      \"outputs\": [],\n      \"stateMutability\": \"payable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"sender\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"nonce\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"initCode\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"callData\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"callGasLimit\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"verificationGasLimit\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"preVerificationGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"maxFeePerGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"maxPriorityFeePerGas\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"paymasterAndData\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"signature\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct UserOperation\",\n          \"name\": \"userOp\",\n          \"type\": \"tuple\"\n        },\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"userOpHash\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"missingAccountFunds\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"validateUserOp\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"validationData\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"payable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"version\",\n      \"outputs\": [\n        {\n          \"internalType\": \"string\",\n          \"name\": \"\",\n          \"type\": \"string\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"stateMutability\": \"payable\",\n      \"type\": \"receive\"\n    }\n  ]";
    ///The parsed JSON ABI of the contract.
    pub static ZERODEVKERNELACCOUNT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct ZeroDevKernelAccount<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ZeroDevKernelAccount<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ZeroDevKernelAccount<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ZeroDevKernelAccount<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ZeroDevKernelAccount<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ZeroDevKernelAccount))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ZeroDevKernelAccount<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ZERODEVKERNELACCOUNT_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `disableMode` (0xd5416221) function
        pub fn disable_mode(
            &self,
            disable_flag: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 65, 98, 33], disable_flag)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eip712Domain` (0x84b0196e) function
        pub fn eip_712_domain(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                [u8; 1],
                ::std::string::String,
                ::std::string::String,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
                [u8; 32],
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([132, 176, 25, 110], ())
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
        ///Calls the contract's `execute` (0x51945447) function
        pub fn execute(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 148, 84, 71], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDefaultValidator` (0x0b3dc354) function
        pub fn get_default_validator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([11, 61, 195, 84], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDisabledMode` (0x57b75047) function
        pub fn get_disabled_mode(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([87, 183, 80, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getExecution` (0x51166ba0) function
        pub fn get_execution(
            &self,
            selector: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, ExecutionDetail> {
            self.0
                .method_hash([81, 22, 107, 160], selector)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLastDisabledTime` (0x88e7fd06) function
        pub fn get_last_disabled_time(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([136, 231, 253, 6], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNonce` (0x3e1b0812) function
        pub fn get_nonce_with_key(
            &self,
            key: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([62, 27, 8, 18], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNonce` (0xd087d288) function
        pub fn get_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([208, 135, 210, 136], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xd1f57894) function
        pub fn initialize(
            &self,
            default_validator: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 245, 120, 148], (default_validator, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isValidSignature` (0x1626ba7e) function
        pub fn is_valid_signature(
            &self,
            hash: [u8; 32],
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([22, 38, 186, 126], (hash, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155BatchReceived` (0xbc197c81) function
        pub fn on_erc1155_batch_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::std::vec::Vec<::ethers::core::types::U256>,
            p3: ::std::vec::Vec<::ethers::core::types::U256>,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([188, 25, 124, 129], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155Received` (0xf23a6e61) function
        pub fn on_erc1155_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::U256,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([242, 58, 110, 97], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC721Received` (0x150b7a02) function
        pub fn on_erc721_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDefaultValidator` (0x55b14f50) function
        pub fn set_default_validator(
            &self,
            default_validator: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 177, 79, 80], (default_validator, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setExecution` (0x29f8b174) function
        pub fn set_execution(
            &self,
            selector: [u8; 4],
            executor: ::ethers::core::types::Address,
            validator: ::ethers::core::types::Address,
            valid_until: u64,
            valid_after: u64,
            enable_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [41, 248, 177, 116],
                    (
                        selector,
                        executor,
                        validator,
                        valid_until,
                        valid_after,
                        enable_data,
                    ),
                )
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
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DefaultValidatorChanged` event
        pub fn default_validator_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultValidatorChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionChanged` event
        pub fn execution_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ExecutionChangedFilter>
        {
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ZeroDevKernelAccountEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for ZeroDevKernelAccount<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AlreadyInitialized` with signature `AlreadyInitialized()` and selector `0x0dc149f0`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "AlreadyInitialized", abi = "AlreadyInitialized()")]
    pub struct AlreadyInitialized;
    ///Custom Error type `DisabledMode` with signature `DisabledMode()` and selector `0xfc2f51c5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "DisabledMode", abi = "DisabledMode()")]
    pub struct DisabledMode;
    ///Custom Error type `NotAuthorizedCaller` with signature `NotAuthorizedCaller()` and selector `0x7046c88d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotAuthorizedCaller", abi = "NotAuthorizedCaller()")]
    pub struct NotAuthorizedCaller;
    ///Custom Error type `NotEntryPoint` with signature `NotEntryPoint()` and selector `0xd663742a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotEntryPoint", abi = "NotEntryPoint()")]
    pub struct NotEntryPoint;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ZeroDevKernelAccountErrors {
        AlreadyInitialized(AlreadyInitialized),
        DisabledMode(DisabledMode),
        NotAuthorizedCaller(NotAuthorizedCaller),
        NotEntryPoint(NotEntryPoint),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ZeroDevKernelAccountErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <AlreadyInitialized as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AlreadyInitialized(decoded));
            }
            if let Ok(decoded) = <DisabledMode as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DisabledMode(decoded));
            }
            if let Ok(decoded) =
                <NotAuthorizedCaller as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NotAuthorizedCaller(decoded));
            }
            if let Ok(decoded) = <NotEntryPoint as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotEntryPoint(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ZeroDevKernelAccountErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AlreadyInitialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DisabledMode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotAuthorizedCaller(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotEntryPoint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ZeroDevKernelAccountErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AlreadyInitialized as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <DisabledMode as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NotAuthorizedCaller as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <NotEntryPoint as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ZeroDevKernelAccountErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AlreadyInitialized(element) => ::core::fmt::Display::fmt(element, f),
                Self::DisabledMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotAuthorizedCaller(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotEntryPoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ZeroDevKernelAccountErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AlreadyInitialized> for ZeroDevKernelAccountErrors {
        fn from(value: AlreadyInitialized) -> Self {
            Self::AlreadyInitialized(value)
        }
    }
    impl ::core::convert::From<DisabledMode> for ZeroDevKernelAccountErrors {
        fn from(value: DisabledMode) -> Self {
            Self::DisabledMode(value)
        }
    }
    impl ::core::convert::From<NotAuthorizedCaller> for ZeroDevKernelAccountErrors {
        fn from(value: NotAuthorizedCaller) -> Self {
            Self::NotAuthorizedCaller(value)
        }
    }
    impl ::core::convert::From<NotEntryPoint> for ZeroDevKernelAccountErrors {
        fn from(value: NotEntryPoint) -> Self {
            Self::NotEntryPoint(value)
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
    #[ethevent(
        name = "DefaultValidatorChanged",
        abi = "DefaultValidatorChanged(address,address)"
    )]
    pub struct DefaultValidatorChangedFilter {
        #[ethevent(indexed)]
        pub old_validator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_validator: ::ethers::core::types::Address,
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
        name = "ExecutionChanged",
        abi = "ExecutionChanged(bytes4,address,address)"
    )]
    pub struct ExecutionChangedFilter {
        #[ethevent(indexed)]
        pub selector: [u8; 4],
        #[ethevent(indexed)]
        pub executor: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub validator: ::ethers::core::types::Address,
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
        pub new_implementation: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ZeroDevKernelAccountEvents {
        DefaultValidatorChangedFilter(DefaultValidatorChangedFilter),
        ExecutionChangedFilter(ExecutionChangedFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ZeroDevKernelAccountEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DefaultValidatorChangedFilter::decode_log(log) {
                return Ok(ZeroDevKernelAccountEvents::DefaultValidatorChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ExecutionChangedFilter::decode_log(log) {
                return Ok(ZeroDevKernelAccountEvents::ExecutionChangedFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(ZeroDevKernelAccountEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ZeroDevKernelAccountEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultValidatorChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultValidatorChangedFilter> for ZeroDevKernelAccountEvents {
        fn from(value: DefaultValidatorChangedFilter) -> Self {
            Self::DefaultValidatorChangedFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionChangedFilter> for ZeroDevKernelAccountEvents {
        fn from(value: ExecutionChangedFilter) -> Self {
            Self::ExecutionChangedFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for ZeroDevKernelAccountEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `disableMode` function with signature `disableMode(bytes4)` and selector `0xd5416221`
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
    #[ethcall(name = "disableMode", abi = "disableMode(bytes4)")]
    pub struct DisableModeCall {
        pub disable_flag: [u8; 4],
    }
    ///Container type for all input parameters for the `eip712Domain` function with signature `eip712Domain()` and selector `0x84b0196e`
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
    #[ethcall(name = "eip712Domain", abi = "eip712Domain()")]
    pub struct Eip712DomainCall;
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
    ///Container type for all input parameters for the `execute` function with signature `execute(address,uint256,bytes,uint8)` and selector `0x51945447`
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
    #[ethcall(name = "execute", abi = "execute(address,uint256,bytes,uint8)")]
    pub struct ExecuteCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
    }
    ///Container type for all input parameters for the `getDefaultValidator` function with signature `getDefaultValidator()` and selector `0x0b3dc354`
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
    #[ethcall(name = "getDefaultValidator", abi = "getDefaultValidator()")]
    pub struct GetDefaultValidatorCall;
    ///Container type for all input parameters for the `getDisabledMode` function with signature `getDisabledMode()` and selector `0x57b75047`
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
    #[ethcall(name = "getDisabledMode", abi = "getDisabledMode()")]
    pub struct GetDisabledModeCall;
    ///Container type for all input parameters for the `getExecution` function with signature `getExecution(bytes4)` and selector `0x51166ba0`
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
    #[ethcall(name = "getExecution", abi = "getExecution(bytes4)")]
    pub struct GetExecutionCall {
        pub selector: [u8; 4],
    }
    ///Container type for all input parameters for the `getLastDisabledTime` function with signature `getLastDisabledTime()` and selector `0x88e7fd06`
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
    #[ethcall(name = "getLastDisabledTime", abi = "getLastDisabledTime()")]
    pub struct GetLastDisabledTimeCall;
    ///Container type for all input parameters for the `getNonce` function with signature `getNonce(uint192)` and selector `0x3e1b0812`
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
    #[ethcall(name = "getNonce", abi = "getNonce(uint192)")]
    pub struct GetNonceWithKeyCall {
        pub key: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getNonce` function with signature `getNonce()` and selector `0xd087d288`
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
    #[ethcall(name = "getNonce", abi = "getNonce()")]
    pub struct GetNonceCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,bytes)` and selector `0xd1f57894`
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
    #[ethcall(name = "initialize", abi = "initialize(address,bytes)")]
    pub struct InitializeCall {
        pub default_validator: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `isValidSignature` function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`
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
    #[ethcall(name = "isValidSignature", abi = "isValidSignature(bytes32,bytes)")]
    pub struct IsValidSignatureCall {
        pub hash: [u8; 32],
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
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
        name = "onERC1155BatchReceived",
        abi = "onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct OnERC1155BatchReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
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
        name = "onERC1155Received",
        abi = "onERC1155Received(address,address,uint256,uint256,bytes)"
    )]
    pub struct OnERC1155ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
        name = "onERC721Received",
        abi = "onERC721Received(address,address,uint256,bytes)"
    )]
    pub struct OnERC721ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `setDefaultValidator` function with signature `setDefaultValidator(address,bytes)` and selector `0x55b14f50`
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
        name = "setDefaultValidator",
        abi = "setDefaultValidator(address,bytes)"
    )]
    pub struct SetDefaultValidatorCall {
        pub default_validator: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setExecution` function with signature `setExecution(bytes4,address,address,uint48,uint48,bytes)` and selector `0x29f8b174`
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
        name = "setExecution",
        abi = "setExecution(bytes4,address,address,uint48,uint48,bytes)"
    )]
    pub struct SetExecutionCall {
        pub selector: [u8; 4],
        pub executor: ::ethers::core::types::Address,
        pub validator: ::ethers::core::types::Address,
        pub valid_until: u64,
        pub valid_after: u64,
        pub enable_data: ::ethers::core::types::Bytes,
    }
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
    ///Container type for all input parameters for the `version` function with signature `version()` and selector `0x54fd4d50`
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
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ZeroDevKernelAccountCalls {
        DisableMode(DisableModeCall),
        Eip712Domain(Eip712DomainCall),
        EntryPoint(EntryPointCall),
        Execute(ExecuteCall),
        GetDefaultValidator(GetDefaultValidatorCall),
        GetDisabledMode(GetDisabledModeCall),
        GetExecution(GetExecutionCall),
        GetLastDisabledTime(GetLastDisabledTimeCall),
        GetNonceWithKey(GetNonceWithKeyCall),
        GetNonce(GetNonceCall),
        Initialize(InitializeCall),
        IsValidSignature(IsValidSignatureCall),
        Name(NameCall),
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        OnERC721Received(OnERC721ReceivedCall),
        SetDefaultValidator(SetDefaultValidatorCall),
        SetExecution(SetExecutionCall),
        UpgradeTo(UpgradeToCall),
        ValidateUserOp(ValidateUserOpCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for ZeroDevKernelAccountCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DisableModeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DisableMode(decoded));
            }
            if let Ok(decoded) = <Eip712DomainCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Eip712Domain(decoded));
            }
            if let Ok(decoded) = <EntryPointCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EntryPoint(decoded));
            }
            if let Ok(decoded) = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded) =
                <GetDefaultValidatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetDefaultValidator(decoded));
            }
            if let Ok(decoded) =
                <GetDisabledModeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetDisabledMode(decoded));
            }
            if let Ok(decoded) = <GetExecutionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetExecution(decoded));
            }
            if let Ok(decoded) =
                <GetLastDisabledTimeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetLastDisabledTime(decoded));
            }
            if let Ok(decoded) =
                <GetNonceWithKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetNonceWithKey(decoded));
            }
            if let Ok(decoded) = <GetNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetNonce(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsValidSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsValidSignature(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) =
                <OnERC1155BatchReceivedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnERC1155BatchReceived(decoded));
            }
            if let Ok(decoded) =
                <OnERC1155ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnERC1155Received(decoded));
            }
            if let Ok(decoded) =
                <OnERC721ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnERC721Received(decoded));
            }
            if let Ok(decoded) =
                <SetDefaultValidatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetDefaultValidator(decoded));
            }
            if let Ok(decoded) = <SetExecutionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetExecution(decoded));
            }
            if let Ok(decoded) = <UpgradeToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpgradeTo(decoded));
            }
            if let Ok(decoded) =
                <ValidateUserOpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateUserOp(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ZeroDevKernelAccountCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DisableMode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Eip712Domain(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EntryPoint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDefaultValidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDisabledMode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetExecution(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetLastDisabledTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNonceWithKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsValidSignature(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnERC1155BatchReceived(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155Received(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnERC721Received(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetDefaultValidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetExecution(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateUserOp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ZeroDevKernelAccountCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DisableMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eip712Domain(element) => ::core::fmt::Display::fmt(element, f),
                Self::EntryPoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDefaultValidator(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDisabledMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetExecution(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLastDisabledTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNonceWithKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsValidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC1155BatchReceived(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC1155Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC721Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDefaultValidator(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetExecution(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateUserOp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DisableModeCall> for ZeroDevKernelAccountCalls {
        fn from(value: DisableModeCall) -> Self {
            Self::DisableMode(value)
        }
    }
    impl ::core::convert::From<Eip712DomainCall> for ZeroDevKernelAccountCalls {
        fn from(value: Eip712DomainCall) -> Self {
            Self::Eip712Domain(value)
        }
    }
    impl ::core::convert::From<EntryPointCall> for ZeroDevKernelAccountCalls {
        fn from(value: EntryPointCall) -> Self {
            Self::EntryPoint(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for ZeroDevKernelAccountCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<GetDefaultValidatorCall> for ZeroDevKernelAccountCalls {
        fn from(value: GetDefaultValidatorCall) -> Self {
            Self::GetDefaultValidator(value)
        }
    }
    impl ::core::convert::From<GetDisabledModeCall> for ZeroDevKernelAccountCalls {
        fn from(value: GetDisabledModeCall) -> Self {
            Self::GetDisabledMode(value)
        }
    }
    impl ::core::convert::From<GetExecutionCall> for ZeroDevKernelAccountCalls {
        fn from(value: GetExecutionCall) -> Self {
            Self::GetExecution(value)
        }
    }
    impl ::core::convert::From<GetLastDisabledTimeCall> for ZeroDevKernelAccountCalls {
        fn from(value: GetLastDisabledTimeCall) -> Self {
            Self::GetLastDisabledTime(value)
        }
    }
    impl ::core::convert::From<GetNonceWithKeyCall> for ZeroDevKernelAccountCalls {
        fn from(value: GetNonceWithKeyCall) -> Self {
            Self::GetNonceWithKey(value)
        }
    }
    impl ::core::convert::From<GetNonceCall> for ZeroDevKernelAccountCalls {
        fn from(value: GetNonceCall) -> Self {
            Self::GetNonce(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for ZeroDevKernelAccountCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsValidSignatureCall> for ZeroDevKernelAccountCalls {
        fn from(value: IsValidSignatureCall) -> Self {
            Self::IsValidSignature(value)
        }
    }
    impl ::core::convert::From<NameCall> for ZeroDevKernelAccountCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OnERC1155BatchReceivedCall> for ZeroDevKernelAccountCalls {
        fn from(value: OnERC1155BatchReceivedCall) -> Self {
            Self::OnERC1155BatchReceived(value)
        }
    }
    impl ::core::convert::From<OnERC1155ReceivedCall> for ZeroDevKernelAccountCalls {
        fn from(value: OnERC1155ReceivedCall) -> Self {
            Self::OnERC1155Received(value)
        }
    }
    impl ::core::convert::From<OnERC721ReceivedCall> for ZeroDevKernelAccountCalls {
        fn from(value: OnERC721ReceivedCall) -> Self {
            Self::OnERC721Received(value)
        }
    }
    impl ::core::convert::From<SetDefaultValidatorCall> for ZeroDevKernelAccountCalls {
        fn from(value: SetDefaultValidatorCall) -> Self {
            Self::SetDefaultValidator(value)
        }
    }
    impl ::core::convert::From<SetExecutionCall> for ZeroDevKernelAccountCalls {
        fn from(value: SetExecutionCall) -> Self {
            Self::SetExecution(value)
        }
    }
    impl ::core::convert::From<UpgradeToCall> for ZeroDevKernelAccountCalls {
        fn from(value: UpgradeToCall) -> Self {
            Self::UpgradeTo(value)
        }
    }
    impl ::core::convert::From<ValidateUserOpCall> for ZeroDevKernelAccountCalls {
        fn from(value: ValidateUserOpCall) -> Self {
            Self::ValidateUserOp(value)
        }
    }
    impl ::core::convert::From<VersionCall> for ZeroDevKernelAccountCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `eip712Domain` function with signature `eip712Domain()` and selector `0x84b0196e`
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
    pub struct Eip712DomainReturn {
        pub fields: [u8; 1],
        pub name: ::std::string::String,
        pub version: ::std::string::String,
        pub chain_id: ::ethers::core::types::U256,
        pub verifying_contract: ::ethers::core::types::Address,
        pub salt: [u8; 32],
        pub extensions: ::std::vec::Vec<::ethers::core::types::U256>,
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
    ///Container type for all return fields from the `getDefaultValidator` function with signature `getDefaultValidator()` and selector `0x0b3dc354`
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
    pub struct GetDefaultValidatorReturn {
        pub validator: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getDisabledMode` function with signature `getDisabledMode()` and selector `0x57b75047`
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
    pub struct GetDisabledModeReturn {
        pub disabled: [u8; 4],
    }
    ///Container type for all return fields from the `getExecution` function with signature `getExecution(bytes4)` and selector `0x51166ba0`
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
    pub struct GetExecutionReturn(pub ExecutionDetail);
    ///Container type for all return fields from the `getLastDisabledTime` function with signature `getLastDisabledTime()` and selector `0x88e7fd06`
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
    pub struct GetLastDisabledTimeReturn(pub u64);
    ///Container type for all return fields from the `getNonce` function with signature `getNonce(uint192)` and selector `0x3e1b0812`
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
    pub struct GetNonceWithKeyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getNonce` function with signature `getNonce()` and selector `0xd087d288`
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
    pub struct GetNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `isValidSignature` function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`
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
    pub struct IsValidSignatureReturn(pub [u8; 4]);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
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
    pub struct OnERC1155BatchReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
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
    pub struct OnERC1155ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
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
    ///Container type for all return fields from the `version` function with signature `version()` and selector `0x54fd4d50`
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
    pub struct VersionReturn(pub ::std::string::String);
    ///`ExecutionDetail(uint48,uint48,address,address)`
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
    pub struct ExecutionDetail {
        pub valid_after: u64,
        pub valid_until: u64,
        pub executor: ::ethers::core::types::Address,
        pub validator: ::ethers::core::types::Address,
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
