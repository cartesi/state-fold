use offchain_utils::configuration;

use configuration::error as config_error;

use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt, Clone, Debug)]
#[structopt(name = "sf_config", about = "Configuration for state fold")]
pub struct SFEnvCLIConfig {
    /// Path to state fold .toml config
    #[structopt(long, env)]
    pub sf_config: Option<String>,
    /// Safety margin for state fold
    #[structopt(long, env)]
    pub sf_safety_margin: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct SFFileConfig {
    pub safety_margin: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct FileConfig {
    pub state_fold: SFFileConfig,
}

#[derive(Clone, Debug)]
pub struct SFConfig {
    pub safety_margin: usize,
}

// default values
const DEFAULT_SAFETY_MARGIN: usize = 10;

impl SFConfig {
    pub fn initialize(
        env_cli_config: SFEnvCLIConfig,
    ) -> config_error::Result<Self> {
        let file_config: FileConfig =
            configuration::config::load_config_file(env_cli_config.sf_config)?;

        let safety_margin = env_cli_config
            .sf_safety_margin
            .or(file_config.state_fold.safety_margin)
            .unwrap_or(DEFAULT_SAFETY_MARGIN);

        Ok(SFConfig { safety_margin })
    }
}
