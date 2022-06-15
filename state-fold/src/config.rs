use state_fold_types::config_utils;

use serde::Deserialize;
use snafu::{ResultExt, Snafu};
use state_fold_types::ethereum_types::U64;
use structopt::StructOpt;

#[derive(StructOpt, Clone, Debug)]
#[structopt(name = "sf_config", about = "Configuration for state fold")]
pub struct SFEnvCLIConfig {
    /// Path to state fold .toml config
    #[structopt(long, env = "SF_CONFIG_PATH")]
    pub config_path: Option<String>,

    /// Concurrent events fetch for logs query
    #[structopt(long, env = "SF_CONCURRENT_EVENTS_FETCH")]
    pub concurrent_events_fetch: Option<usize>,

    /// Genesis block number for state fold access
    #[structopt(long, env = "SF_GENESIS_BLOCK")]
    pub genesis_block: Option<U64>,

    /// Query limit error codes for state fold access
    #[structopt(long, env = "SF_QUERY_LIMIT_ERROR_CODES")]
    pub query_limit_error_codes: Option<Vec<i32>>,

    /// Safety margin for state fold
    #[structopt(long, env = "SF_SAFETY_MARGIN")]
    pub safety_margin: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct SFFileConfig {
    pub concurrent_events_fetch: Option<usize>,
    pub genesis_block: Option<U64>,
    pub query_limit_error_codes: Option<Vec<i32>>,
    pub safety_margin: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct FileConfig {
    pub state_fold: SFFileConfig,
}

#[derive(Clone, Debug)]
pub struct SFConfig {
    pub concurrent_events_fetch: usize,
    pub genesis_block: U64,
    pub query_limit_error_codes: Vec<i32>,
    pub safety_margin: usize,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Error while loading configuration file: {}", source,))]
    ConfigFileError { source: config_utils::Error },
}

pub type Result<T> = std::result::Result<T, Error>;

const DEFAULT_CONCURRENT_EVENTS_FETCH: usize = 16;
const DEFAULT_GENESIS_BLOCK: u64 = 0;
const DEFAULT_QUERY_LIMIT_ERROR_CODES: Vec<i32> = vec![];
const DEFAULT_SAFETY_MARGIN: usize = 20;

impl SFConfig {
    pub fn initialize_from_args() -> Result<Self> {
        let env_cli_config = SFEnvCLIConfig::from_args();
        Self::initialize(env_cli_config)
    }

    pub fn initialize(env_cli_config: SFEnvCLIConfig) -> Result<Self> {
        let file_config: FileConfig =
            config_utils::load_config_file(env_cli_config.config_path).context(ConfigFileSnafu)?;

        let concurrent_events_fetch = env_cli_config
            .concurrent_events_fetch
            .or(file_config.state_fold.concurrent_events_fetch)
            .unwrap_or(DEFAULT_CONCURRENT_EVENTS_FETCH);

        let genesis_block = env_cli_config
            .genesis_block
            .or(file_config.state_fold.genesis_block)
            .unwrap_or(U64::from(DEFAULT_GENESIS_BLOCK));

        let query_limit_error_codes = env_cli_config
            .query_limit_error_codes
            .or(file_config.state_fold.query_limit_error_codes)
            .unwrap_or(DEFAULT_QUERY_LIMIT_ERROR_CODES);

        let safety_margin = env_cli_config
            .safety_margin
            .or(file_config.state_fold.safety_margin)
            .unwrap_or(DEFAULT_SAFETY_MARGIN);

        Ok(SFConfig {
            concurrent_events_fetch,
            genesis_block,
            query_limit_error_codes,
            safety_margin,
        })
    }
}
