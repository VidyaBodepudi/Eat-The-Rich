use crate::models::RawMention;

#[derive(Debug, PartialEq)]
pub enum InferenceTier {
    Tier1, // Fast path: Gemma 2B / Phi-3 Mini (Predictable structure)
    Tier2, // Slow path: Gemma 4 MoE / 9B (Complex, sarcastic, deep reasoning)
}

pub struct TaskRouter;

impl TaskRouter {
    pub fn new() -> Self {
        Self
    }

    /// Semantic Decomposition: Evaluates a mention and decides which inference
    /// model size needs to handle it based on complexity and sarcasm markers.
    pub fn route_mention(&self, mention: &RawMention) -> InferenceTier {
        let text_lower = mention.text.to_lowercase();
        
        // 1. Sarcasm / Complex Signal Detection
        // If they use loss porn/bankruptcy language but accompany it with bullish emojis 
        // or extreme positive sentiment, it is highly likely deep sarcasm.
        let has_bullish_emojis = text_lower.contains("🚀") || text_lower.contains("💎") || text_lower.contains("moon");
        let has_bearish_text = text_lower.contains("loss porn") 
            || text_lower.contains("bankrupt") 
            || text_lower.contains("down 50%")
            || text_lower.contains("dilution")
            || text_lower.contains("bagholder");

        if has_bullish_emojis && has_bearish_text {
            // "Loss porn 🚀🚀" -> Requires Tier 2 to unravel the sarcastic entanglement
            return InferenceTier::Tier2;
        }

        // 2. High Perplexity / Multi-ticker Entanglement
        // Mentioning multiple tickers creates nested grammar that Tier 1 may fail to separate.
        let ticker_count = text_lower.matches('$').count();
        if ticker_count > 1 {
            return InferenceTier::Tier2;
        }

        // 3. SEC Filings or Formal News (usually longer, lacking retail slang)
        // If it's very long but has zero emojis, it's likely a complex macro thesis or filing.
        if text_lower.len() > 300 && !has_bullish_emojis && !text_lower.contains("🐻") {
            return InferenceTier::Tier2;
        }

        // 4. Default: Standard Triage Flow
        // Short, simple formats ("$TSLA to the moon", "bought more AAPL")
        InferenceTier::Tier1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::RawMention;

    fn mock_mention(text: &str) -> RawMention {
        RawMention {
            id: "test_id".to_string(),
            source: "mock".to_string(),
            ticker: "GME".to_string(),
            text: text.to_string(),
            timestamp: 0,
            engagement: 0,
        }
    }

    #[test]
    fn test_tier1_simple_bullish() {
        let router = TaskRouter::new();
        let mention = mock_mention("$GME to the moon! 🚀");
        assert_eq!(router.route_mention(&mention), InferenceTier::Tier1);
    }

    #[test]
    fn test_tier2_sarcasm_entanglement() {
        let router = TaskRouter::new();
        let mention = mock_mention("This stock is headed for complete loss porn and bankruptcy... 🚀💎");
        assert_eq!(router.route_mention(&mention), InferenceTier::Tier2);
    }

    #[test]
    fn test_tier2_multi_ticker() {
        let router = TaskRouter::new();
        let mention = mock_mention("$TSLA is fine but $RIVN is crashing hard.");
        assert_eq!(router.route_mention(&mention), InferenceTier::Tier2);
    }
}
