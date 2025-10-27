use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;

mod event_collector;
mod trade_mapper;
mod orderbook_reducer;
mod db;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("indexer=debug,info")
        .init();

    info!("🚀 Starting Orderbook Indexer...");

    // Load config
    dotenv::dotenv().ok();
    let node_url = std::env::var("NODE_WS_URL")
        .unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Initialize database
    info!("📊 Connecting to database...");
    let pool = db::init_pool(&db_url).await?;
    db::run_migrations(&pool).await?;

    // Start event collector
    info!("🔌 Connecting to node at {}", node_url);
    event_collector::start(&node_url, pool).await?;

    Ok(())
}