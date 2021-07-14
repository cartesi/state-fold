use offchain_utils::configuration;

use configuration::error as config_error;

use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "sf_config", about = "Configuration for state fold")]
struct SFEnvCLIConfig {
    /// Path to state fold config
    #[structopt(long, env)]
    pub sf_config: Option<String>,
    /// Safety margin for state fold
    #[structopt(long, env)]
    pub sf_safety_margin: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct SFFileConfig {
    pub sf_safety_margin: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct SFConfig {
    pub safety_margin: usize,
}

impl SFConfig {
    pub fn initialize() -> config_error::Result<Self> {
        let env_cli_config = SFEnvCLIConfig::from_args();

        let file_config: SFFileConfig =
            configuration::config::load_config_file(
                env_cli_config.sf_config,
                "state fold",
            )?;

        let safety_margin = env_cli_config
            .sf_safety_margin
            .or(file_config.sf_safety_margin)
            .unwrap_or(10);

        Ok(SFConfig { safety_margin })
    }
}
