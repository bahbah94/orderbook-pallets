use std::{env, fmt::Display, str::FromStr};
use thiserror::Error;
use url::Url;

/// Deployment environment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Env {
    Dev,
    Prod,
}

impl Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Env::Dev => write!(f, "dev"),
            Env::Prod => write!(f, "prod"),
        }
    }
}

impl Env {
    fn from_str_lossy<S: AsRef<str>>(s: S) -> Self {
        match s.as_ref().to_ascii_lowercase().as_str() {
            "prod" | "production" => Env::Prod,
            _ => Env::Dev,
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid database url: {0}")]
    InvalidDbUrl(String),
    #[error("invalid solochain rpc url: {0}")]
    InvalidRpcUrl(String),
    #[error("invalid port value for INDEXER_PORT: {0}")]
    InvalidPort(String),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub env: Env,
    pub database_url: Url,
    pub database_name: String,
    /// Solochain RPC endpoint (ws/wss).
    pub solochain_rpc: Url,
    pub indexer_port: u16,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        // 1) Env type
        let env_str = env::var("ENV").ok().unwrap_or_else(|| "dev".into());
        let env_type = Env::from_str_lossy(env_str);

        // 2) Build or parse DB URL
        let db_url = match env::var("DATABASE_URL") {
            Ok(base) => Self::from_base_url(&base, env_type)?,
            Err(_) => Self::from_parts(env_type)?,
        };

        // Extract final database name for convenience
        let db_name = db_url.path().trim_start_matches('/').to_string();

        // 3) Solochain RPC
        let rpc_raw = env::var("SOLOCHAIN_RPC").unwrap_or_else(|_| "ws://solochain:9944".into());
        let solochain_rpc = Url::parse(&rpc_raw)
            .map_err(|e| ConfigError::InvalidRpcUrl(e.to_string()))?;

        // 4) Indexer port
        let port_raw = env::var("INDEXER_PORT").unwrap_or_else(|_| "8081".into());
        let indexer_port = u16::from_str(&port_raw)
            .map_err(|_| ConfigError::InvalidPort(port_raw))?;

        Ok(Self {
            env: env_type,
            database_url: db_url,
            database_name: db_name,
            solochain_rpc,
            indexer_port,
        })
    }

    fn from_base_url(base: &str, env_type: Env) -> Result<Url, ConfigError> {
        let db_base_name = env::var("POSTGRES_DB").unwrap_or_else(|_| "ohlc".into());

        let expanded = base
            .replace("{env}", &env_type.to_string())
            .replace("{ENV}", &env_type.to_string())
            .replace("{db}", &db_base_name)
            .replace("{POSTGRES_DB}", &db_base_name);

        let mut url = Url::parse(&expanded)
            .map_err(|e| ConfigError::InvalidDbUrl(e.to_string()))?;

        // Ensure path contains a db name and is suffixed properly
        let mut name = url.path().trim_start_matches('/').to_string();
        if name.is_empty() {
            // No db given: default to provided POSTGRES_DB
            name = db_base_name;
        }
        name = Self::with_env_suffix(&name, env_type);
        url.set_path(&format!("/{}", name));

        Ok(url)
    }

    /// Build DB URL from discrete POSTGRES_* values when `DATABASE_URL` is not provided.
    fn from_parts(env_type: Env) -> Result<Url, ConfigError> {
        let user = env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".into());
        let pass = env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".into());
        let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "db".into());
        let port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".into());
        let base_name = env::var("POSTGRES_DB").unwrap_or_else(|_| "ohlc".into());

        let db_name = Self::with_env_suffix(&base_name, env_type);
        let conn = format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db_name);
        Url::parse(&conn).map_err(|e| ConfigError::InvalidDbUrl(e.to_string()))
    }

    /// Add `_dev`/`_prod` suffix to a db name, avoiding double suffixes and correcting mismatches.
    fn with_env_suffix(base: &str, env_type: Env) -> String {
        let want = env_type.to_string();
        // strip any trailing _dev/_prod and apply wanted one
        let stripped = base
            .trim_end_matches("_dev")
            .trim_end_matches("_prod");
        format!("{}_{}", stripped, want)
    }
}
