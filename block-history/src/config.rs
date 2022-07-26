use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt, Clone, Debug)]
#[structopt(name = "bh_config", about = "Configuration for block-history")]
pub struct BHEnvCLIConfig {
    /// URL of websocket enpoint for block history
    #[structopt(long)]
    pub bh_ws_endpoint: Option<String>,

    /// Timeout value (secs) for block subscription
    #[structopt(long)]
    pub bh_block_timeout: Option<u64>,

    /// How far back can we look into the block history from the most recent
    /// block index
    #[structopt(long)]
    pub bh_max_depth: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct BHConfig {
    pub ws_endpoint: String,
    pub block_timeout: Duration,
    pub max_depth: usize,
}

// default values
const DEFAULT_ENDPOINT: &str = "ws://localhost:8545";
const DEFAULT_MAX_DEPTH: usize = 1000;
const DEFAULT_TIMEOUT: u64 = 60;

impl BHConfig {
    pub fn initialize_from_args() -> Self {
        let env_cli_config = BHEnvCLIConfig::from_args();
        Self::initialize(env_cli_config)
    }

    pub fn initialize(env_cli_config: BHEnvCLIConfig) -> Self {
        let ws_endpoint = env_cli_config
            .bh_ws_endpoint
            .unwrap_or(DEFAULT_ENDPOINT.to_string());

        let block_timeout =
            Duration::from_secs(env_cli_config.bh_block_timeout.unwrap_or(DEFAULT_TIMEOUT));

        let max_depth = env_cli_config.bh_max_depth.unwrap_or(DEFAULT_MAX_DEPTH);

        BHConfig {
            ws_endpoint,
            block_timeout,
            max_depth,
        }
    }
}
