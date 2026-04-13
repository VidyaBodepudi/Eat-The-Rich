use crate::yfinance::{Fundamentals, YFinanceClient};

#[derive(Debug)]
pub enum Signal {
    ShortTarget, // Overbought (High Hype + Bad Fundamentals)
    BuyTarget,   // Oversold (High Fear + Good Fundamentals)
    NoSignal,
}

pub struct ArbitrageMatcher {
    yf_client: YFinanceClient,
}

impl ArbitrageMatcher {
    pub fn new() -> Self {
        Self {
            yf_client: YFinanceClient::new(),
        }
    }

    /// Evaluates if explosive social momentum contradicts the 
    /// financial reality of the balance sheet.
    pub async fn evaluate_anomaly(&self, ticker: &str, acceleration: f64) -> Signal {
        // Fetch fundamentals
        let fundamentals = self.yf_client.fetch_fundamentals(ticker).await;
        
        // 1. Define constraints
        let is_hype_explosive = acceleration > 0.5; // Massive positive momentum spike
        let is_panic_explosive = acceleration < -0.5; // Massive negative panic spike
        
        let pe = fundamentals.forward_pe.unwrap_or(15.0);
        let is_overvalued = pe > 50.0;
        let is_undervalued = pe < 10.0;
        
        let has_bad_earnings = fundamentals.eps_trailing.unwrap_or(1.0) < 0.0;
        let has_good_earnings = fundamentals.eps_trailing.unwrap_or(-1.0) > 0.0;

        // 2. Overlay Logics
        if is_hype_explosive && is_overvalued && has_bad_earnings {
            println!("🚨 [ARBITRAGE ENGINE] SIGNAL MATCH: {} is suffering Irrational Exuberance! (SHORT TARGET)", ticker);
            Signal::ShortTarget
        } else if is_panic_explosive && is_undervalued && has_good_earnings {
            println!("🚨 [ARBITRAGE ENGINE] SIGNAL MATCH: {} is experiencing Unwarranted Panic! (BUY TARGET)", ticker);
            Signal::BuyTarget
        } else {
            Signal::NoSignal
        }
    }
}
