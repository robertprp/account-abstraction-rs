pub use verifying_paymaster::*;
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
pub mod verifying_paymaster {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"_entryPoint\",\"type\":\"address\",\"internalType\":\"contract IEntryPoint\"},{\"name\":\"_verifyingSigner\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"addStake\",\"inputs\":[{\"name\":\"unstakeDelaySec\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"deposit\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"entryPoint\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contract IEntryPoint\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getDeposit\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getHash\",\"inputs\":[{\"name\":\"userOp\",\"type\":\"tuple\",\"internalType\":\"struct UserOperation\",\"components\":[{\"name\":\"sender\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"nonce\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"initCode\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"callData\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"callGasLimit\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"verificationGasLimit\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"preVerificationGas\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"maxFeePerGas\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"maxPriorityFeePerGas\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"paymasterAndData\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"signature\",\"type\":\"bytes\",\"internalType\":\"bytes\"}]},{\"name\":\"validUntil\",\"type\":\"uint48\",\"internalType\":\"uint48\"},{\"name\":\"validAfter\",\"type\":\"uint48\",\"internalType\":\"uint48\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"parsePaymasterAndData\",\"inputs\":[{\"name\":\"paymasterAndData\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[{\"name\":\"validUntil\",\"type\":\"uint48\",\"internalType\":\"uint48\"},{\"name\":\"validAfter\",\"type\":\"uint48\",\"internalType\":\"uint48\"},{\"name\":\"signature\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"stateMutability\":\"pure\"},{\"type\":\"function\",\"name\":\"postOp\",\"inputs\":[{\"name\":\"mode\",\"type\":\"uint8\",\"internalType\":\"enum IPaymaster.PostOpMode\"},{\"name\":\"context\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"actualGasCost\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"senderNonce\",\"inputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"unlockStake\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"validatePaymasterUserOp\",\"inputs\":[{\"name\":\"userOp\",\"type\":\"tuple\",\"internalType\":\"struct UserOperation\",\"components\":[{\"name\":\"sender\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"nonce\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"initCode\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"callData\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"callGasLimit\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"verificationGasLimit\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"preVerificationGas\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"maxFeePerGas\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"maxPriorityFeePerGas\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"paymasterAndData\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"signature\",\"type\":\"bytes\",\"internalType\":\"bytes\"}]},{\"name\":\"userOpHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"maxCost\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"context\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"validationData\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"verifyingSigner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"withdrawStake\",\"inputs\":[{\"name\":\"withdrawAddress\",\"type\":\"address\",\"internalType\":\"address payable\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"withdrawTo\",\"inputs\":[{\"name\":\"withdrawAddress\",\"type\":\"address\",\"internalType\":\"address payable\"},{\"name\":\"amount\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false}]";
    ///The parsed JSON ABI of the contract.
    pub static VERIFYINGPAYMASTER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct VerifyingPaymaster<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for VerifyingPaymaster<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for VerifyingPaymaster<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for VerifyingPaymaster<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for VerifyingPaymaster<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(VerifyingPaymaster)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> VerifyingPaymaster<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    VERIFYINGPAYMASTER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `addStake` (0x0396cb60) function
        pub fn add_stake(
            &self,
            unstake_delay_sec: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 150, 203, 96], unstake_delay_sec)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0xd0e30db0) function
        pub fn deposit(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 227, 13, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `entryPoint` (0xb0d691fe) function
        pub fn entry_point(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([176, 214, 145, 254], ())
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
        ///Calls the contract's `getHash` (0x94e1fc19) function
        pub fn get_hash(
            &self,
            user_op: UserOperation,
            valid_until: u64,
            valid_after: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([148, 225, 252, 25], (user_op, valid_until, valid_after))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parsePaymasterAndData` (0x94d4ad60) function
        pub fn parse_paymaster_and_data(
            &self,
            paymaster_and_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u64, u64, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([148, 212, 173, 96], paymaster_and_data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `postOp` (0xa9a23409) function
        pub fn post_op(
            &self,
            mode: u8,
            context: ::ethers::core::types::Bytes,
            actual_gas_cost: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 162, 52, 9], (mode, context, actual_gas_cost))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `senderNonce` (0x9c90b443) function
        pub fn sender_nonce(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([156, 144, 180, 67], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unlockStake` (0xbb9fe6bf) function
        pub fn unlock_stake(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 159, 230, 191], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validatePaymasterUserOp` (0xf465c77e) function
        pub fn validate_paymaster_user_op(
            &self,
            user_op: UserOperation,
            user_op_hash: [u8; 32],
            max_cost: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Bytes, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([244, 101, 199, 126], (user_op, user_op_hash, max_cost))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyingSigner` (0x23d9ac9b) function
        pub fn verifying_signer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([35, 217, 172, 155], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawStake` (0xc23a5cea) function
        pub fn withdraw_stake(
            &self,
            withdraw_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 58, 92, 234], withdraw_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawTo` (0x205c2878) function
        pub fn withdraw_to(
            &self,
            withdraw_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 92, 40, 120], (withdraw_address, amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for VerifyingPaymaster<M> {
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
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addStake` function with signature `addStake(uint32)` and selector `0x0396cb60`
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
    #[ethcall(name = "addStake", abi = "addStake(uint32)")]
    pub struct AddStakeCall {
        pub unstake_delay_sec: u32,
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit()` and selector `0xd0e30db0`
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
    #[ethcall(name = "deposit", abi = "deposit()")]
    pub struct DepositCall;
    ///Container type for all input parameters for the `entryPoint` function with signature `entryPoint()` and selector `0xb0d691fe`
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
    #[ethcall(name = "entryPoint", abi = "entryPoint()")]
    pub struct EntryPointCall;
    ///Container type for all input parameters for the `getDeposit` function with signature `getDeposit()` and selector `0xc399ec88`
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
    #[ethcall(name = "getDeposit", abi = "getDeposit()")]
    pub struct GetDepositCall;
    ///Container type for all input parameters for the `getHash` function with signature `getHash((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),uint48,uint48)` and selector `0x94e1fc19`
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
        name = "getHash",
        abi = "getHash((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),uint48,uint48)"
    )]
    pub struct GetHashCall {
        pub user_op: UserOperation,
        pub valid_until: u64,
        pub valid_after: u64,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `parsePaymasterAndData` function with signature `parsePaymasterAndData(bytes)` and selector `0x94d4ad60`
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
    #[ethcall(name = "parsePaymasterAndData", abi = "parsePaymasterAndData(bytes)")]
    pub struct ParsePaymasterAndDataCall {
        pub paymaster_and_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `postOp` function with signature `postOp(uint8,bytes,uint256)` and selector `0xa9a23409`
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
    #[ethcall(name = "postOp", abi = "postOp(uint8,bytes,uint256)")]
    pub struct PostOpCall {
        pub mode: u8,
        pub context: ::ethers::core::types::Bytes,
        pub actual_gas_cost: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `senderNonce` function with signature `senderNonce(address)` and selector `0x9c90b443`
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
    #[ethcall(name = "senderNonce", abi = "senderNonce(address)")]
    pub struct SenderNonceCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unlockStake` function with signature `unlockStake()` and selector `0xbb9fe6bf`
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
    #[ethcall(name = "unlockStake", abi = "unlockStake()")]
    pub struct UnlockStakeCall;
    ///Container type for all input parameters for the `validatePaymasterUserOp` function with signature `validatePaymasterUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)` and selector `0xf465c77e`
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
        name = "validatePaymasterUserOp",
        abi = "validatePaymasterUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)"
    )]
    pub struct ValidatePaymasterUserOpCall {
        pub user_op: UserOperation,
        pub user_op_hash: [u8; 32],
        pub max_cost: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verifyingSigner` function with signature `verifyingSigner()` and selector `0x23d9ac9b`
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
    #[ethcall(name = "verifyingSigner", abi = "verifyingSigner()")]
    pub struct VerifyingSignerCall;
    ///Container type for all input parameters for the `withdrawStake` function with signature `withdrawStake(address)` and selector `0xc23a5cea`
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
    #[ethcall(name = "withdrawStake", abi = "withdrawStake(address)")]
    pub struct WithdrawStakeCall {
        pub withdraw_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawTo` function with signature `withdrawTo(address,uint256)` and selector `0x205c2878`
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
    #[ethcall(name = "withdrawTo", abi = "withdrawTo(address,uint256)")]
    pub struct WithdrawToCall {
        pub withdraw_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VerifyingPaymasterCalls {
        AddStake(AddStakeCall),
        Deposit(DepositCall),
        EntryPoint(EntryPointCall),
        GetDeposit(GetDepositCall),
        GetHash(GetHashCall),
        Owner(OwnerCall),
        ParsePaymasterAndData(ParsePaymasterAndDataCall),
        PostOp(PostOpCall),
        RenounceOwnership(RenounceOwnershipCall),
        SenderNonce(SenderNonceCall),
        TransferOwnership(TransferOwnershipCall),
        UnlockStake(UnlockStakeCall),
        ValidatePaymasterUserOp(ValidatePaymasterUserOpCall),
        VerifyingSigner(VerifyingSignerCall),
        WithdrawStake(WithdrawStakeCall),
        WithdrawTo(WithdrawToCall),
    }
    impl ::ethers::core::abi::AbiDecode for VerifyingPaymasterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AddStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddStake(decoded));
            }
            if let Ok(decoded)
                = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded)
                = <EntryPointCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EntryPoint(decoded));
            }
            if let Ok(decoded)
                = <GetDepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetDeposit(decoded));
            }
            if let Ok(decoded)
                = <GetHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetHash(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <ParsePaymasterAndDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ParsePaymasterAndData(decoded));
            }
            if let Ok(decoded)
                = <PostOpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PostOp(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded)
                = <SenderNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SenderNonce(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <UnlockStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnlockStake(decoded));
            }
            if let Ok(decoded)
                = <ValidatePaymasterUserOpCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ValidatePaymasterUserOp(decoded));
            }
            if let Ok(decoded)
                = <VerifyingSignerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VerifyingSigner(decoded));
            }
            if let Ok(decoded)
                = <WithdrawStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawStake(decoded));
            }
            if let Ok(decoded)
                = <WithdrawToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawTo(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VerifyingPaymasterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EntryPoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ParsePaymasterAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PostOp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SenderNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnlockStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidatePaymasterUserOp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyingSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for VerifyingPaymasterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::EntryPoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParsePaymasterAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PostOp(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SenderNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnlockStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidatePaymasterUserOp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyingSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawTo(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddStakeCall> for VerifyingPaymasterCalls {
        fn from(value: AddStakeCall) -> Self {
            Self::AddStake(value)
        }
    }
    impl ::core::convert::From<DepositCall> for VerifyingPaymasterCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<EntryPointCall> for VerifyingPaymasterCalls {
        fn from(value: EntryPointCall) -> Self {
            Self::EntryPoint(value)
        }
    }
    impl ::core::convert::From<GetDepositCall> for VerifyingPaymasterCalls {
        fn from(value: GetDepositCall) -> Self {
            Self::GetDeposit(value)
        }
    }
    impl ::core::convert::From<GetHashCall> for VerifyingPaymasterCalls {
        fn from(value: GetHashCall) -> Self {
            Self::GetHash(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for VerifyingPaymasterCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ParsePaymasterAndDataCall> for VerifyingPaymasterCalls {
        fn from(value: ParsePaymasterAndDataCall) -> Self {
            Self::ParsePaymasterAndData(value)
        }
    }
    impl ::core::convert::From<PostOpCall> for VerifyingPaymasterCalls {
        fn from(value: PostOpCall) -> Self {
            Self::PostOp(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for VerifyingPaymasterCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SenderNonceCall> for VerifyingPaymasterCalls {
        fn from(value: SenderNonceCall) -> Self {
            Self::SenderNonce(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for VerifyingPaymasterCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnlockStakeCall> for VerifyingPaymasterCalls {
        fn from(value: UnlockStakeCall) -> Self {
            Self::UnlockStake(value)
        }
    }
    impl ::core::convert::From<ValidatePaymasterUserOpCall> for VerifyingPaymasterCalls {
        fn from(value: ValidatePaymasterUserOpCall) -> Self {
            Self::ValidatePaymasterUserOp(value)
        }
    }
    impl ::core::convert::From<VerifyingSignerCall> for VerifyingPaymasterCalls {
        fn from(value: VerifyingSignerCall) -> Self {
            Self::VerifyingSigner(value)
        }
    }
    impl ::core::convert::From<WithdrawStakeCall> for VerifyingPaymasterCalls {
        fn from(value: WithdrawStakeCall) -> Self {
            Self::WithdrawStake(value)
        }
    }
    impl ::core::convert::From<WithdrawToCall> for VerifyingPaymasterCalls {
        fn from(value: WithdrawToCall) -> Self {
            Self::WithdrawTo(value)
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
        Hash
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
        Hash
    )]
    pub struct GetDepositReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getHash` function with signature `getHash((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),uint48,uint48)` and selector `0x94e1fc19`
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
    pub struct GetHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `parsePaymasterAndData` function with signature `parsePaymasterAndData(bytes)` and selector `0x94d4ad60`
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
    pub struct ParsePaymasterAndDataReturn {
        pub valid_until: u64,
        pub valid_after: u64,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `senderNonce` function with signature `senderNonce(address)` and selector `0x9c90b443`
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
    pub struct SenderNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `validatePaymasterUserOp` function with signature `validatePaymasterUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)` and selector `0xf465c77e`
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
    pub struct ValidatePaymasterUserOpReturn {
        pub context: ::ethers::core::types::Bytes,
        pub validation_data: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `verifyingSigner` function with signature `verifyingSigner()` and selector `0x23d9ac9b`
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
    pub struct VerifyingSignerReturn(pub ::ethers::core::types::Address);
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
