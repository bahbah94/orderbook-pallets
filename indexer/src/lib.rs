pub mod env;
pub mod db;
pub mod server;

use crate::env::Config;
use postgres::{Client, NoTls};
use std::{thread, time::Duration};

/// Boot the indexer: ensure DB exists, then start HTTPS/WSS server.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let cfg = Config::load()?;

    // DB bootstrap with retry
    let mut last_err: Option<postgres::Error> = None;
    for _ in 0..10 {
        match Client::connect(cfg.database_url.as_str(), NoTls) {
            Ok(mut client) => {
                db::init_schema(&mut client)?;
                println!(
                    "indexer ready (env={}, db={}, rpc={}, port={})",
                    cfg.env, cfg.database_name, cfg.solochain_rpc, cfg.indexer_port
                );
                // Start HTTPS+WSS server (blocking on tokio runtime)
                return server::start(cfg);
            }
            Err(e) => {
                last_err = Some(e);
                thread::sleep(Duration::from_millis(500));
            }
        }
    }

    match last_err {
        Some(e) => Err(Box::new(e)),
        None => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to connect to Postgres (no error captured)",
        ))),
    }
}
