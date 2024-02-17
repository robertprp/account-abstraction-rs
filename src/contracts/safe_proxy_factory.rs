pub use safe_proxy_factory::*;
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
pub mod safe_proxy_factory {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"contract SafeProxy\",\"name\":\"proxy\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"singleton\",\"type\":\"address\"}],\"name\":\"ProxyCreation\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_singleton\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"initializer\",\"type\":\"bytes\"},{\"internalType\":\"uint256\",\"name\":\"saltNonce\",\"type\":\"uint256\"}],\"name\":\"createChainSpecificProxyWithNonce\",\"outputs\":[{\"internalType\":\"contract SafeProxy\",\"name\":\"proxy\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_singleton\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"initializer\",\"type\":\"bytes\"},{\"internalType\":\"uint256\",\"name\":\"saltNonce\",\"type\":\"uint256\"},{\"internalType\":\"contract IProxyCreationCallback\",\"name\":\"callback\",\"type\":\"address\"}],\"name\":\"createProxyWithCallback\",\"outputs\":[{\"internalType\":\"contract SafeProxy\",\"name\":\"proxy\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_singleton\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"initializer\",\"type\":\"bytes\"},{\"internalType\":\"uint256\",\"name\":\"saltNonce\",\"type\":\"uint256\"}],\"name\":\"createProxyWithNonce\",\"outputs\":[{\"internalType\":\"contract SafeProxy\",\"name\":\"proxy\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getChainId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"proxyCreationCode\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"stateMutability\":\"pure\",\"type\":\"function\"}]";
    ///The parsed JSON ABI of the contract.
    pub static SAFEPROXYFACTORY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct SafeProxyFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SafeProxyFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SafeProxyFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SafeProxyFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SafeProxyFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(SafeProxyFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SafeProxyFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SAFEPROXYFACTORY_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `createChainSpecificProxyWithNonce` (0xec9e80bb) function
        pub fn create_chain_specific_proxy_with_nonce(
            &self,
            singleton: ::ethers::core::types::Address,
            initializer: ::ethers::core::types::Bytes,
            salt_nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([236, 158, 128, 187], (singleton, initializer, salt_nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createProxyWithCallback` (0xd18af54d) function
        pub fn create_proxy_with_callback(
            &self,
            singleton: ::ethers::core::types::Address,
            initializer: ::ethers::core::types::Bytes,
            salt_nonce: ::ethers::core::types::U256,
            callback: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash(
                    [209, 138, 245, 77],
                    (singleton, initializer, salt_nonce, callback),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createProxyWithNonce` (0x1688f0b9) function
        pub fn create_proxy_with_nonce(
            &self,
            singleton: ::ethers::core::types::Address,
            initializer: ::ethers::core::types::Bytes,
            salt_nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([22, 136, 240, 185], (singleton, initializer, salt_nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainId` (0x3408e470) function
        pub fn get_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxyCreationCode` (0x53e5d935) function
        pub fn proxy_creation_code(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([83, 229, 217, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ProxyCreation` event
        pub fn proxy_creation_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProxyCreationFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProxyCreationFilter>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for SafeProxyFactory<M>
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
    #[ethevent(name = "ProxyCreation", abi = "ProxyCreation(address,address)")]
    pub struct ProxyCreationFilter {
        #[ethevent(indexed)]
        pub proxy: ::ethers::core::types::Address,
        pub singleton: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `createChainSpecificProxyWithNonce` function with signature `createChainSpecificProxyWithNonce(address,bytes,uint256)` and selector `0xec9e80bb`
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
        name = "createChainSpecificProxyWithNonce",
        abi = "createChainSpecificProxyWithNonce(address,bytes,uint256)"
    )]
    pub struct CreateChainSpecificProxyWithNonceCall {
        pub singleton: ::ethers::core::types::Address,
        pub initializer: ::ethers::core::types::Bytes,
        pub salt_nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `createProxyWithCallback` function with signature `createProxyWithCallback(address,bytes,uint256,address)` and selector `0xd18af54d`
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
        name = "createProxyWithCallback",
        abi = "createProxyWithCallback(address,bytes,uint256,address)"
    )]
    pub struct CreateProxyWithCallbackCall {
        pub singleton: ::ethers::core::types::Address,
        pub initializer: ::ethers::core::types::Bytes,
        pub salt_nonce: ::ethers::core::types::U256,
        pub callback: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `createProxyWithNonce` function with signature `createProxyWithNonce(address,bytes,uint256)` and selector `0x1688f0b9`
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
        name = "createProxyWithNonce",
        abi = "createProxyWithNonce(address,bytes,uint256)"
    )]
    pub struct CreateProxyWithNonceCall {
        pub singleton: ::ethers::core::types::Address,
        pub initializer: ::ethers::core::types::Bytes,
        pub salt_nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
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
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    ///Container type for all input parameters for the `proxyCreationCode` function with signature `proxyCreationCode()` and selector `0x53e5d935`
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
    #[ethcall(name = "proxyCreationCode", abi = "proxyCreationCode()")]
    pub struct ProxyCreationCodeCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SafeProxyFactoryCalls {
        CreateChainSpecificProxyWithNonce(CreateChainSpecificProxyWithNonceCall),
        CreateProxyWithCallback(CreateProxyWithCallbackCall),
        CreateProxyWithNonce(CreateProxyWithNonceCall),
        GetChainId(GetChainIdCall),
        ProxyCreationCode(ProxyCreationCodeCall),
    }
    impl ::ethers::core::abi::AbiDecode for SafeProxyFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <CreateChainSpecificProxyWithNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::CreateChainSpecificProxyWithNonce(decoded));
            }
            if let Ok(decoded) =
                <CreateProxyWithCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreateProxyWithCallback(decoded));
            }
            if let Ok(decoded) =
                <CreateProxyWithNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreateProxyWithNonce(decoded));
            }
            if let Ok(decoded) = <GetChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetChainId(decoded));
            }
            if let Ok(decoded) =
                <ProxyCreationCodeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProxyCreationCode(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SafeProxyFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CreateChainSpecificProxyWithNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateProxyWithCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateProxyWithNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProxyCreationCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SafeProxyFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CreateChainSpecificProxyWithNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateProxyWithCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateProxyWithNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxyCreationCode(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CreateChainSpecificProxyWithNonceCall> for SafeProxyFactoryCalls {
        fn from(value: CreateChainSpecificProxyWithNonceCall) -> Self {
            Self::CreateChainSpecificProxyWithNonce(value)
        }
    }
    impl ::core::convert::From<CreateProxyWithCallbackCall> for SafeProxyFactoryCalls {
        fn from(value: CreateProxyWithCallbackCall) -> Self {
            Self::CreateProxyWithCallback(value)
        }
    }
    impl ::core::convert::From<CreateProxyWithNonceCall> for SafeProxyFactoryCalls {
        fn from(value: CreateProxyWithNonceCall) -> Self {
            Self::CreateProxyWithNonce(value)
        }
    }
    impl ::core::convert::From<GetChainIdCall> for SafeProxyFactoryCalls {
        fn from(value: GetChainIdCall) -> Self {
            Self::GetChainId(value)
        }
    }
    impl ::core::convert::From<ProxyCreationCodeCall> for SafeProxyFactoryCalls {
        fn from(value: ProxyCreationCodeCall) -> Self {
            Self::ProxyCreationCode(value)
        }
    }
    ///Container type for all return fields from the `createChainSpecificProxyWithNonce` function with signature `createChainSpecificProxyWithNonce(address,bytes,uint256)` and selector `0xec9e80bb`
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
    pub struct CreateChainSpecificProxyWithNonceReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createProxyWithCallback` function with signature `createProxyWithCallback(address,bytes,uint256,address)` and selector `0xd18af54d`
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
    pub struct CreateProxyWithCallbackReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `createProxyWithNonce` function with signature `createProxyWithNonce(address,bytes,uint256)` and selector `0x1688f0b9`
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
    pub struct CreateProxyWithNonceReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
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
    pub struct GetChainIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `proxyCreationCode` function with signature `proxyCreationCode()` and selector `0x53e5d935`
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
    pub struct ProxyCreationCodeReturn(pub ::ethers::core::types::Bytes);
}
