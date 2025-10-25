use postgres::{Client, Error};

#[derive(Debug, Clone)]
pub struct Candle {
    pub symbol: String,
    pub timeframe: String,
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl Candle {
    #[allow(dead_code)]
    pub fn sample(symbol: &str, timeframe: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            timeframe: timeframe.to_string(),
            timestamp: 1_700_000_000, // example
            open: 1.0,
            high: 2.0,
            low: 0.5,
            close: 1.5,
            volume: 42.0,
        }
    }
}

pub fn init_schema(client: &mut Client) -> Result<(), Error> {
    client.batch_execute(
        r#"
        CREATE TABLE IF NOT EXISTS ohlc_candle (
            id          SERIAL PRIMARY KEY,
            symbol      VARCHAR NOT NULL,
            timeframe   VARCHAR NOT NULL,
            timestamp   BIGINT  NOT NULL,
            open        DOUBLE PRECISION NOT NULL,
            high        DOUBLE PRECISION NOT NULL,
            low         DOUBLE PRECISION NOT NULL,
            close       DOUBLE PRECISION NOT NULL,
            volume      DOUBLE PRECISION NOT NULL,
            created_at  TIMESTAMPTZ DEFAULT now()
        );

        -- Unique per (symbol, timeframe, timestamp) to allow UPSERT-like behavior
        CREATE UNIQUE INDEX IF NOT EXISTS ohlc_candle_uq
            ON ohlc_candle(symbol, timeframe, timestamp);
        "#,
    )
}

pub fn insert_candle(client: &mut Client, c: Candle) -> Result<u64, Error> {
    client.execute(
        r#"
        INSERT INTO ohlc_candle
            (symbol, timeframe, timestamp, open, high, low, close, volume)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (symbol, timeframe, timestamp) DO NOTHING
        "#,
        &[
            &c.symbol,
            &c.timeframe,
            &c.timestamp,
            &c.open,
            &c.high,
            &c.low,
            &c.close,
            &c.volume,
        ],
    )
}

pub fn latest_candles(
    client: &mut Client,
    symbol: &str,
    timeframe: &str,
    limit: i64,
) -> Result<Vec<Candle>, Error> {
    let rows = client.query(
        r#"
        SELECT symbol, timeframe, timestamp, open, high, low, close, volume
        FROM ohlc_candle
        WHERE symbol = $1 AND timeframe = $2
        ORDER BY timestamp DESC
        LIMIT $3
        "#,
        &[&symbol, &timeframe, &limit],
    )?;

    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        out.push(Candle {
            symbol: row.get(0),
            timeframe: row.get(1),
            timestamp: row.get(2),
            open: row.get(3),
            high: row.get(4),
            low: row.get(5),
            close: row.get(6),
            volume: row.get(7),
        });
    }
    Ok(out)
}
