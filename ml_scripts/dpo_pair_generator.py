import json
import random
from datetime import datetime, timedelta

def mock_market_api(ticker):
    """Mocks resolving actual T+1 market closes."""
    # Return a random percentage movement between -10% and +10%
    return random.uniform(-0.10, 0.10)

def generate_dpo_pairs():
    print("🔄 Running Retrospective DPO Pair Generation...")
    
    # In a real environment, this pulls from the DuckDB Time-Series DB
    # We mock yesterday's predictions here.
    historical_predictions = [
        {"post": "GME is ripping! 🚀", "model_predicted_polarity": 0.9, "ticker": "GME"},
        {"post": "Holding TSLA but the dilution fears are real", "model_predicted_polarity": 0.2, "ticker": "TSLA"},
        {"post": "The CEO is a fraud, I'm buying Puts", "model_predicted_polarity": -0.8, "ticker": "RIVN"}
    ]
    
    dpo_dataset = []
    
    for pred in historical_predictions:
        delta = mock_market_api(pred["ticker"])
        
        # Determine if the market validated the model's sentiment
        market_was_bullish = delta > 0.02
        market_was_bearish = delta < -0.02
        
        # Basic mapping logic:
        # If model predicted Bullish (> 0.5) and market was Bullish, the prediction is the 'chosen' pair.
        # If market did the opposite, we synthesize a rejected pair.
        
        if pred["model_predicted_polarity"] > 0.5:
            model_string = "bullish"
        elif pred["model_predicted_polarity"] < -0.5:
            model_string = "bearish"
        else:
            model_string = "neutral"
            
        is_correct = (model_string == "bullish" and market_was_bullish) or \
                     (model_string == "bearish" and market_was_bearish) or \
                     (model_string == "neutral" and not market_was_bearish and not market_was_bullish)

        if is_correct:
            chosen = f"Classification: {model_string}"
            rejected = f"Classification: {'bearish' if model_string == 'bullish' else 'bullish'}"
        else:
            # The model made a mistake. The chosen pair is what the market ACTUALLY did.
            if market_was_bullish:
                chosen = "Classification: bullish"
            elif market_was_bearish:
                chosen = "Classification: bearish"
            else:
                chosen = "Classification: neutral"
            rejected = f"Classification: {model_string}"
            
        dpo_dataset.append({
            "prompt": pred["post"],
            "chosen": chosen,
            "rejected": rejected,
            "market_delta_t1": delta
        })
        
    output_file = "dpo_training_pairs.jsonl"
    with open(output_file, "w") as f:
        for item in dpo_dataset:
            f.write(json.dumps(item) + "\n")
            
    print(f"✅ Generated {len(dpo_dataset)} training pairs based on T+1 resolution. Saved to {output_file}")

if __name__ == "__main__":
    generate_dpo_pairs()
