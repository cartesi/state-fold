pub use testmappingcontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod testmappingcontract_mod {
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
    #[doc = "TestMappingContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static TESTMAPPINGCONTRACT_ABI: ethers_contract::Lazy<
        ethers_core::abi::Abi,
    > = ethers_contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"key\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"Modified\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"key\",\"type\":\"uint256\"}],\"name\":\"Removed\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"key\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"modify\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"key\",\"type\":\"uint256\"}],\"name\":\"remove\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct TestMappingContract<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for TestMappingContract<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug
        for TestMappingContract<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(TestMappingContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> TestMappingContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers_contract::Contract::new(
                address.into(),
                TESTMAPPINGCONTRACT_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `modify` (0x85d05887) function"]
        pub fn modify(
            &self,
            key: ethers_core::types::U256,
            value: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 208, 88, 135], (key, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `remove` (0x4cc82215) function"]
        pub fn remove(
            &self,
            key: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 200, 34, 21], key)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Modified` event"]
        pub fn modified_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ModifiedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Removed` event"]
        pub fn removed_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, RemovedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers_contract::builders::Event<M, TestMappingContractEvents>
        {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "Modified", abi = "Modified(uint256,uint256)")]
    pub struct ModifiedFilter {
        pub key: ethers_core::types::U256,
        pub value: ethers_core::types::U256,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "Removed", abi = "Removed(uint256)")]
    pub struct RemovedFilter {
        pub key: ethers_core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TestMappingContractEvents {
        ModifiedFilter(ModifiedFilter),
        RemovedFilter(RemovedFilter),
    }
    impl ethers_core::abi::Tokenizable for TestMappingContractEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ModifiedFilter::from_token(token.clone()) {
                return Ok(TestMappingContractEvents::ModifiedFilter(decoded));
            }
            if let Ok(decoded) = RemovedFilter::from_token(token.clone()) {
                return Ok(TestMappingContractEvents::RemovedFilter(decoded));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                TestMappingContractEvents::ModifiedFilter(element) => {
                    element.into_token()
                }
                TestMappingContractEvents::RemovedFilter(element) => {
                    element.into_token()
                }
            }
        }
    }
    impl ethers_core::abi::TokenizableItem for TestMappingContractEvents {}
    impl ethers_contract::EthLogDecode for TestMappingContractEvents {
        fn decode_log(
            log: &ethers_core::abi::RawLog,
        ) -> Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ModifiedFilter::decode_log(log) {
                return Ok(TestMappingContractEvents::ModifiedFilter(decoded));
            }
            if let Ok(decoded) = RemovedFilter::decode_log(log) {
                return Ok(TestMappingContractEvents::RemovedFilter(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
}
