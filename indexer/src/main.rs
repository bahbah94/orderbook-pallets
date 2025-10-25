use indexer::env::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment from .env (optional)
    dotenvy::dotenv().ok();

    // Build configuration
    let cfg = Config::load()?;
    println!("✅ Loaded configuration:");
    println!("  Environment: {:?}", cfg.env);
    println!("  Database URL: {}", cfg.database_url);
    println!("  Database name: {}", cfg.database_name);
    println!("  Solochain RPC: {}", cfg.solochain_rpc);
    println!("  Indexer port: {}", cfg.indexer_port);

    // Run the indexer core logic
    indexer::run()?;
    Ok(())
}
