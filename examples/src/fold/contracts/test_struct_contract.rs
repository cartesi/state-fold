pub use teststructcontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod teststructcontract_mod {
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use ethers::{
        contract::{
            self as ethers_contract,
            builders::{ContractCall, Event},
            Contract, Lazy,
        },
        core::{
            self as ethers_core,
            abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
            types::*,
        },
        providers::{self as ethers_providers, Middleware},
    };
    #[doc = "TestStructContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static TESTSTRUCTCONTRACT_ABI: ethers_contract::Lazy<
        ethers_core::abi::Abi,
    > = ethers_contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"age\",\"type\":\"uint256\"}],\"name\":\"ModifiedAge\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\"}],\"name\":\"ModifiedName\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"age\",\"type\":\"uint256\"}],\"name\":\"modifyAge\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\"}],\"name\":\"modifyName\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct TestStructContract<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for TestStructContract<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug
        for TestStructContract<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(TestStructContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> TestStructContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers_contract::Contract::new(
                address.into(),
                TESTSTRUCTCONTRACT_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `modifyAge` (0xae75368c) function"]
        pub fn modify_age(
            &self,
            age: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 117, 54, 140], age)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `modifyName` (0x53079c9f) function"]
        pub fn modify_name(
            &self,
            name: String,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 7, 156, 159], name)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ModifiedAge` event"]
        pub fn modified_age_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ModifiedAgeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ModifiedName` event"]
        pub fn modified_name_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ModifiedNameFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers_contract::builders::Event<M, TestStructContractEvents>
        {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "ModifiedAge", abi = "ModifiedAge(uint256)")]
    pub struct ModifiedAgeFilter {
        pub age: ethers_core::types::U256,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "ModifiedName", abi = "ModifiedName(string)")]
    pub struct ModifiedNameFilter {
        pub name: String,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TestStructContractEvents {
        ModifiedAgeFilter(ModifiedAgeFilter),
        ModifiedNameFilter(ModifiedNameFilter),
    }
    impl ethers_core::abi::Tokenizable for TestStructContractEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ModifiedAgeFilter::from_token(token.clone()) {
                return Ok(TestStructContractEvents::ModifiedAgeFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ModifiedNameFilter::from_token(token.clone()) {
                return Ok(TestStructContractEvents::ModifiedNameFilter(
                    decoded,
                ));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                TestStructContractEvents::ModifiedAgeFilter(element) => {
                    element.into_token()
                }
                TestStructContractEvents::ModifiedNameFilter(element) => {
                    element.into_token()
                }
            }
        }
    }
    impl ethers_core::abi::TokenizableItem for TestStructContractEvents {}
    impl ethers_contract::EthLogDecode for TestStructContractEvents {
        fn decode_log(
            log: &ethers_core::abi::RawLog,
        ) -> Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ModifiedAgeFilter::decode_log(log) {
                return Ok(TestStructContractEvents::ModifiedAgeFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ModifiedNameFilter::decode_log(log) {
                return Ok(TestStructContractEvents::ModifiedNameFilter(
                    decoded,
                ));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
}
