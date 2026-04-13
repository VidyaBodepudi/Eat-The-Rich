use reqwest::Client;
use crate::models::RawMention;

pub struct StocktwitsClient {
    client: Client,
}

impl StocktwitsClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Fetches the latest stream for a specific ticker directly from the StockTwits pure-finance pipeline.
    pub async fn fetch_stream(&self, _ticker: &str) -> Vec<RawMention> {
        // Placeholder hitting `api.stocktwits.com/api/2/streams/symbol/{ticker}.json`
        vec![]
    }
}
