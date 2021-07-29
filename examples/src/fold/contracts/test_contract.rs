pub use testcontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod testcontract_mod {
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
    #[doc = "TestContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static TESTCONTRACT_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"Modified\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"Popped\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"Pushed\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"modify\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"pop\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"push\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct TestContract<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for TestContract<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for TestContract<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(TestContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> TestContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers_contract::Contract::new(
                address.into(),
                TESTCONTRACT_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `modify` (0x85d05887) function"]
        pub fn modify(
            &self,
            index: ethers_core::types::U256,
            value: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 208, 88, 135], (index, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pop` (0xa4ece52c) function"]
        pub fn pop(&self) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 236, 229, 44], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `push` (0x959ac484) function"]
        pub fn push(
            &self,
            value: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([149, 154, 196, 132], value)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Modified` event"]
        pub fn modified_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ModifiedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Popped` event"]
        pub fn popped_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, PoppedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Pushed` event"]
        pub fn pushed_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, PushedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers_contract::builders::Event<M, TestContractEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "Modified", abi = "Modified(uint256,uint256)")]
    pub struct ModifiedFilter {
        #[ethevent(indexed)]
        pub index: ethers_core::types::U256,
        #[ethevent(indexed)]
        pub value: ethers_core::types::U256,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "Popped", abi = "Popped()")]
    pub struct PoppedFilter();
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "Pushed", abi = "Pushed(uint256)")]
    pub struct PushedFilter {
        #[ethevent(indexed)]
        pub value: ethers_core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TestContractEvents {
        ModifiedFilter(ModifiedFilter),
        PoppedFilter(PoppedFilter),
        PushedFilter(PushedFilter),
    }
    impl ethers_core::abi::Tokenizable for TestContractEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ModifiedFilter::from_token(token.clone()) {
                return Ok(TestContractEvents::ModifiedFilter(decoded));
            }
            if let Ok(decoded) = PoppedFilter::from_token(token.clone()) {
                return Ok(TestContractEvents::PoppedFilter(decoded));
            }
            if let Ok(decoded) = PushedFilter::from_token(token.clone()) {
                return Ok(TestContractEvents::PushedFilter(decoded));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                TestContractEvents::ModifiedFilter(element) => {
                    element.into_token()
                }
                TestContractEvents::PoppedFilter(element) => {
                    element.into_token()
                }
                TestContractEvents::PushedFilter(element) => {
                    element.into_token()
                }
            }
        }
    }
    impl ethers_core::abi::TokenizableItem for TestContractEvents {}
    impl ethers_contract::EthLogDecode for TestContractEvents {
        fn decode_log(
            log: &ethers_core::abi::RawLog,
        ) -> Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ModifiedFilter::decode_log(log) {
                return Ok(TestContractEvents::ModifiedFilter(decoded));
            }
            if let Ok(decoded) = PoppedFilter::decode_log(log) {
                return Ok(TestContractEvents::PoppedFilter(decoded));
            }
            if let Ok(decoded) = PushedFilter::decode_log(log) {
                return Ok(TestContractEvents::PushedFilter(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
}
