use anyhow::Result;
use sqlx::PgPool;
use subxt::{OnlineClient, PolkadotConfig};
use tracing::{info, debug};

pub async fn start(node_url: &str, pool: PgPool) -> Result<()> {
    let api = OnlineClient::<PolkadotConfig>::from_url(node_url).await?;
    
    info!("✅ Connected to chain: {:?}", api.runtime_version());

    let mut blocks = api.blocks().subscribe_finalized().await?;
    
    info!("📡 Listening for events...");

    while let Some(block) = blocks.next().await {
        let block = block?;
        let block_number = block.header().number;

        debug!("Processing block number: {}", block_number);

        // Get events directly from block
        let events = block.events().await?;

        debug!("   EVENTS:");
        for evt in events.iter() {
            let evt = evt?;
            let pallet_name = evt.pallet_name();
            let event_name = evt.variant_name();
            let event_values = evt.field_values()?;

            println!("        {}_{}", pallet_name, event_name);
            println!("          {}", event_values);

            // Route to appropriate handler
            match (pallet_name, event_name) {
                ("Orderbook", "TradeExecuted") => {
                    println!("🎯 TradeExecuted event detected!");
                    println!("Event values: {:#?}", event_values);
                    info!("✅ Trade executed in block {}", block_number);
                },
                ("Orderbook", "OrderPlaced") => {
                    info!("📦 Order placed in block {}", block_number);
                },
                ("Orderbook", "OrderCancelled") => {
                    info!("❌ Order cancelled in block {}", block_number);
                },
                ("Orderbook", "OrderFilled") => {
                    info!("✅ Order filled in block {}", block_number);
                },
                ("Orderbook", "OrderPartiallyFilled") => {
                    info!("📊 Order partially filled in block {}", block_number);
                },
                _ => {
                    // Ignore events from other pallets
                }
            }
        }
    }

    Ok(())
}