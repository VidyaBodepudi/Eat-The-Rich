use reqwest::Client;
use crate::models::RawMention;

pub struct RedditClient {
    client: Client,
}

impl RedditClient {
    pub fn new() -> Self {
        // We will pull config from dotenv in the main runner
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("User-Agent", "EatTheRich/1.0.0 (High Frequency Scraping Engine)".parse().unwrap());
        
        Self {
            client: Client::builder().default_headers(headers).build().unwrap(),
        }
    }
    
    /// Async function to fetch latest mentions for a ticker on a specific subreddit
    /// Because we are relying on Rust tokio, this fetch is non-blocking.
    pub async fn fetch_latest(&self, _subreddit: &str, _ticker: &str) -> Vec<RawMention> {
        // Placeholder for the actual Reddit search API logic.
        // It maps the JSON response immediately onto our RawMention structs.
        vec![]
    }
}
