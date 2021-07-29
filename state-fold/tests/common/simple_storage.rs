pub use simplestorage_mod::*;
#[allow(clippy::too_many_arguments)]
mod simplestorage_mod {
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
    #[doc = "SimpleStorage was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static SIMPLESTORAGE_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"value\",\"type\":\"string\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"author\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"oldAuthor\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"n\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"string\",\"name\":\"oldValue\",\"type\":\"string\"},{\"indexed\":false,\"internalType\":\"string\",\"name\":\"newValue\",\"type\":\"string\"}],\"name\":\"ValueChanged\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"getValue\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getValues\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"lastSender\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"value\",\"type\":\"string\"}],\"name\":\"setValue\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct SimpleStorage<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for SimpleStorage<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for SimpleStorage<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(SimpleStorage))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> SimpleStorage<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers_contract::Contract::new(
                address.into(),
                SIMPLESTORAGE_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `getValue` (0x20965255) function"]
        pub fn get_value(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([32, 150, 82, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getValues` (0x19eb4a90) function"]
        pub fn get_values(
            &self,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (String, ethers_core::types::Address),
        > {
            self.0
                .method_hash([25, 235, 74, 144], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastSender` (0x256fec88) function"]
        pub fn last_sender(
            &self,
        ) -> ethers_contract::builders::ContractCall<
            M,
            ethers_core::types::Address,
        > {
            self.0
                .method_hash([37, 111, 236, 136], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setValue` (0x93a09352) function"]
        pub fn set_value(
            &self,
            value: String,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 160, 147, 82], value)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ValueChanged` event"]
        pub fn value_changed_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ValueChangedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers_contract::builders::Event<M, ValueChangedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(
        name = "ValueChanged",
        abi = "ValueChanged(address,address,uint256,string,string)"
    )]
    pub struct ValueChangedFilter {
        #[ethevent(indexed)]
        pub author: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub old_author: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub n: ethers_core::types::U256,
        pub old_value: String,
        pub new_value: String,
    }
}
