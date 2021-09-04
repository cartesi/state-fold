use offchain_utils::configuration;

use configuration::error as config_error;

use offchain_utils::offchain_core::ethers::core::types::U64;
use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt, Clone, Debug)]
#[structopt(name = "sf_config", about = "Configuration for state fold")]
pub struct SFEnvCLIConfig {
    /// Path to state fold .toml config
    #[structopt(long, env)]
    pub sf_config: Option<String>,
    /// Concurrent events fetch for state fold access
    #[structopt(long, env)]
    pub sf_concurrent_events_fetch: Option<usize>,
    /// Genesis block number for state fold access
    #[structopt(long, env)]
    pub sf_genesis_block: Option<U64>,
    /// Query limit error codes for state fold access
    #[structopt(long, env)]
    pub sf_query_limit_error_codes: Option<Vec<i32>>,
    /// Safety margin for state fold
    #[structopt(long, env)]
    pub sf_safety_margin: Option<usize>,
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

// default values
const DEFAULT_CONCURRENT_EVENTS_FETCH: usize = 16;
const DEFAULT_GENESIS_BLOCK: u64 = 0;
const DEFAULT_QUERY_LIMIT_ERROR_CODES: Vec<i32> = vec![];
const DEFAULT_SAFETY_MARGIN: usize = 10;

impl SFConfig {
    pub fn initialize(
        env_cli_config: SFEnvCLIConfig,
    ) -> config_error::Result<Self> {
        let file_config: FileConfig =
            configuration::config::load_config_file(env_cli_config.sf_config)?;

        let concurrent_events_fetch = env_cli_config
            .sf_concurrent_events_fetch
            .or(file_config.state_fold.concurrent_events_fetch)
            .unwrap_or(DEFAULT_CONCURRENT_EVENTS_FETCH);

        let genesis_block = env_cli_config
            .sf_genesis_block
            .or(file_config.state_fold.genesis_block)
            .unwrap_or(U64::from(DEFAULT_GENESIS_BLOCK));

        let query_limit_error_codes = env_cli_config
            .sf_query_limit_error_codes
            .or(file_config.state_fold.query_limit_error_codes)
            .unwrap_or(DEFAULT_QUERY_LIMIT_ERROR_CODES);

        let safety_margin = env_cli_config
            .sf_safety_margin
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
