// (c) Cartesi and individual authors (see AUTHORS)
// SPDX-License-Identifier: Apache-2.0 (see LICENSE)

use clap::Parser;
use eth_state_fold_types::ethereum_types::U64;

const DEFAULT_CONCURRENT_EVENTS_FETCH: usize = 15;
const DEFAULT_GENESIS_BLOCK: u64 = 0;
const DEFAULT_QUERY_LIMIT_ERROR_CODES: Vec<i32> = vec![];
const DEFAULT_SAFETY_MARGIN: usize = 20;

#[derive(Debug, Clone, Parser)]
#[command(name = "sf_config")]
#[command(about = "Configuration for state fold")]
pub struct SFEnvCLIConfig {
    /// Concurrent events fetch for logs query
    #[arg(long, env, default_value_t = DEFAULT_CONCURRENT_EVENTS_FETCH)]
    pub sf_concurrent_events_fetch: usize,

    /// Genesis block number for state fold access
    #[arg(long, env, default_value_t = DEFAULT_GENESIS_BLOCK)]
    pub sf_genesis_block: u64,

    /// Query limit error codes for state fold access
    #[arg(long, env, default_values_t = DEFAULT_QUERY_LIMIT_ERROR_CODES)]
    pub sf_query_limit_error_codes: Vec<i32>,

    /// Safety margin for state fold
    #[arg(long, env, default_value_t = DEFAULT_SAFETY_MARGIN)]
    pub sf_safety_margin: usize,
}

#[derive(Clone, Debug)]
pub struct SFConfig {
    pub concurrent_events_fetch: usize,
    pub genesis_block: U64,
    pub query_limit_error_codes: Vec<i32>,
    pub safety_margin: usize,
}

impl SFConfig {
    pub fn initialize_from_args() -> Self {
        let env_cli_config = SFEnvCLIConfig::parse();
        Self::initialize(env_cli_config)
    }

    pub fn initialize(env_cli_config: SFEnvCLIConfig) -> Self {
        let concurrent_events_fetch = env_cli_config.sf_concurrent_events_fetch;
        let genesis_block = env_cli_config.sf_genesis_block.into();
        let query_limit_error_codes = env_cli_config.sf_query_limit_error_codes;
        let safety_margin = env_cli_config.sf_safety_margin;

        SFConfig {
            concurrent_events_fetch,
            genesis_block,
            query_limit_error_codes,
            safety_margin,
        }
    }
}
