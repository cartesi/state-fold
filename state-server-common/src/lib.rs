// (c) Cartesi and individual authors (see AUTHORS)
// SPDX-License-Identifier: Apache-2.0 (see LICENSE)

#![allow(clippy::module_inception)]

pub mod conversions;

mod grpc_interface;
pub use grpc_interface::state_fold_server;
