mod grpc_client;
mod interfaces;

pub mod config;
pub mod error;

pub use grpc_client::GrpcStateFoldClient;
pub use interfaces::BlockServer;
pub use interfaces::StateServer;
