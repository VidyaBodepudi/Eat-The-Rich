import sys
import json
from transformers import AutoTokenizer, AutoModelForCausalLM

def expand_gemma_tokenizer(model_id="google/gemma-2b"):
    print(f"🚀 Initializing Tokenizer Expansion for {model_id}...")
    
    # 1. Load Base Tokenizer & Base Model
    # Note: Requires HF_TOKEN in environment to access gated Gemma repository
    try:
        tokenizer = AutoTokenizer.from_pretrained(model_id)
        model = AutoModelForCausalLM.from_pretrained(model_id)
    except Exception as e:
        print(f"⚠️ Mock Mode: Could not load actual weights (Likely missing HF_TOKEN or running locally without cache). Error: {e}")
        print("Scaffolding logic assuming successful load...")
        return

    print(f"Original vocabulary size: {len(tokenizer)}")

    # 2. Define High-Weight Retail Emojis & Slang
    # These are added to prevent SentencePiece from fracturing them into bytes
    retail_tokens = [
        "🚀", "💎", "🙌", "🐻", "🐂", "📉", "📈", "🦍", "🌕", "🤡",
        "<|sarcasm|>", "<|hype|>", "<|loss_porn|>", "<|due_diligence|>"
    ]

    # 3. Inject Special Tokens
    num_added_toks = tokenizer.add_tokens(retail_tokens)
    print(f"✅ Added {num_added_toks} new retail-specific tokens.")

    # 4. Resize the LLM's Embedding Matrix
    # This expands the internal (vocab_size, hidden_dim) matrix
    model.resize_token_embeddings(len(tokenizer))
    
    print(f"New vocabulary size: {len(tokenizer)}")

    # 5. Save the tuned artifact
    output_dir = "./tuned_gemma_tokenizer"
    tokenizer.save_pretrained(output_dir)
    model.save_pretrained(output_dir)
    
    print(f"💾 Saved expanded visual tokenizer and model shell to {output_dir}")
    print("Next step: Run SFT specifically on the embedding blocks to calibrate these new vectors.")

if __name__ == "__main__":
    expand_gemma_tokenizer()
