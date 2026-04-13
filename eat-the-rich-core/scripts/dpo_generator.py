"""
Phase 3 - Direct Preference Optimization (DPO) Dataset Generator
----------------------------------------------------------------
This script runs weekly to scrape backward-looking T+1 day performance data
from the market and matches it against the Sentiment NLP outputs. 

If our LLM said "Bullish" but the stock tanked 10% the next day, that prediction 
becomes a "Rejected" sample. If it predicted correctly, it becomes "Preferred".
This pairs directly with HuggingFace TRL (Transformer Reinforcement Learning) libs.
"""

import json
from datetime import datetime

def generate_dpo_pairs():
    # Scaffold data pipeline
    historical_ticks = [
        {"text": "Ryan Cohen is adding $GME to the moon! 🚀🚀", "model_prediction": "Bullish", "actual_t1_return": -0.15},
        {"text": "The earnings are fake, huge dilution coming for $AMC", "model_prediction": "Bearish", "actual_t1_return": -0.22}
    ]
    
    dpo_dataset = []
    
    for tick in historical_ticks:
        # If prediction was Bullish but return was negative -> Bad Prediction
        if tick["model_prediction"] == "Bullish" and tick["actual_t1_return"] < 0:
            dpo_dataset.append({
                "prompt": f"Analyze sentiment context and market trajectory: {tick['text']}",
                "chosen": "Bearish",     # Market forces dictate it was actually bearish
                "rejected": "Bullish"    # The model's initial mistake
            })
        elif tick["model_prediction"] == "Bearish" and tick["actual_t1_return"] < 0:
            dpo_dataset.append({
                "prompt": f"Analyze sentiment context and market trajectory: {tick['text']}",
                "chosen": "Bearish",
                "rejected": "Bullish"
            })
            
    print(f"Generated {len(dpo_dataset)} Preference Pairs for DPO LoRA run.")
    with open('dpo_dataset_weekly.jsonl', 'w') as f:
        for pair in dpo_dataset:
            f.write(json.dumps(pair) + '\n')

if __name__ == "__main__":
    generate_dpo_pairs()
