pub use safe_4337_module::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod safe_4337_module {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"entryPoint\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"SUPPORTED_ENTRYPOINT\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"domainSeparator\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contract Safe\",\"name\":\"safe\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\"}],\"name\":\"encodeMessageDataForSafe\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"},{\"internalType\":\"uint8\",\"name\":\"operation\",\"type\":\"uint8\"}],\"name\":\"executeUserOp\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"},{\"internalType\":\"uint8\",\"name\":\"operation\",\"type\":\"uint8\"}],\"name\":\"executeUserOpWithErrorString\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\"}],\"name\":\"getMessageHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contract Safe\",\"name\":\"safe\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\"}],\"name\":\"getMessageHashForSafe\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getModules\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"initCode\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\"},{\"internalType\":\"uint256\",\"name\":\"callGasLimit\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"verificationGasLimit\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"preVerificationGas\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"maxFeePerGas\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"maxPriorityFeePerGas\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"paymasterAndData\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"}],\"internalType\":\"struct UserOperation\",\"name\":\"userOp\",\"type\":\"tuple\"}],\"name\":\"getOperationHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"operationHash\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_dataHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"_signature\",\"type\":\"bytes\"}],\"name\":\"isValidSignature\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"_signature\",\"type\":\"bytes\"}],\"name\":\"isValidSignature\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\"},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"name\":\"onERC1155BatchReceived\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"name\":\"onERC1155Received\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"name\":\"onERC721Received\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"targetContract\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"calldataPayload\",\"type\":\"bytes\"}],\"name\":\"simulate\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"response\",\"type\":\"bytes\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\"}],\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"name\":\"tokensReceived\",\"outputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"initCode\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\"},{\"internalType\":\"uint256\",\"name\":\"callGasLimit\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"verificationGasLimit\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"preVerificationGas\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"maxFeePerGas\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"maxPriorityFeePerGas\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"paymasterAndData\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"}],\"internalType\":\"struct UserOperation\",\"name\":\"userOp\",\"type\":\"tuple\"},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"missingAccountFunds\",\"type\":\"uint256\"}],\"name\":\"validateUserOp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"validationData\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]";
    ///The parsed JSON ABI of the contract.
    pub static SAFE4337MODULE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct Safe4337Module<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Safe4337Module<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Safe4337Module<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Safe4337Module<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Safe4337Module<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Safe4337Module)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Safe4337Module<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SAFE4337MODULE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `SUPPORTED_ENTRYPOINT` (0x137e051e) function
        pub fn supported_entrypoint(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([19, 126, 5, 30], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `domainSeparator` (0xf698da25) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([246, 152, 218, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeMessageDataForSafe` (0x23031640) function
        pub fn encode_message_data_for_safe(
            &self,
            safe: ::ethers::core::types::Address,
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([35, 3, 22, 64], (safe, message))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeUserOp` (0x7bb37428) function
        pub fn execute_user_op(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 179, 116, 40], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeUserOpWithErrorString` (0x541d63c8) function
        pub fn execute_user_op_with_error_string(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([84, 29, 99, 200], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMessageHash` (0x0a1028c4) function
        pub fn get_message_hash(
            &self,
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([10, 16, 40, 196], message)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMessageHashForSafe` (0x6ac24784) function
        pub fn get_message_hash_for_safe(
            &self,
            safe: ::ethers::core::types::Address,
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([106, 194, 71, 132], (safe, message))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getModules` (0xb2494df3) function
        pub fn get_modules(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([178, 73, 77, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperationHash` (0xb25f3776) function
        pub fn get_operation_hash(
            &self,
            user_op: UserOperation,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([178, 95, 55, 118], (user_op,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isValidSignature` (0x1626ba7e) function
        pub fn is_valid_signature(
            &self,
            data_hash: [u8; 32],
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([22, 38, 186, 126], (data_hash, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isValidSignature` (0x20c13b0b) function
        pub fn is_valid_signature_with_data(
            &self,
            data: ::ethers::core::types::Bytes,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([32, 193, 59, 11], (data, signature))
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
        ///Calls the contract's `simulate` (0xbd61951d) function
        pub fn simulate(
            &self,
            target_contract: ::ethers::core::types::Address,
            calldata_payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([189, 97, 149, 29], (target_contract, calldata_payload))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokensReceived` (0x0023de29) function
        pub fn tokens_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::Address,
            p3: ::ethers::core::types::U256,
            p4: ::ethers::core::types::Bytes,
            p5: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 35, 222, 41], (p0, p1, p2, p3, p4, p5))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateUserOp` (0x3a871cdd) function
        pub fn validate_user_op(
            &self,
            user_op: UserOperation,
            p1: [u8; 32],
            missing_account_funds: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([58, 135, 28, 221], (user_op, p1, missing_account_funds))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Safe4337Module<M> {
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
        Hash
    )]
    #[ethcall(name = "SUPPORTED_ENTRYPOINT", abi = "SUPPORTED_ENTRYPOINT()")]
    pub struct SupportedEntrypointCall;
    ///Container type for all input parameters for the `domainSeparator` function with signature `domainSeparator()` and selector `0xf698da25`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "domainSeparator", abi = "domainSeparator()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `encodeMessageDataForSafe` function with signature `encodeMessageDataForSafe(address,bytes)` and selector `0x23031640`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "encodeMessageDataForSafe",
        abi = "encodeMessageDataForSafe(address,bytes)"
    )]
    pub struct EncodeMessageDataForSafeCall {
        pub safe: ::ethers::core::types::Address,
        pub message: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `executeUserOp` function with signature `executeUserOp(address,uint256,bytes,uint8)` and selector `0x7bb37428`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "executeUserOp",
        abi = "executeUserOp(address,uint256,bytes,uint8)"
    )]
    pub struct ExecuteUserOpCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
    }
    ///Container type for all input parameters for the `executeUserOpWithErrorString` function with signature `executeUserOpWithErrorString(address,uint256,bytes,uint8)` and selector `0x541d63c8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "executeUserOpWithErrorString",
        abi = "executeUserOpWithErrorString(address,uint256,bytes,uint8)"
    )]
    pub struct ExecuteUserOpWithErrorStringCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
    }
    ///Container type for all input parameters for the `getMessageHash` function with signature `getMessageHash(bytes)` and selector `0x0a1028c4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getMessageHash", abi = "getMessageHash(bytes)")]
    pub struct GetMessageHashCall {
        pub message: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getMessageHashForSafe` function with signature `getMessageHashForSafe(address,bytes)` and selector `0x6ac24784`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getMessageHashForSafe",
        abi = "getMessageHashForSafe(address,bytes)"
    )]
    pub struct GetMessageHashForSafeCall {
        pub safe: ::ethers::core::types::Address,
        pub message: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getModules` function with signature `getModules()` and selector `0xb2494df3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getModules", abi = "getModules()")]
    pub struct GetModulesCall;
    ///Container type for all input parameters for the `getOperationHash` function with signature `getOperationHash((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes))` and selector `0xb25f3776`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getOperationHash",
        abi = "getOperationHash((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes))"
    )]
    pub struct GetOperationHashCall {
        pub user_op: UserOperation,
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
        Hash
    )]
    #[ethcall(name = "isValidSignature", abi = "isValidSignature(bytes32,bytes)")]
    pub struct IsValidSignatureCall {
        pub data_hash: [u8; 32],
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `isValidSignature` function with signature `isValidSignature(bytes,bytes)` and selector `0x20c13b0b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isValidSignature", abi = "isValidSignature(bytes,bytes)")]
    pub struct IsValidSignatureWithDataCall {
        pub data: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
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
        Hash
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
        Hash
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
    ///Container type for all input parameters for the `simulate` function with signature `simulate(address,bytes)` and selector `0xbd61951d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "simulate", abi = "simulate(address,bytes)")]
    pub struct SimulateCall {
        pub target_contract: ::ethers::core::types::Address,
        pub calldata_payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `tokensReceived` function with signature `tokensReceived(address,address,address,uint256,bytes,bytes)` and selector `0x0023de29`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "tokensReceived",
        abi = "tokensReceived(address,address,address,uint256,bytes,bytes)"
    )]
    pub struct TokensReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `validateUserOp` function with signature `validateUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)` and selector `0x3a871cdd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "validateUserOp",
        abi = "validateUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)"
    )]
    pub struct ValidateUserOpCall {
        pub user_op: UserOperation,
        pub p1: [u8; 32],
        pub missing_account_funds: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum Safe4337ModuleCalls {
        SupportedEntrypoint(SupportedEntrypointCall),
        DomainSeparator(DomainSeparatorCall),
        EncodeMessageDataForSafe(EncodeMessageDataForSafeCall),
        ExecuteUserOp(ExecuteUserOpCall),
        ExecuteUserOpWithErrorString(ExecuteUserOpWithErrorStringCall),
        GetMessageHash(GetMessageHashCall),
        GetMessageHashForSafe(GetMessageHashForSafeCall),
        GetModules(GetModulesCall),
        GetOperationHash(GetOperationHashCall),
        IsValidSignature(IsValidSignatureCall),
        IsValidSignatureWithData(IsValidSignatureWithDataCall),
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        OnERC721Received(OnERC721ReceivedCall),
        Simulate(SimulateCall),
        SupportsInterface(SupportsInterfaceCall),
        TokensReceived(TokensReceivedCall),
        ValidateUserOp(ValidateUserOpCall),
    }
    impl ::ethers::core::abi::AbiDecode for Safe4337ModuleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <SupportedEntrypointCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportedEntrypoint(decoded));
            }
            if let Ok(decoded)
                = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded)
                = <EncodeMessageDataForSafeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EncodeMessageDataForSafe(decoded));
            }
            if let Ok(decoded)
                = <ExecuteUserOpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecuteUserOp(decoded));
            }
            if let Ok(decoded)
                = <ExecuteUserOpWithErrorStringCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExecuteUserOpWithErrorString(decoded));
            }
            if let Ok(decoded)
                = <GetMessageHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMessageHash(decoded));
            }
            if let Ok(decoded)
                = <GetMessageHashForSafeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetMessageHashForSafe(decoded));
            }
            if let Ok(decoded)
                = <GetModulesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetModules(decoded));
            }
            if let Ok(decoded)
                = <GetOperationHashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetOperationHash(decoded));
            }
            if let Ok(decoded)
                = <IsValidSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsValidSignature(decoded));
            }
            if let Ok(decoded)
                = <IsValidSignatureWithDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsValidSignatureWithData(decoded));
            }
            if let Ok(decoded)
                = <OnERC1155BatchReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OnERC1155BatchReceived(decoded));
            }
            if let Ok(decoded)
                = <OnERC1155ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OnERC1155Received(decoded));
            }
            if let Ok(decoded)
                = <OnERC721ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OnERC721Received(decoded));
            }
            if let Ok(decoded)
                = <SimulateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Simulate(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded)
                = <TokensReceivedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokensReceived(decoded));
            }
            if let Ok(decoded)
                = <ValidateUserOpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ValidateUserOp(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for Safe4337ModuleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::SupportedEntrypoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeMessageDataForSafe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteUserOp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteUserOpWithErrorString(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMessageHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMessageHashForSafe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetModules(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperationHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsValidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsValidSignatureWithData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155BatchReceived(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC721Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Simulate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokensReceived(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateUserOp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for Safe4337ModuleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SupportedEntrypoint(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::EncodeMessageDataForSafe(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteUserOp(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteUserOpWithErrorString(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMessageHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMessageHashForSafe(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetModules(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperationHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsValidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsValidSignatureWithData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnERC1155BatchReceived(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnERC1155Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC721Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::Simulate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokensReceived(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateUserOp(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<SupportedEntrypointCall> for Safe4337ModuleCalls {
        fn from(value: SupportedEntrypointCall) -> Self {
            Self::SupportedEntrypoint(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for Safe4337ModuleCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<EncodeMessageDataForSafeCall> for Safe4337ModuleCalls {
        fn from(value: EncodeMessageDataForSafeCall) -> Self {
            Self::EncodeMessageDataForSafe(value)
        }
    }
    impl ::core::convert::From<ExecuteUserOpCall> for Safe4337ModuleCalls {
        fn from(value: ExecuteUserOpCall) -> Self {
            Self::ExecuteUserOp(value)
        }
    }
    impl ::core::convert::From<ExecuteUserOpWithErrorStringCall>
    for Safe4337ModuleCalls {
        fn from(value: ExecuteUserOpWithErrorStringCall) -> Self {
            Self::ExecuteUserOpWithErrorString(value)
        }
    }
    impl ::core::convert::From<GetMessageHashCall> for Safe4337ModuleCalls {
        fn from(value: GetMessageHashCall) -> Self {
            Self::GetMessageHash(value)
        }
    }
    impl ::core::convert::From<GetMessageHashForSafeCall> for Safe4337ModuleCalls {
        fn from(value: GetMessageHashForSafeCall) -> Self {
            Self::GetMessageHashForSafe(value)
        }
    }
    impl ::core::convert::From<GetModulesCall> for Safe4337ModuleCalls {
        fn from(value: GetModulesCall) -> Self {
            Self::GetModules(value)
        }
    }
    impl ::core::convert::From<GetOperationHashCall> for Safe4337ModuleCalls {
        fn from(value: GetOperationHashCall) -> Self {
            Self::GetOperationHash(value)
        }
    }
    impl ::core::convert::From<IsValidSignatureCall> for Safe4337ModuleCalls {
        fn from(value: IsValidSignatureCall) -> Self {
            Self::IsValidSignature(value)
        }
    }
    impl ::core::convert::From<IsValidSignatureWithDataCall> for Safe4337ModuleCalls {
        fn from(value: IsValidSignatureWithDataCall) -> Self {
            Self::IsValidSignatureWithData(value)
        }
    }
    impl ::core::convert::From<OnERC1155BatchReceivedCall> for Safe4337ModuleCalls {
        fn from(value: OnERC1155BatchReceivedCall) -> Self {
            Self::OnERC1155BatchReceived(value)
        }
    }
    impl ::core::convert::From<OnERC1155ReceivedCall> for Safe4337ModuleCalls {
        fn from(value: OnERC1155ReceivedCall) -> Self {
            Self::OnERC1155Received(value)
        }
    }
    impl ::core::convert::From<OnERC721ReceivedCall> for Safe4337ModuleCalls {
        fn from(value: OnERC721ReceivedCall) -> Self {
            Self::OnERC721Received(value)
        }
    }
    impl ::core::convert::From<SimulateCall> for Safe4337ModuleCalls {
        fn from(value: SimulateCall) -> Self {
            Self::Simulate(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for Safe4337ModuleCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<TokensReceivedCall> for Safe4337ModuleCalls {
        fn from(value: TokensReceivedCall) -> Self {
            Self::TokensReceived(value)
        }
    }
    impl ::core::convert::From<ValidateUserOpCall> for Safe4337ModuleCalls {
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
        Hash
    )]
    pub struct SupportedEntrypointReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `domainSeparator` function with signature `domainSeparator()` and selector `0xf698da25`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `encodeMessageDataForSafe` function with signature `encodeMessageDataForSafe(address,bytes)` and selector `0x23031640`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EncodeMessageDataForSafeReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getMessageHash` function with signature `getMessageHash(bytes)` and selector `0x0a1028c4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetMessageHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getMessageHashForSafe` function with signature `getMessageHashForSafe(address,bytes)` and selector `0x6ac24784`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetMessageHashForSafeReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getModules` function with signature `getModules()` and selector `0xb2494df3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetModulesReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getOperationHash` function with signature `getOperationHash((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes))` and selector `0xb25f3776`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetOperationHashReturn {
        pub operation_hash: [u8; 32],
    }
    ///Container type for all return fields from the `isValidSignature` function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsValidSignatureReturn(pub [u8; 4]);
    ///Container type for all return fields from the `isValidSignature` function with signature `isValidSignature(bytes,bytes)` and selector `0x20c13b0b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsValidSignatureWithDataReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
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
        Hash
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
        Hash
    )]
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `simulate` function with signature `simulate(address,bytes)` and selector `0xbd61951d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SimulateReturn {
        pub response: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `validateUserOp` function with signature `validateUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)` and selector `0x3a871cdd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
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
        Hash
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
