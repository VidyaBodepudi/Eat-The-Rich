use duckdb::{Connection, Result};
use std::path::Path;

pub struct DbClient {
    pub conn: Connection,
}

impl DbClient {
    /// Initialize the local DuckDB embedded instance
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        // Initialize the analytical schema
        // DuckDB is highly optimized for OLAP column-based time-series queries
        conn.execute(
            "CREATE TABLE IF NOT EXISTS sentiment_ticks (
                ticker VARCHAR,
                timestamp BIGINT,
                polarity DOUBLE,
                topic VARCHAR,
                velocity DOUBLE
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    /// Creates analytical views for tracking momentum velocity and acceleration.
    /// Velocity is the change in polarity. Acceleration is the change in Velocity.
    pub fn init_views(&self) -> Result<()> {
        self.conn.execute(
            "CREATE OR REPLACE VIEW v_sentiment_velocity AS
            SELECT 
                ticker,
                timestamp,
                polarity,
                (polarity - LAG(polarity, 3) OVER (PARTITION BY ticker ORDER BY timestamp)) / 3.0 AS velocity
            FROM sentiment_ticks",
            [],
        )?;
        
        self.conn.execute(
            "CREATE OR REPLACE VIEW v_sentiment_acceleration AS
            SELECT 
                ticker,
                timestamp,
                velocity,
                (velocity - LAG(velocity, 2) OVER (PARTITION BY ticker ORDER BY timestamp)) / 2.0 AS acceleration
            FROM v_sentiment_velocity
            WHERE velocity IS NOT NULL",
            [],
        )?;

        Ok(())
    }

    /// Appends a new sentiment reading directly into the analytical DB
    pub fn insert_tick(&self, tick: &crate::models::SentimentTick) -> Result<()> {
        self.conn.execute(
            "INSERT INTO sentiment_ticks (ticker, timestamp, polarity, topic, velocity) VALUES (?, ?, ?, ?, ?)",
            (&tick.ticker, &tick.timestamp, &tick.polarity, &tick.topic, &tick.velocity),
        )?;
        Ok(())
    }
}
