use serde::Deserialize;
use std::path::PathBuf;

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
    pub client_dir: Option<String>,
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
        let manifest_config = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/config");
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let cfg = config::Config::builder()
            .add_source(config::File::from(manifest_config).required(false))
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("__"),
            )
            .build()?;

        let mut settings: Settings = cfg.try_deserialize()?;

        if let Some(client_dir) = &settings.database.client_dir {
            let path = PathBuf::from(client_dir);
            if path.is_relative() {
                settings.database.client_dir = Some(
                    manifest_dir
                        .join(path)
                        .to_string_lossy()
                        .into_owned(),
                );
            }
        }

        Ok(settings)
    }
}
