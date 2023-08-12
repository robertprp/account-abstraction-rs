pub use zerodev_kernel_factory::*;
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
pub mod zerodev_kernel_factory {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"contract IEntryPoint\",\"name\":\"_entryPoint\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"}],\"name\":\"AccountCreated\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_index\",\"type\":\"uint256\"}],\"name\":\"createAccount\",\"outputs\":[{\"internalType\":\"contract EIP1967Proxy\",\"name\":\"proxy\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_index\",\"type\":\"uint256\"}],\"name\":\"getAccountAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"kernelTemplate\",\"outputs\":[{\"internalType\":\"contract Kernel\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]";
    ///The parsed JSON ABI of the contract.
    pub static KERNELFACTORY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct KernelFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for KernelFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for KernelFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for KernelFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for KernelFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(KernelFactory)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> KernelFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    KERNELFACTORY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `createAccount` (0x5fbfb9cf) function
        pub fn create_account(
            &self,
            owner: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([95, 191, 185, 207], (owner, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAccountAddress` (0x0d253d76) function
        pub fn get_account_address(
            &self,
            owner: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([13, 37, 61, 118], (owner, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `kernelTemplate` (0x037637aa) function
        pub fn kernel_template(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([3, 118, 55, 170], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AccountCreated` event
        pub fn account_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountCreatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccountCreatedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for KernelFactory<M> {
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
        Hash
    )]
    #[ethevent(name = "AccountCreated", abi = "AccountCreated(address,address,uint256)")]
    pub struct AccountCreatedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `createAccount` function with signature `createAccount(address,uint256)` and selector `0x5fbfb9cf`
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
    #[ethcall(name = "createAccount", abi = "createAccount(address,uint256)")]
    pub struct CreateAccountCall {
        pub owner: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getAccountAddress` function with signature `getAccountAddress(address,uint256)` and selector `0x0d253d76`
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
    #[ethcall(name = "getAccountAddress", abi = "getAccountAddress(address,uint256)")]
    pub struct GetAccountAddressCall {
        pub owner: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `kernelTemplate` function with signature `kernelTemplate()` and selector `0x037637aa`
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
    #[ethcall(name = "kernelTemplate", abi = "kernelTemplate()")]
    pub struct KernelTemplateCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum KernelFactoryCalls {
        CreateAccount(CreateAccountCall),
        GetAccountAddress(GetAccountAddressCall),
        KernelTemplate(KernelTemplateCall),
    }
    impl ::ethers::core::abi::AbiDecode for KernelFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CreateAccountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreateAccount(decoded));
            }
            if let Ok(decoded)
                = <GetAccountAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAccountAddress(decoded));
            }
            if let Ok(decoded)
                = <KernelTemplateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::KernelTemplate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for KernelFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CreateAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAccountAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KernelTemplate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for KernelFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CreateAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAccountAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::KernelTemplate(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CreateAccountCall> for KernelFactoryCalls {
        fn from(value: CreateAccountCall) -> Self {
            Self::CreateAccount(value)
        }
    }
    impl ::core::convert::From<GetAccountAddressCall> for KernelFactoryCalls {
        fn from(value: GetAccountAddressCall) -> Self {
            Self::GetAccountAddress(value)
        }
    }
    impl ::core::convert::From<KernelTemplateCall> for KernelFactoryCalls {
        fn from(value: KernelTemplateCall) -> Self {
            Self::KernelTemplate(value)
        }
    }
    ///Container type for all return fields from the `createAccount` function with signature `createAccount(address,uint256)` and selector `0x5fbfb9cf`
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
    pub struct CreateAccountReturn {
        pub proxy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getAccountAddress` function with signature `getAccountAddress(address,uint256)` and selector `0x0d253d76`
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
    pub struct GetAccountAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `kernelTemplate` function with signature `kernelTemplate()` and selector `0x037637aa`
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
    pub struct KernelTemplateReturn(pub ::ethers::core::types::Address);
}
