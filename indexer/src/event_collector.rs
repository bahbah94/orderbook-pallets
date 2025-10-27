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

        let extrins = block.extrinsics().await?;
        for ex in extrins.iter() {
            let ex = ex?;
            let events = ex.events().await?;

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
                        if let Err(e) = parse_and_insert_trade(&pool, block_number, &event_values).await {
                            debug!("❌ Failed to parse trade: {}", e);
                            println!("Parse error: {}", e);
                        } else {
                            info!("✅ Trade executed in block {}", block_number);
                        }
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
    }

    Ok(())
}

/// Parse TradeExecuted event fields and insert into database
async fn parse_and_insert_trade(
    pool: &PgPool,
    block_number: u32,
    event_values: &subxt::ext::scale_value::Value,
) -> Result<()> {
    // Debug print the structure so we can see what we're working with
    debug!("Event fields: {:?}", event_values);
    println!("Event structure: {}", event_values);

    // For now, just log that we received it
    // TODO: Parse the actual field values once we see the structure
    info!("📊 TradeExecuted event received in block {}", block_number);

    Ok(())
}