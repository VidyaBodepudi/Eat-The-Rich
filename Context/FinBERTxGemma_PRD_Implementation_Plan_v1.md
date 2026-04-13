# Product Requirements Document: FinBERT x Gemma NLP Engine 

## 1. Goal
To architect a production-ready, engineering-grade Natural Language Processing pipeline for the "Eat The Rich" sentiment tracker. This system ingests high-velocity retail chatter, continuously improves its logic via Reinforcement Learning (DPO) and LoRA adapters, and surfaces high-probability market signals (T-1 analysis) directly to the user for evaluation.

---

## 2. Core Architecture Overview
The system operates across three tightly decoupled zones:
1. **The Ingestion & Data Logging Zone**
2. **The Rust Inference Serving Layer (Real-time Task Decomposition)**
3. **The RL & Continuous Training Loop Zone (Asynchronous)**

### 2.1 Ingestion & Storage Layer (Time-Series Logging)
Since the platform is designed to aggregate and surface signal recommendations rather than execute fully autonomous trades, we can bypass strict immutable event-sourcing in favor of an optimized analytical data store.
- **Data Flow**: Social firehose data (Reddit, StockTwits) is ingested and parsed.
- **Storage**: We utilize a high-performance time-series database (e.g., TimescaleDB or ClickHouse). Every post is stored alongside the assigned sentiment score, the ticker symbol, and the timestamp. This allows for rapid T-1 lookback aggregation.

### 2.2 The Rust Inference Serving Layer & Task Decomposition
To maximize computational speed and local capability, the entire inference backbone will be written in **Rust using wrappers like local Candle or TensorRT-LLM**. 

**Hierarchical Task Routing (Semantic Decomposition):**
The serving layer acts as an intelligent router assigning tasks dynamically to optimize VRAM and limit latency:
- **Tier 1 Routing (High-Volume Triage):** Simple posts ("$TSLA to the moon 🚀") route to smaller, extremely fast local parameters (Gemma 2B / Phi-3 Mini) for basic polarity scoring.
- **Tier 2/3 Routing (Complex Reasoning & Sarcasm):** High perplexity data/sarcasm flags are escalated to the larger models (Gemma 4 9B or the 30B MoE). These trade slower generation speeds for deep, multi-shot contextual logic.
- **Adapter Hot-Swapping:** The Rust server holds the base Gemma weights in memory and hot-swaps specific LoRA vectors over them on the fly.

### 2.3 Continuous Training & DPO Pipeline
The model retroactively grades itself based on real market outcomes.
1. **Retrospective Pair Generator**: Computes yesterday's predicted sentiment vs actual stock price Delta, generating `chosen` and `rejected` pair data.
2. **DPO Worker Node**: Ingests new pairs in batches to continuously update the LoRA without modifying base model behavior.

---

## 3. Deployment Safety: LoRA Evaluation Benchmarks
Prior to a newly DPO-trained "Slang" LoRA being pushed to the live Rust inference server, it must pass an automated CI/CD gating process against a fixed holdout dataset. Here is the suite of ~20 safety checks organized by category to prevent mode collapse and echo chambers:

**Category A: Catastrophic Forgetting & Base Logic**
1. **SEC Filing Baseline Check:** Model must still correctly score the sentiment of a dry, emotionless institutional 10-K filing.
2. **Standard Lexical Check:** Simple statements ("The earnings missed by 10%") must stay Bearish.
3. **Ticker Hallucination:** Model must not generate sentiment for tickers not explicitly mentioned in the text.
4. **Length Penalty Check:** Model must not automatically assign higher absolute polarity just because a post is very long.

**Category B: Sarcasm, Emoji, & Memetic Rigidity**
5. **Sarcasm Inversion Test:** "GME is down 50% again, I love losing money so much 🚀📉" must correctly route to Bearish/Anger, overriding the 🚀 emoji.
6. **Meme-Overfit Limit:** A post with 50 rocket emojis should not score 50x higher than a post with 2 rocket emojis; it must normalize.
7. **Baggage Decay:** If $BBBY goes bankrupt, the model shouldn't permanently treat the string "BBBY" as toxic in historical/theoretical contexts.
8. **Emoji Contrast Test:** A positive text paired with a bear icon ("Great quarter 🐻") must trigger high perplexity/Tier 2 routing rather than an immediate bullish tag.

**Category C: Financial Context & Entanglement**
9. **Multi-Ticker Clarification:** "$RIVN is trash but $TSLA is about to pop" must yield exact opposite vectors for both tickers.
10. **Options Awareness:** Understanding that "Buying Puts" is inherently Bearish for the underlying stock, even if the user is "Bullish" that they will make money.
11. **Short Squeeze Semantics:** Recognizing that "Short interest is at 100%" translates to immense retail Bullish momentum, not institutional logic.
12. **Dilution Recognition:** Posts cheering a stock split must be parsed differently than posts despairing over offering/dilution dilution, despite similar vocabulary ("shares").

**Category D: Echo Chamber / Systemic Degradation**
13. **Neutral Bias Trap:** The new training loop must not learn to simply label everything "Neutral" to safely reduce DPO penalty loss.
14. **Always-Bullish Retail Trap:** The model cannot trend toward making >80% of predictions bullish just because retail texts skew heavily positive.
15. **Hype-Word Saturation:** Repeated exposure to words like "Moon" shouldn't erode its sentiment weight to zero.
16. **T-1 Causality Check:** The LoRA must prove it hasn't mapped sentiment retroactively. It shouldn't just "guess" high sentiment because a stock historically goes up on Tuesdays.
17. **Volume Independence:** Sentiment classification of a *single text* should remain constant regardless of whether that ticker has 10 or 10,000 posts that minute.

---

## 4. UI & Data Surfacing Strategy
Since the platform functions as an analytical oracle rather than an execution bot, surfacing the data intuitively is paramount.

- **Backend Push Mechanics:** The Rust pipeline calculates trailing 24h sentiment shifts continually. We will use **WebSockets** or **gRPC** to stream live sentiment deltas to the frontend.
- **The Dashboard UI (React/Vite or Next.js):**
    - **The Hype Heatmap:** A visual block-map of the top 100 most active tickers. Deep Green = High Sentiment + High Volume.
    - **T-1 Signal Alerts:** A chronological notification feed surfacing mathematically anomalous divergences (e.g., "TSLA Price is flat, but Bullish Sentiment Volume spiked 400% in the last 15 minutes").
    - **Post Verification Drill-Down:** Clicking a specific stock's hype meter will pull up an embedded feed of the exact raw Reddit/StockTwit data driving the score, allowing the human trader to verify the logic.
    - **Adapter Mode Toggle:** A UI switch enabling the trader to compare what the "Institutional Base Model" thinks versus what the "Current Memetic LoRA" thinks about the exact same data feed.

---

## 5. Tokenizer Engineering Plan
Bridging the gap between unicode characters and financial semantic intent.

1. **Vocabulary Expansion**: Add highly critical retail emojis and slang compounds as discrete native tokens to the Gemma `tokenizer.model` to prevent byte-fallback fracturing.
2. **Matrix Resizing & SFT Tuning**: Expand the embedding matrix and freeze the transformer blocks. Run a rapid embedding-only SFT pass so the model learns where to map these new unique tokens in its existing linguistic space.
