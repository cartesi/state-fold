pub mod delegate_server;
pub mod fold;
pub mod test_contract_lib;

use block_subscriber::{BlockSubscriber, BlockSubscriberHandle};
use fold::test_contract_delegate;
use middleware_factory::WsProviderFactory;
use state_fold::{Access, StateFold};

use ethers::contract::{Contract, ContractFactory};
use ethers::core::utils::Solc;
use ethers::providers::{Http, Middleware, Provider, Ws};
use ethers::types::U64;

use std::convert::TryFrom;
use std::sync::Arc;
use std::time::Duration;

static WS_URL: &'static str = "ws://localhost:8546";
static HTTP_URL: &'static str = "http://localhost:8545";

pub async fn deploy_test_contract<M: Middleware>(
    client: Arc<M>,
) -> Contract<M> {
    let contract_name = "TestContract";
    let path = "./common/contract/TestContract.sol";
    let contracts = Solc::new(&path).build().unwrap();
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

pub async fn setup_test_contract_delegate() -> (
    Arc<Access<Provider<Http>>>,
    StateFold<
        test_contract_delegate::ContractFoldDelegate,
        Access<Provider<Http>>,
    >,
) {
    // construct StateFold
    let provider = Provider::<Http>::try_from(HTTP_URL).unwrap();
    let deployer = provider.get_accounts().await.unwrap()[0];
    let client = provider.clone().with_sender(deployer);

    let test_contract = deploy_test_contract(Arc::new(client)).await;
    let contract_address = test_contract.address();

    let access = Access::new(Arc::new(provider), U64::from(0), vec![], 4);
    let arc_access = Arc::new(access);

    let contract_delegate =
        test_contract_delegate::ContractFoldDelegate::new(contract_address);
    let contract_fold =
        StateFold::new(contract_delegate, Arc::clone(&arc_access), 0);

    (arc_access, contract_fold)
}
