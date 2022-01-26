use offchain_utils::configuration;

use serde::Deserialize;
use snafu::{ResultExt, Snafu};
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt, Clone, Debug)]
#[structopt(name = "bh_config", about = "Configuration for block-history")]
pub struct BHEnvCLIConfig {
    /// Path to block history .toml config
    #[structopt(long, env)]
    pub bh_config_path: Option<String>,

    /// URL of websocket enpoint for block hisory
    #[structopt(long, env)]
    pub bh_ws_endpoint: Option<String>,

    /// Timeout value (secs) for block subscription
    #[structopt(long, env)]
    pub bh_block_timeout: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct BHFileConfig {
    pub ws_endpoint: Option<String>,
    pub block_timeout: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct FileConfig {
    pub block_history: BHFileConfig,
}

#[derive(Clone, Debug)]
pub struct BHConfig {
    pub ws_endpoint: String,
    pub block_timeout: Duration,
}

#[derive(Debug, Snafu)]
pub enum ConfigError {
    #[snafu(display("Configuration missing websocket endpoint url"))]
    MissingWsUrl {},

    #[snafu(display("Error while loading configuration file: {}", source))]
    ConfigFileError { source: configuration::error::Error },
}
pub type Result<T> = std::result::Result<T, ConfigError>;

// default values
const DEFAULT_TIMEOUT: u64 = 60;

impl BHConfig {
    pub fn initialize(env_cli_config: BHEnvCLIConfig) -> Result<Self> {
        let file_config: FileConfig = configuration::config::load_config_file(
            env_cli_config.bh_config_path,
        )
        .context(ConfigFileError)?;

        let ws_endpoint = env_cli_config
            .bh_ws_endpoint
            .or(file_config.block_history.ws_endpoint)
            .ok_or(snafu::NoneError)
            .context(MissingWsUrl)?;

        let block_timeout = Duration::from_secs(
            env_cli_config
                .bh_block_timeout
                .or(file_config.block_history.block_timeout)
                .unwrap_or(DEFAULT_TIMEOUT),
        );

        Ok(BHConfig {
            ws_endpoint,
            block_timeout,
        })
    }
}
