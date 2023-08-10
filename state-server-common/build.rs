// (c) Cartesi and individual authors (see AUTHORS)
// SPDX-License-Identifier: Apache-2.0 (see LICENSE)

use anyhow::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let grpc_interfaces_dir = PathBuf::from("grpc-interfaces");
    let state_fold_server_dir = grpc_interfaces_dir.join("state-fold-server.proto");

    tonic_build::configure()
        .build_client(cfg!(feature = "client"))
        .build_server(cfg!(feature = "server"))
        .compile(&[&state_fold_server_dir], &[&grpc_interfaces_dir])?;

    println!(
        "cargo:rerun-if-changed={}",
        state_fold_server_dir.to_str().unwrap()
    );
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
