use sqlx::PgPool;
use anyhow::Result;

pub async fn process_trade(
    pool: &PgPool,
    trade_id: u64,
    // other stuff
) -> Result<()> {
    todo!("impl trade transformation")

    Ok(())
}