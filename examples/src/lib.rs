pub mod delegate_array_server;
pub mod delegate_struct_server;
pub mod fold;
pub mod test_array_contract_lib;
pub mod test_struct_contract_lib;
pub mod types;

use block_subscriber::{BlockSubscriber, BlockSubscriberHandle};
use fold::test_array_contract_delegate;
use fold::test_struct_contract_delegate;
use middleware_factory::WsProviderFactory;
use state_fold::{Access, StateFold};

use ethers::contract::{Contract, ContractFactory};
use ethers::core::utils::Solc;
use ethers::providers::{Http, Middleware, Provider, Ws};
use ethers::types::U64;

use std::convert::TryFrom;
use std::env;
use std::result::Result;
use std::sync::Arc;
use std::time::Duration;

use types::{ExampleContractFoldDelegate, Examples};

static WS_URL: &'static str = "ws://localhost:8546";
static HTTP_URL: &'static str = "http://localhost:8545";

pub async fn deploy_test_contract<M: Middleware>(
    contract_name: &'static str,
    contract_path: &'static str,
    client: Arc<M>,
) -> Contract<M> {
    let contracts = Solc::new(contract_path).build().unwrap();
    let contract = contracts.get(contract_name).unwrap();
    let abi = contract.abi.clone();
    let bytecode = contract.bytecode.clone();

    let factory = ContractFactory::new(abi, bytecode, client);
    factory.deploy(()).unwrap().send().await.unwrap()
}

pub async fn setup_block_subscriber() -> (
    Arc<BlockSubscriber<WsProviderFactory>>,
    BlockSubscriberHandle<Provider<Ws>>,
) {
    // construct BlockSubscriber
    let ws_factory =
        WsProviderFactory::new(WS_URL.to_string(), 0, Duration::from_secs(1))
            .await
            .unwrap();

    let (block_subscriber, handle) = BlockSubscriber::create_and_start(
        ws_factory,
        Duration::from_secs(5),
        5,
        Duration::from_secs(1),
    );

    (block_subscriber, handle)
}

pub async fn setup_test_contract_delegate<CFD: ExampleContractFoldDelegate>(
    contract_name: &'static str,
    contract_path: &'static str,
) -> StateFold<CFD, Access<Provider<Http>>> {
    // construct StateFold
    let provider = Provider::<Http>::try_from(HTTP_URL).unwrap();
    let deployer = provider.get_accounts().await.unwrap()[0];
    let client = provider.clone().with_sender(deployer);

    let test_contract =
        deploy_test_contract(contract_name, contract_path, Arc::new(client))
            .await;
    let contract_address = test_contract.address();

    let access =
        Arc::new(Access::new(Arc::new(provider), U64::from(0), vec![], 4));

    let contract_delegate = ExampleContractFoldDelegate::new(contract_address);
    let contract_fold =
        StateFold::new(contract_delegate, Arc::clone(&access), 0);

    contract_fold
}

#[macro_export]
macro_rules! setup_test_contract {
    ($contract_fold_delegate: tt, $contract_name: ident, $contract_path: ident) => {
        state_fold_examples::setup_test_contract_delegate::<
            $contract_fold_delegate,
        >($contract_name, $contract_path)
        .await
    };
}

pub fn handle_env_args() -> Result<Examples, &'static str> {
    let args: Vec<String> = env::args().collect();

    let example = match args[1].as_str() {
        "Array" => Examples::Array(
            "TestArrayContract",
            "./common/contract/TestArrayContract.sol",
        ),
        "Struct" => Examples::Struct(
            "TestStructContract",
            "./common/contract/TestStructContract.sol",
        ),
        arg => panic!("Unknown example {}", arg),
    };

    Ok(example)
}
