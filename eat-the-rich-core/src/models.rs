use serde::{Deserialize, Serialize};
use arrow::datatypes::{Field, DataType, Schema};
use std::sync::Arc;

/// Raw incoming text data from social platforms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawMention {
    pub id: String,
    pub source: String, // "reddit", "stocktwits", etc.
    pub ticker: String,
    pub text: String,
    pub timestamp: i64, // Unix timestamp
    pub engagement: i32, // Upvotes, retweets, etc.
}

/// The final processed sentiment tick to be inserted into DuckDB
#[derive(Debug, Clone)]
pub struct SentimentTick {
    pub ticker: String,
    pub timestamp: i64,
    pub polarity: f64, // -1.0 to +1.0
    pub topic: String,
    pub velocity: f64,
}

/// Arrow schema definition for Zero-Copy pass to DuckDB
pub fn sentiment_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("ticker", DataType::Utf8, false),
        Field::new("timestamp", DataType::Int64, false),
        Field::new("polarity", DataType::Float64, false),
        Field::new("topic", DataType::Utf8, false),
        Field::new("velocity", DataType::Float64, false),
    ]))
}
