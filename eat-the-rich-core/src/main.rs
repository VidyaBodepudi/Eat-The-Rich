pub mod models;
pub mod db;
pub mod ingest;
pub mod api;
pub mod router;
pub mod infer;
pub mod yfinance;
pub mod arbitrage;

use dotenvy::dotenv;
use ringbuf::traits::{Consumer, Producer, Split};
use ringbuf::HeapRb;
use std::sync::Arc;
use models::RawMention;

#[tokio::main]
async fn main() {
    // 1. Initialize environment (Keys, Intervals)
    dotenv().ok();
    println!("Starting Eat The Rich v3 (HFT Local Engine) 🚀");

    // 2. Initialize Embedded Zero-Copy Analytics DB (DuckDB)
    let db_client = Arc::new(std::sync::Mutex::new(db::DbClient::new("eat_the_rich.duckdb").expect("Failed to init DuckDB")));
    db_client.lock().unwrap().init_views().expect("Failed to initialize analytical views");
    println!("✅ Embedded DuckDB Time-Series Engine Online (Views Bound)");

    // 3. Setup SPSC Lock-Free Ring Buffer for Thread Comm
    // The ring buffer is pre-allocated contiguous memory to prevent allocation spikes
    let rb = HeapRb::<RawMention>::new(8192);
    let (mut prod, mut cons) = rb.split();

    let handle = tokio::runtime::Handle::current();
    // 4. Spawn the Lock-Free Consumer Task (Phase 2 & 3: NLP/DB/Arbitrage)
    // This task sits in a zero-allocation loop waiting for new memory to process
    let _processor_thread = std::thread::spawn(move || {
        let task_router = router::TaskRouter::new();
        let inference_engine = infer::LocalInferenceEngine::new().expect("Failed to bind ML device");
        let arbitrage_engine = arbitrage::ArbitrageMatcher::new();
        
        loop {
            // Because this is a lock-free buffer, popping does not acquire OS Mutexes.
            if let Some(mention) = cons.try_pop() {
                // Semantic Decomposition: Figure out which local model executes this
                let tier = task_router.route_mention(&mention);
                
                println!("🧠 [NLP Pipeline] Received 0-Copy mention for {}: {}", mention.ticker, mention.id);
                println!("🔀 [Router] Decomposed task to: {:?}", tier);
                
                // Native Rust LLM Execution via HuggingFace Candle
                let polarity = inference_engine.classify(&mention.text, &tier);
                
                let tick = models::SentimentTick {
                    ticker: mention.ticker.clone(),
                    timestamp: mention.timestamp,
                    polarity,
                    topic: "Earnings/Hype".to_string(), // Would be derived via BERTopic overlay
                    velocity: 1.0,
                };
                
                // Save cleanly into the analytical store
                if let Err(e) = db_client.lock().unwrap().insert_tick(&tick) {
                    eprintln!("❌ Failed to write tick to DuckDB: {}", e);
                } else {
                    println!("💾 [DB] Successfully saved tick for {}", tick.ticker);
                }
                
                // TRIGGER LAYER 2: ARBITRAGE OVERLAY
                // Evaluating immediately as scaffold (normally pulled via async DuckDB Accel view polling)
                handle.block_on(async {
                    arbitrage_engine.evaluate_anomaly(&mention.ticker, tick.velocity).await;
                });
            }
            // Sleep slightly to prevent burning 100% CPU on empty buffer
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    // 5. Run the async Toki-powered Ingest Loop & API Server
    let reddit_client = ingest::reddit::RedditClient::new();
    let stocktwits_client = ingest::stocktwits::StocktwitsClient::new();
    
    // Spawn Dashboard API Server in background
    tokio::spawn(async {
        api::start_server().await;
    });
    
    // Simulate a polling loop fetching every 10 seconds for scaffolding
    loop {
        println!("📡 Polling APIs for momentum...");
        
        // This simulates our Tokio concurrent fetch
        let mut _mock_mentions = reddit_client.fetch_latest("wallstreetbets", "$GME").await;
        let mut _st_mentions = stocktwits_client.fetch_stream("GME").await;
        
        // Mock data injection to test pipeline
        let mock_mention = RawMention {
            id: format!("mock_id_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros()),
            source: "mock".to_string(),
            ticker: "GME".to_string(),
            text: "This is going to the moon! 🚀🚀🚀".to_string(),
            timestamp: 1620000000,
            engagement: 4200,
        };
        
        // The lock-free push immediately passes memory ownership to the NLP thread
        let _ = prod.try_push(mock_mention);

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
