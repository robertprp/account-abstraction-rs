pub use simple_account_factory::*;
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
pub mod simple_account_factory {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"contract IEntryPoint\",\"name\":\"_entryPoint\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accountImplementation\",\"outputs\":[{\"internalType\":\"contract SimpleAccount\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createAccount\",\"outputs\":[{\"internalType\":\"contract SimpleAccount\",\"name\":\"ret\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static SIMPLEACCOUNTFACTORY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct SimpleAccountFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SimpleAccountFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SimpleAccountFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SimpleAccountFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SimpleAccountFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(SimpleAccountFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SimpleAccountFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SIMPLEACCOUNTFACTORY_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `accountImplementation` (0x11464fbe) function
        pub fn account_implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([17, 70, 79, 190], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createAccount` (0x5fbfb9cf) function
        pub fn create_account(
            &self,
            owner: ::ethers::core::types::Address,
            salt: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([95, 191, 185, 207], (owner, salt))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAddress` (0x8cb84e18) function
        pub fn get_address(
            &self,
            owner: ::ethers::core::types::Address,
            salt: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([140, 184, 78, 24], (owner, salt))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for SimpleAccountFactory<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `accountImplementation` function with signature `accountImplementation()` and selector `0x11464fbe`
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
    #[ethcall(name = "accountImplementation", abi = "accountImplementation()")]
    pub struct AccountImplementationCall;
    ///Container type for all input parameters for the `createAccount` function with signature `createAccount(address,uint256)` and selector `0x5fbfb9cf`
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
    #[ethcall(name = "createAccount", abi = "createAccount(address,uint256)")]
    pub struct CreateAccountCall {
        pub owner: ::ethers::core::types::Address,
        pub salt: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getAddress` function with signature `getAddress(address,uint256)` and selector `0x8cb84e18`
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
    #[ethcall(name = "getAddress", abi = "getAddress(address,uint256)")]
    pub struct GetAddressCall {
        pub owner: ::ethers::core::types::Address,
        pub salt: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SimpleAccountFactoryCalls {
        AccountImplementation(AccountImplementationCall),
        CreateAccount(CreateAccountCall),
        GetAddress(GetAddressCall),
    }
    impl ::ethers::core::abi::AbiDecode for SimpleAccountFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AccountImplementationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AccountImplementation(decoded));
            }
            if let Ok(decoded) = <CreateAccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreateAccount(decoded));
            }
            if let Ok(decoded) = <GetAddressCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SimpleAccountFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AccountImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateAccount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SimpleAccountFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountImplementation(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAddress(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccountImplementationCall> for SimpleAccountFactoryCalls {
        fn from(value: AccountImplementationCall) -> Self {
            Self::AccountImplementation(value)
        }
    }
    impl ::core::convert::From<CreateAccountCall> for SimpleAccountFactoryCalls {
        fn from(value: CreateAccountCall) -> Self {
            Self::CreateAccount(value)
        }
    }
    impl ::core::convert::From<GetAddressCall> for SimpleAccountFactoryCalls {
        fn from(value: GetAddressCall) -> Self {
            Self::GetAddress(value)
        }
    }
    ///Container type for all return fields from the `accountImplementation` function with signature `accountImplementation()` and selector `0x11464fbe`
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
    pub struct AccountImplementationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `createAccount` function with signature `createAccount(address,uint256)` and selector `0x5fbfb9cf`
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
    pub struct CreateAccountReturn {
        pub ret: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getAddress` function with signature `getAddress(address,uint256)` and selector `0x8cb84e18`
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
    pub struct GetAddressReturn(pub ::ethers::core::types::Address);
}
