use state_fold::{Access, StateFold};
use state_server_grpc::state_server::delegate_manager_server::DelegateManager;
use state_server_grpc::state_server::{GetStateRequest, GetStateResponse};

use ethers::providers::{Http, Provider};
use tonic::{Code, Request, Response, Status};

pub struct ContractDelegateManager {
    pub fold: StateFold<
        crate::fold::test_mapping_contract_delegate::MappingContractFoldDelegate,
        Access<Provider<Http>>,
    >,
}

#[tonic::async_trait]
impl DelegateManager for ContractDelegateManager {
    async fn get_state(
        &self,
        request: Request<GetStateRequest>,
    ) -> Result<Response<GetStateResponse>, Status> {
        println!(
            "Got a request from {:?}, initial state: {}",
            request.remote_addr(),
            request.into_inner().json_initial_state
        );

        let contract_state = self
            .fold
            .get_state_for_block(&(), None)
            .await
            .map_err(|e| Status::new(Code::Unavailable, format!("{}", e)))?
            .state;

        let reply = GetStateResponse {
            json_state: format!("state: {:?}", contract_state),
        };

        Ok(Response::new(reply))
    }
}
