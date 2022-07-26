mod grpc_client;
mod interfaces;

pub mod error;
pub mod config;

pub use grpc_client::GrpcStateFoldClient;
pub use interfaces::BlockServer;
pub use interfaces::StateServer;
