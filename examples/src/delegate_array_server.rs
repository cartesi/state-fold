// Copyright (C) 2021 Cartesi Pte. Ltd.

// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.

// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Note: This component currently has dependencies that are licensed under the
// GNU GPL, version 3, and so you should treat this component as a whole as
// being under the GPL version 3. But all Cartesi-written code in this component
// is licensed under the Apache License, version 2, or a compatible permissive
// license, and can be used independently under the Apache v2 license. After
// this component is rewritten, the entire component will be released under the
// Apache v2 license.

use state_fold::{Access, StateFold};
use state_server_grpc::state_server::delegate_manager_server::DelegateManager;
use state_server_grpc::state_server::{GetStateRequest, GetStateResponse};

use ethers::providers::{Http, Provider};
use tonic::{Code, Request, Response, Status};

pub struct ContractDelegateManager {
    pub fold: StateFold<
        crate::test_array_contract_delegate::ArrayContractFoldDelegate,
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
