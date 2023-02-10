use snafu::{ResultExt, Snafu};
use structopt::StructOpt;

#[derive(StructOpt, Clone, Debug)]
#[structopt(name = "sc_config", about = "Configuration for state-client-lib")]
pub struct SCEnvCLIConfig {
    /// URL of state-fold server grpc
    #[structopt(long, env)]
    pub sc_grpc_endpoint: Option<String>,

    /// Default confirmations
    #[structopt(long, env)]
    pub sc_default_confirmations: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct SCConfig {
    pub grpc_endpoint: String,
    pub default_confirmations: usize,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Configuration missing server manager endpoint"))]
    MissingEndpoint {},
}

pub type Result<T> = std::result::Result<T, Error>;

const DEFAULT_DEFAULT_CONFIRMATIONS: usize = 7;

impl SCConfig {
    pub fn initialize_from_args() -> Result<Self> {
        let env_cli_config = SCEnvCLIConfig::from_args();
        Self::initialize(env_cli_config)
    }

    pub fn initialize(env_cli_config: SCEnvCLIConfig) -> Result<Self> {
        let grpc_endpoint = env_cli_config
            .sc_grpc_endpoint
            .ok_or(snafu::NoneError)
            .context(MissingEndpointSnafu)?;

        let default_confirmations = env_cli_config
            .sc_default_confirmations
            .unwrap_or(DEFAULT_DEFAULT_CONFIRMATIONS);

        Ok(SCConfig {
            grpc_endpoint,
            default_confirmations,
        })
    }
}
