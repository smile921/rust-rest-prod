use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Database {
    pub url: Option<String>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Logging {
    pub log_level: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[allow(unused)]
pub struct Tracing {
    pub otlp_endpoint: Option<String>,
}
#[derive(Debug, Default, Deserialize, Clone)]
pub struct Settings {
    #[serde(default)]
    pub database: Database,
    #[serde(default)]
    pub logging: Logging,
    #[serde(default)]
    pub config: ConfigInfo,
    #[serde(default)]
    pub token_secret: String,
    #[serde(default)]
    pub token_timeout_seconds: i64,
    #[serde(default)]
    pub tracing: Tracing,
    #[serde(default)]
    pub assets_dir: String,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct ConfigInfo {
    pub location: Option<String>,
    pub env_prefix: Option<String>,
}

impl Settings {
    pub fn new(location: &str, env_prefix: &str) -> anyhow::Result<Self> {
        let s = Config::builder()
            .add_source(File::with_name(location))
            .add_source(
                Environment::with_prefix(env_prefix)
                    .separator("__")
                    .prefix_separator("__"),
            )
            .set_override("config.location", location)?
            .set_override("config.env_prefix", env_prefix)?
            .build()?;

        let settings = s.try_deserialize()?;
        Ok(settings)
    }
}
