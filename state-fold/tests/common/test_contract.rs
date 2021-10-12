pub use testcontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod testcontract_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use ethers_contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers_core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers_providers::Middleware;
    #[doc = "TestContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static TESTCONTRACT_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"Address\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"h\",\"type\":\"bytes32\"}],\"name\":\"Bytes32\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"h\",\"type\":\"bytes32\"}],\"name\":\"Complex\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256[2]\",\"name\":\"arr\",\"type\":\"uint256[2]\"}],\"name\":\"FArray\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"IndexedAddress\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"h\",\"type\":\"bytes32\"}],\"name\":\"IndexedBytes32\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\"}],\"name\":\"IndexedInteger\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\"}],\"name\":\"Integer\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"Unit\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256[]\",\"name\":\"arr\",\"type\":\"uint256[]\"}],\"name\":\"VArray\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"emitEvents\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getBytes\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\"},{\"internalType\":\"uint256[3]\",\"name\":\"fa\",\"type\":\"uint256[3]\"}],\"name\":\"getFArray\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256[3]\",\"name\":\"\",\"type\":\"uint256[3]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\"}],\"name\":\"getInteger\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"j\",\"type\":\"uint256\"}],\"name\":\"getMultiple\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getSender\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"i\",\"type\":\"uint32\"}],\"name\":\"getSmallInteger\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"getStruct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"internalType\":\"struct TestContract.Struct\",\"name\":\"\",\"type\":\"tuple\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\"},{\"internalType\":\"uint256[]\",\"name\":\"va\",\"type\":\"uint256[]\"}],\"name\":\"getVArray\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"increment\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]") . expect ("invalid abi")
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
        #[doc = "Calls the contract's `emitEvents` (0x6c8893d3) function"]
        pub fn emit_events(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 136, 147, 211], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBlock` (0x2e97766d) function"]
        pub fn get_block(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256>
        {
            self.0
                .method_hash([46, 151, 118, 109], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBytes` (0x0bcd3b33) function"]
        pub fn get_bytes(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, Vec<u8>> {
            self.0
                .method_hash([11, 205, 59, 51], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFArray` (0xd3119776) function"]
        pub fn get_f_array(
            &self,
            i: ethers_core::types::U256,
            fa: [ethers_core::types::U256; 3usize],
        ) -> ethers_contract::builders::ContractCall<
            M,
            (ethers_core::types::Address, [ethers_core::types::U256; 3]),
        > {
            self.0
                .method_hash([211, 17, 151, 118], (i, fa))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getInteger` (0x37ac2658) function"]
        pub fn get_integer(
            &self,
            i: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256>
        {
            self.0
                .method_hash([55, 172, 38, 88], i)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMultiple` (0x6592d769) function"]
        pub fn get_multiple(
            &self,
            i: ethers_core::types::U256,
            j: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (ethers_core::types::Address, ethers_core::types::U256),
        > {
            self.0
                .method_hash([101, 146, 215, 105], (i, j))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSender` (0x5e01eb5a) function"]
        pub fn get_sender(
            &self,
        ) -> ethers_contract::builders::ContractCall<
            M,
            ethers_core::types::Address,
        > {
            self.0
                .method_hash([94, 1, 235, 90], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSmallInteger` (0x361bc785) function"]
        pub fn get_small_integer(
            &self,
            i: u32,
        ) -> ethers_contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([54, 27, 199, 133], i)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getStruct` (0xcdfa8495) function"]
        pub fn get_struct(
            &self,
            value: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::U256,
                (ethers_core::types::Address, ethers_core::types::U256),
            ),
        > {
            self.0
                .method_hash([205, 250, 132, 149], value)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getVArray` (0xa0ba9e04) function"]
        pub fn get_v_array(
            &self,
            i: ethers_core::types::U256,
            va: ::std::vec::Vec<ethers_core::types::U256>,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (ethers_core::types::Address, Vec<ethers_core::types::U256>),
        > {
            self.0
                .method_hash([160, 186, 158, 4], (i, va))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increment` (0xd09de08a) function"]
        pub fn increment(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 157, 224, 138], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Address` event"]
        pub fn address_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, AddressFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Bytes32` event"]
        pub fn bytes_32_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, Bytes32Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Complex` event"]
        pub fn complex_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ComplexFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FArray` event"]
        pub fn f_array_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, FarrayFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `IndexedAddress` event"]
        pub fn indexed_address_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, IndexedAddressFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `IndexedBytes32` event"]
        pub fn indexed_bytes_32_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, IndexedBytes32Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `IndexedInteger` event"]
        pub fn indexed_integer_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, IndexedIntegerFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Integer` event"]
        pub fn integer_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, IntegerFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Unit` event"]
        pub fn unit_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, UnitFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `VArray` event"]
        pub fn v_array_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, VarrayFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers_contract::builders::Event<M, TestContractEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "Address", abi = "Address(address)")]
    pub struct AddressFilter {
        pub sender: ethers_core::types::Address,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "Bytes32", abi = "Bytes32(bytes32)")]
    pub struct Bytes32Filter {
        pub h: [u8; 32],
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "Complex", abi = "Complex(address,uint256,bytes32)")]
    pub struct ComplexFilter {
        #[ethevent(indexed)]
        pub sender: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub i: ethers_core::types::U256,
        pub h: [u8; 32],
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "FArray", abi = "FArray(address,uint256,uint256[2])")]
    pub struct FarrayFilter {
        #[ethevent(indexed)]
        pub sender: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub i: ethers_core::types::U256,
        pub arr: [ethers_core::types::U256; 2],
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "IndexedAddress", abi = "IndexedAddress(address)")]
    pub struct IndexedAddressFilter {
        #[ethevent(indexed)]
        pub sender: ethers_core::types::Address,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "IndexedBytes32", abi = "IndexedBytes32(bytes32)")]
    pub struct IndexedBytes32Filter {
        #[ethevent(indexed)]
        pub h: [u8; 32],
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "IndexedInteger", abi = "IndexedInteger(uint256)")]
    pub struct IndexedIntegerFilter {
        #[ethevent(indexed)]
        pub i: ethers_core::types::U256,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "Integer", abi = "Integer(uint256)")]
    pub struct IntegerFilter {
        pub i: ethers_core::types::U256,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "Unit", abi = "Unit()")]
    pub struct UnitFilter();
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "VArray", abi = "VArray(address,uint256,uint256[])")]
    pub struct VarrayFilter {
        #[ethevent(indexed)]
        pub sender: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub i: ethers_core::types::U256,
        pub arr: Vec<ethers_core::types::U256>,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TestContractEvents {
        AddressFilter(AddressFilter),
        Bytes32Filter(Bytes32Filter),
        ComplexFilter(ComplexFilter),
        FarrayFilter(FarrayFilter),
        IndexedAddressFilter(IndexedAddressFilter),
        IndexedBytes32Filter(IndexedBytes32Filter),
        IndexedIntegerFilter(IndexedIntegerFilter),
        IntegerFilter(IntegerFilter),
        UnitFilter(UnitFilter),
        VarrayFilter(VarrayFilter),
    }
    impl ethers_core::abi::Tokenizable for TestContractEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddressFilter::from_token(token.clone()) {
                return Ok(TestContractEvents::AddressFilter(decoded));
            }
            if let Ok(decoded) = Bytes32Filter::from_token(token.clone()) {
                return Ok(TestContractEvents::Bytes32Filter(decoded));
            }
            if let Ok(decoded) = ComplexFilter::from_token(token.clone()) {
                return Ok(TestContractEvents::ComplexFilter(decoded));
            }
            if let Ok(decoded) = FarrayFilter::from_token(token.clone()) {
                return Ok(TestContractEvents::FarrayFilter(decoded));
            }
            if let Ok(decoded) = IndexedAddressFilter::from_token(token.clone())
            {
                return Ok(TestContractEvents::IndexedAddressFilter(decoded));
            }
            if let Ok(decoded) = IndexedBytes32Filter::from_token(token.clone())
            {
                return Ok(TestContractEvents::IndexedBytes32Filter(decoded));
            }
            if let Ok(decoded) = IndexedIntegerFilter::from_token(token.clone())
            {
                return Ok(TestContractEvents::IndexedIntegerFilter(decoded));
            }
            if let Ok(decoded) = IntegerFilter::from_token(token.clone()) {
                return Ok(TestContractEvents::IntegerFilter(decoded));
            }
            if let Ok(decoded) = UnitFilter::from_token(token.clone()) {
                return Ok(TestContractEvents::UnitFilter(decoded));
            }
            if let Ok(decoded) = VarrayFilter::from_token(token.clone()) {
                return Ok(TestContractEvents::VarrayFilter(decoded));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                TestContractEvents::AddressFilter(element) => {
                    element.into_token()
                }
                TestContractEvents::Bytes32Filter(element) => {
                    element.into_token()
                }
                TestContractEvents::ComplexFilter(element) => {
                    element.into_token()
                }
                TestContractEvents::FarrayFilter(element) => {
                    element.into_token()
                }
                TestContractEvents::IndexedAddressFilter(element) => {
                    element.into_token()
                }
                TestContractEvents::IndexedBytes32Filter(element) => {
                    element.into_token()
                }
                TestContractEvents::IndexedIntegerFilter(element) => {
                    element.into_token()
                }
                TestContractEvents::IntegerFilter(element) => {
                    element.into_token()
                }
                TestContractEvents::UnitFilter(element) => element.into_token(),
                TestContractEvents::VarrayFilter(element) => {
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
            if let Ok(decoded) = AddressFilter::decode_log(log) {
                return Ok(TestContractEvents::AddressFilter(decoded));
            }
            if let Ok(decoded) = Bytes32Filter::decode_log(log) {
                return Ok(TestContractEvents::Bytes32Filter(decoded));
            }
            if let Ok(decoded) = ComplexFilter::decode_log(log) {
                return Ok(TestContractEvents::ComplexFilter(decoded));
            }
            if let Ok(decoded) = FarrayFilter::decode_log(log) {
                return Ok(TestContractEvents::FarrayFilter(decoded));
            }
            if let Ok(decoded) = IndexedAddressFilter::decode_log(log) {
                return Ok(TestContractEvents::IndexedAddressFilter(decoded));
            }
            if let Ok(decoded) = IndexedBytes32Filter::decode_log(log) {
                return Ok(TestContractEvents::IndexedBytes32Filter(decoded));
            }
            if let Ok(decoded) = IndexedIntegerFilter::decode_log(log) {
                return Ok(TestContractEvents::IndexedIntegerFilter(decoded));
            }
            if let Ok(decoded) = IntegerFilter::decode_log(log) {
                return Ok(TestContractEvents::IntegerFilter(decoded));
            }
            if let Ok(decoded) = UnitFilter::decode_log(log) {
                return Ok(TestContractEvents::UnitFilter(decoded));
            }
            if let Ok(decoded) = VarrayFilter::decode_log(log) {
                return Ok(TestContractEvents::VarrayFilter(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
    #[doc = "`Struct(address,uint256)`"]
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthAbiType,
    )]
    pub struct Struct {
        pub sender: ethers_core::types::Address,
        pub value: ethers_core::types::U256,
    }
}
