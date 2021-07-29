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

use block_subscriber::block_subscriber::NewBlockSubscriber;

#[tokio::main]
async fn main() -> std::result::Result<(), ()> {
    let (block_subscriber, _handle) =
        state_fold_examples::setup_block_subscriber().await;
    let (_, contract_fold) =
        state_fold_examples::setup_test_contract_delegate().await;
    let mut subscription = block_subscriber.subscribe().await.unwrap();

    loop {
        let current_block = subscription.recv().await.unwrap();
        let contract_state = contract_fold
            .get_state_for_block(&(), current_block.hash)
            .await
            .map_err(|e| {
                println!("error getting state for contract fold: {}", e);
                ()
            })?
            .state;

        println!("Current block: {}", current_block.number);
        println!("{:?}", contract_state.ctx);
    }
}
