#![allow(clippy::module_inception)]

pub mod conversions;

mod grpc_interface;
pub use grpc_interface::state_fold_server;
