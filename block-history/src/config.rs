use serde::Deserialize;
use snafu::{ResultExt, Snafu};
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt, Clone, Debug)]
#[structopt(name = "bh_config", about = "Configuration for block-history")]
pub struct BHEnvCLIConfig {
    /// Path to block history .toml config
    #[structopt(long, env = "BH_CONFIG_PATH")]
    pub config_path: Option<String>,

    /// URL of websocket enpoint for block history
    #[structopt(long, env = "BH_WS_ENDPOINT")]
    pub ws_endpoint: Option<String>,

    /// Timeout value (secs) for block subscription
    #[structopt(long, env = "BH_BLOCK_TIMEOUT")]
    pub block_timeout: Option<u64>,

    /// How far back can we look into the block history from the most recent
    /// block index
    #[structopt(long, env = "BH_MAX_DEPTH")]
    pub max_depth: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct BHFileConfig {
    pub ws_endpoint: Option<String>,
    pub block_timeout: Option<u64>,
    pub max_depth: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct FileConfig {
    pub block_history: BHFileConfig,
}

#[derive(Clone, Debug)]
pub struct BHConfig {
    pub ws_endpoint: String,
    pub block_timeout: Duration,
    pub max_depth: usize,
}

#[derive(Debug, Snafu)]
pub enum ConfigError {
    #[snafu(display("Configuration missing websocket endpoint url"))]
    MissingWsUrl {},

    #[snafu(display("Error while loading configuration file: {}", source))]
    ConfigFileError { source: std::io::Error },
}
pub type Result<T> = std::result::Result<T, ConfigError>;

// default values
const DEFAULT_MAX_DEPTH: usize = 10000;
const DEFAULT_TIMEOUT: u64 = 60;

impl BHConfig {
    pub fn initialize(env_cli_config: BHEnvCLIConfig) -> Result<Self> {
        let file_config: FileConfig = match env_cli_config.config_path {
            None => Default::default(),
            Some(v) => std::fs::read_to_string(&v)
                .map(|v| toml::from_str(&v))
                .context(ConfigFileSnafu)?
                .unwrap_or_default(),
        };

        let ws_endpoint = env_cli_config
            .ws_endpoint
            .or(file_config.block_history.ws_endpoint)
            .ok_or(snafu::NoneError)
            .context(MissingWsUrlSnafu)?;

        let block_timeout = Duration::from_secs(
            env_cli_config
                .block_timeout
                .or(file_config.block_history.block_timeout)
                .unwrap_or(DEFAULT_TIMEOUT),
        );

        let max_depth = env_cli_config
            .max_depth
            .or(file_config.block_history.max_depth)
            .unwrap_or(DEFAULT_MAX_DEPTH);

        Ok(BHConfig {
            ws_endpoint,
            block_timeout,
            max_depth,
        })
    }
}
