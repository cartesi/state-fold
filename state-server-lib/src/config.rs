use snafu::{ResultExt, Snafu};
use structopt::StructOpt;

use block_history::config::{BHConfig, BHEnvCLIConfig, Error as BHError};
use state_fold::config::{Error as SFError, SFConfig, SFEnvCLIConfig};

#[derive(StructOpt, Clone, Debug)]
#[structopt(
    name = "sate_server_config",
    about = "Configuration for state-fold state-server"
)]
pub struct StateServerEnvCLIConfig {
    #[structopt(flatten)]
    pub state_fold: SFEnvCLIConfig,

    #[structopt(flatten)]
    pub block_history: BHEnvCLIConfig,

    /// Server address
    #[structopt(long, env = "SS_SERVER_ADDRESS")]
    pub server_address: Option<String>,
}

#[derive(Clone, Debug)]
pub struct StateServerConfig {
    pub state_fold: SFConfig,
    pub block_history: BHConfig,
    pub server_address: std::net::SocketAddr,
}

#[derive(Debug, Snafu)]
pub enum ConfigError {
    #[snafu(display("Error loading state-fold configuration: {}", source))]
    SFError { source: SFError },

    #[snafu(display("Error loading block-history configuration: {}", source))]
    BHError { source: BHError },

    #[snafu(display("Error loading block-history configuration: {}", source))]
    AddressParseError { source: std::net::AddrParseError },
}
pub type Result<T> = std::result::Result<T, ConfigError>;

const DEFAULT_SERVER_ADDRESS: &str = "0.0.0.0:50051";

impl StateServerConfig {
    pub fn initialize_from_args() -> Result<Self> {
        let env_cli_config = StateServerEnvCLIConfig::from_args();
        Self::initialize(env_cli_config)
    }

    pub fn initialize(env_cli_config: StateServerEnvCLIConfig) -> Result<Self> {
        let state_fold = SFConfig::initialize(env_cli_config.state_fold).context(SFSnafu)?;
        let block_history = BHConfig::initialize(env_cli_config.block_history).context(BHSnafu)?;

        let server_address: std::net::SocketAddr = env_cli_config
            .server_address
            .unwrap_or(DEFAULT_SERVER_ADDRESS.to_string())
            .parse()
            .context(AddressParseSnafu)?;

        Ok(Self {
            state_fold,
            block_history,
            server_address,
        })
    }
}