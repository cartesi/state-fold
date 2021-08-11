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

#![warn(unused_extern_crates)]
use state_fold_examples::fold::test_array_contract_delegate::ArrayContractFoldDelegate;
use state_fold_examples::fold::test_struct_contract_delegate::StructContractFoldDelegate;
use state_fold_examples::setup_test_contract;
use state_fold_examples::types::Examples;
use state_server_grpc::{serve_delegate_manager, wait_for_signal};

use tokio::sync::oneshot;

macro_rules! start_serving {
    ($server_file: ident, $shutdown_rx: ident, $contract_fold: ident) => {
        serve_delegate_manager(
            "[::1]:50051",
            state_fold_examples::$server_file::ContractDelegateManager {
                fold: $contract_fold,
            },
            $shutdown_rx,
        )
        .await
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    let _ = tokio::spawn(wait_for_signal(shutdown_tx));

    let example = state_fold_examples::handle_env_args()?;

    match example {
        Examples::Array(contract_name, contract_path) => {
            let contract_fold = setup_test_contract!(
                ArrayContractFoldDelegate,
                contract_name,
                contract_path
            );
            start_serving!(delegate_array_server, shutdown_rx, contract_fold)
        }
        Examples::Struct(contract_name, contract_path) => {
            let contract_fold = setup_test_contract!(
                StructContractFoldDelegate,
                contract_name,
                contract_path
            );
            start_serving!(delegate_struct_server, shutdown_rx, contract_fold)
        }
    }
}
