use serde::Deserialize;

/// Database connection settings.
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub service: String,
    pub user: String,
    pub password: String,
    #[serde(default = "default_pool_size")]
    pub pool_size: usize,
}

fn default_pool_size() -> usize {
    10
}

/// Top-level application settings.
#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
}

impl Settings {
    /// Load settings from layered sources (lowest → highest priority):
    ///
    /// 1. `config.toml` in the working directory (if present)
    /// 2. Environment variables prefixed with `APP_` and separated by `__`
    ///    e.g. `APP_DATABASE__HOST=myhost`
    pub fn load() -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::File::with_name("config/config").required(false))
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("__"),
            )
            .build()?;

        cfg.try_deserialize()
    }
}
