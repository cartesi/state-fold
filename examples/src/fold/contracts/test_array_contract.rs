pub use testarraycontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod testarraycontract_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "TestArrayContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static TESTARRAYCONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"Modified\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"Popped\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"Pushed\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"modify\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"pop\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"push\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct TestArrayContract<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for TestArrayContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for TestArrayContract<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(TestArrayContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> TestArrayContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                TESTARRAYCONTRACT_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `modify` (0x85d05887) function"]
        pub fn modify(
            &self,
            index: ethers::core::types::U256,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 208, 88, 135], (index, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pop` (0xa4ece52c) function"]
        pub fn pop(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 236, 229, 44], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `push` (0x959ac484) function"]
        pub fn push(
            &self,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([149, 154, 196, 132], value)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Modified` event"]
        pub fn modified_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ModifiedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Popped` event"]
        pub fn popped_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PoppedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Pushed` event"]
        pub fn pushed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PushedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, TestArrayContractEvents>
        {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(name = "Modified", abi = "Modified(uint256,uint256)")]
    pub struct ModifiedFilter {
        #[ethevent(indexed)]
        pub index: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub value: ethers::core::types::U256,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(name = "Popped", abi = "Popped()")]
    pub struct PoppedFilter();
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(name = "Pushed", abi = "Pushed(uint256)")]
    pub struct PushedFilter {
        #[ethevent(indexed)]
        pub value: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TestArrayContractEvents {
        ModifiedFilter(ModifiedFilter),
        PoppedFilter(PoppedFilter),
        PushedFilter(PushedFilter),
    }
    impl ethers::core::abi::Tokenizable for TestArrayContractEvents {
        fn from_token(
            token: ethers::core::abi::Token,
        ) -> Result<Self, ethers::core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ModifiedFilter::from_token(token.clone()) {
                return Ok(TestArrayContractEvents::ModifiedFilter(decoded));
            }
            if let Ok(decoded) = PoppedFilter::from_token(token.clone()) {
                return Ok(TestArrayContractEvents::PoppedFilter(decoded));
            }
            if let Ok(decoded) = PushedFilter::from_token(token.clone()) {
                return Ok(TestArrayContractEvents::PushedFilter(decoded));
            }
            Err(ethers::core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers::core::abi::Token {
            match self {
                TestArrayContractEvents::ModifiedFilter(element) => {
                    element.into_token()
                }
                TestArrayContractEvents::PoppedFilter(element) => {
                    element.into_token()
                }
                TestArrayContractEvents::PushedFilter(element) => {
                    element.into_token()
                }
            }
        }
    }
    impl ethers::core::abi::TokenizableItem for TestArrayContractEvents {}
    impl ethers::contract::EthLogDecode for TestArrayContractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ModifiedFilter::decode_log(log) {
                return Ok(TestArrayContractEvents::ModifiedFilter(decoded));
            }
            if let Ok(decoded) = PoppedFilter::decode_log(log) {
                return Ok(TestArrayContractEvents::PoppedFilter(decoded));
            }
            if let Ok(decoded) = PushedFilter::decode_log(log) {
                return Ok(TestArrayContractEvents::PushedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
}
