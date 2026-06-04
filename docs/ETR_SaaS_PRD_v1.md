# Product Requirements Document — Eat The Rich (SaaS)

**Version:** 1.0
**Status:** Draft for build
**Date:** 2026-06-03
**Supersedes:** `Context/ETR_PRD_Implementation_Plan_v3.md`, `Context/FinBERTxGemma_PRD_Implementation_Plan_v1.md` (deployment architecture only)
**Companion:** [ETR_SaaS_TDD_v1.md](./ETR_SaaS_TDD_v1.md)

---

## 1. Vision

Eat The Rich (ETR) is a multi-platform SaaS that tells active traders **when the crowd is wrong**.

Every trading day, retail discourse on Reddit, StockTwits, X, and beyond generates a tidal wave of conviction — euphoria, panic, hype, capitulation. Most of it is noise. Occasionally it is a tradable signal: a moment where the crowd's sentiment has *detached* from reality. ETR's job is to find those moments and rank them by how confidently history, fundamentals, and the crowd disagree with each other.

ETR fuses **three independent signal legs** into one conviction score:

1. **The crowd leg** — real-time retail sentiment, its velocity, and its acceleration (how fast and how hard the mood is shifting).
2. **The reality leg** — fundamentals, insider activity, options flow, and short interest (what the money is actually doing).
3. **The history leg** — a 30-year historical base rate: given this exact technical setup, what happened next, and how often.

When all three legs align against the crowd — *retail is euphoric, insiders are selling, and this setup faded 77% of the time historically* — ETR emits a high-conviction signal. When they conflict, it says so, and lowers confidence accordingly.

> **One-line thesis:** Detect when retail sentiment diverges from fundamental reality **and** the historical base rate — and surface that divergence as a single, confidence-scored conviction signal.

### 1.1 What changed from the original product

The original ETR was a **local-first desktop app**: a single Rust binary running ingestion, inference, storage, and UI on `localhost`, optimized for zero external dependencies and GPU-local inference. This document defines a **clean break** to a **multi-platform SaaS** (web + desktop + mobile) with a hosted backend.

The *algorithms* carry forward. The *deployment model* does not. See [§9 Migration & Supersession](#9-migration--supersession).

### 1.2 Why now / why this wins

- **Differentiation.** Pure base-rate products (e.g., TradeOdds) answer "what did history do?" Pure sentiment trackers answer "what's hot?" **ETR is the only product fusing both with a fundamentals leg into a divergence thesis.** That fusion is hard to copy and is the core moat.
- **Owned data asset.** A self-computed historical base-rate corpus (see [§5.1](#51-the-base-rate-engine-owned-corpus)) is a durable, defensible asset that improves with every data source we add.
- **Cost-aware reach.** A two-model sentiment cascade (cheap triage + premium reasoning) plus an on-device "Pro" mode lets us serve free/mass users without linear GPU cost growth.

---

## 2. Goals & Non-Goals

### 2.1 Goals
- Ship a hosted, multi-tenant SaaS serving web, desktop, and mobile clients from a single backend.
- Build and own a 30-year historical base-rate corpus covering all standard equities, securities, and ETFs.
- Fuse sentiment + fundamentals + base rate into a transparent, verifiable conviction score.
- Preserve the performance ethos of the original (zero-copy, low-latency hot path) where it matters.
- Establish a tiered, metered monetization model (free → Pro → Power/Quant).

### 2.2 Non-Goals (v1)
- **Trade execution.** ETR is an analytical oracle, not an order-routing or auto-trading system. It surfaces signals for human evaluation.
- **Investment advice.** All outputs are historical/statistical context, not recommendations (see [§8 Compliance](#8-compliance-legal--trust)).
- **Crypto & alternative assets.** Deferred to Phase 2.
- **Redistribution of third-party computed datasets.** We compute our own factors from raw inputs; we never resell another vendor's derived data (see [§8.3](#83-third-party-data--ip)).

---

## 3. Personas & Tiers

### 3.1 Personas
- **The Active Retail Trader** — trades daily, follows WSB/StockTwits, wants a fast gut-check: "is this move real, or am I chasing hype?"
- **The Self-Directed Investor** — swing-trades a watchlist, wants divergence alerts and historical context before committing.
- **The Power User / Quant** — builds bots and scanners, wants API/MCP access and direct query capability over the corpus.

### 3.2 Tiers

| Tier | Price model | Sentiment compute | Key capabilities |
|------|-------------|-------------------|------------------|
| **Free** | $0, capped daily usage | Server-side **FinBERT triage** | Base-rate lookups, limited conviction signals, hype heatmap |
| **Pro** | Monthly subscription | Hosted **Gemma** reasoning + optional **on-device Gemma** | Unlimited signals, full conviction scoring, watchlist alerts, drill-down verification |
| **Power / Quant** | Subscription + metered credits | All of Pro | REST API, MCP server, direct corpus query, factor-match scanner at scale |

The tiering deliberately mirrors a proven credit/tier model: free for life with caps to drive adoption, Pro for unlimited analysis, Power/Quant for programmatic access. **On-device Gemma is the cost lever** — Pro users with capable GPUs offload reasoning compute to their own machines, lowering our server footprint while improving their latency and privacy.

---

## 4. Product Phases

Build order is **inward-out**: the engine first, then trust, then discovery, then delight. Rationale: the conviction score improves every downstream surface, so it must exist and be trustworthy before discovery/NL features add value.

| Phase | Theme | Depth in this doc |
|-------|-------|-------------------|
| **Phase 0** | Historical base-rate corpus + SaaS skeleton (auth, billing, gateway) | **Deep** |
| **Phase 1** | Conviction engine: ingestion + sentiment cascade + 3-leg fusion | **Deep** |
| **Phase 2** | Trust dashboard, Factor-Match scanner, crypto/alternative assets | Outline |
| **Phase 3** | Natural-language base-rate queries, on-device Pro mode GA | Outline |

---

## 5. Features

### 5.1 The Base-Rate Engine (owned corpus) — Phase 0

**What:** A deterministic quant engine that, for any (symbol, day), computes a **factor fingerprint** and looks up **what happened next** across 30 years of history.

**Why it's owned, not borrowed:** Base-rate facts (RSI, VIX zone, forward return for a symbol on a past date) are deterministic and idempotent — they never change. We compute them once from **raw inputs** (institutional OHLCV + VIX) and ship the SaaS **pre-loaded** with the full corpus. We do **not** scrape or redistribute any competitor's computed dataset; we may use a competitor's API only as a **validation benchmark** to confirm our numbers match (see [§8.3](#83-third-party-data--ip)).

**Factor fingerprint (Phase 0 set):**
- Move intensity (ATR-normalized daily change)
- VIX level and VIX move
- RSI zone and RSI slope
- Market trend / regime
- Relative volume
- Price streaks (consecutive up/down days)
- Earnings proximity (where data exists)

**User-facing capabilities:**
- **Single-ticker base rate** — "Given today's setup for AAPL, across N historical matches it closed higher X% of the time over the next 1d / 5d / 20d. Median return: Y%."
- **Verifiability** — every matching historical day is individually inspectable. No black box. Sample size always shown so the user judges sufficiency.

### 5.2 The Conviction Engine — Phase 1

**What:** The fusion layer that combines the three legs into a single, confidence-weighted signal.

**Inputs:**
- **Crowd leg** — sentiment polarity, velocity (dPolarity/dt), acceleration (d²Polarity/dt²) from the sentiment cascade.
- **Reality leg** — fundamentals (P/E, EPS, debt), plus Phase-1 additions: **options flow** and **SEC EDGAR** (insider Form 4, 13F, 8-K).
- **History leg** — base rate (win %, median return, sample size) from §5.1.

**Output:** A conviction-scored signal — e.g., `ShortTarget` / `BuyTarget` / `NoSignal` with a confidence score. **Sample size gates confidence:** a base rate from 8 historical matches yields low confidence regardless of how lopsided it is. The three legs aligning against the crowd yields high confidence.

**Verifiability:** Each signal shows *which legs fired and why* — the user sees the sentiment spike, the insider sale, and the historical fade that produced the score.

### 5.3 Sentiment Cascade — Phase 1

A **two-model cascade** optimized for cost-at-scale:
- **FinBERT-class triage** (cheap, CPU-batchable) handles the high-volume firehose and free/mass tier first-pass polarity.
- **Fine-tuned Gemma 4 + weekly Slang-LoRA** handles escalations: sarcasm, multi-ticker entanglement, structured reasoning. Hosted for thin clients; **shipped quantized on-device** for Pro.

Carries forward from prior research: emoji tokenizer, DPO continuous-training loop, the 20-check safety gate, and `model_version` / `lora_version` tagging on every score.

### 5.4 Trust Dashboard — Phase 2 (outline)
- Hype heatmap of the most active tickers (polarity × volume).
- Per-signal drill-down: the raw social posts, the fundamentals, and every matching historical day behind the score.
- "Base model vs. current LoRA" comparison toggle.

### 5.5 Factor-Match Scanner — Phase 2 (outline)
- Inverts the flow: instead of "is this hot ticker confirmed?", rank the whole universe by historical win rate under today's conditions. Surfaces names the crowd hasn't hyped yet.

### 5.6 Crypto & Alternative Assets — Phase 2 (outline)
- Extend the corpus + ingestion to crypto and alternative strategies once the equities/ETF core is proven.

### 5.7 Natural-Language Queries ("Ask") — Phase 3 (outline)
- Plain-English → base-rate query, answered by the hosted/on-device Gemma over the owned corpus. ("What does SPY do after a 2% drop on a VIX > 30 day?")

### 5.8 On-Device Pro Mode GA — Phase 3 (outline)
- General availability of quantized Gemma running on the Pro user's GPU, with a hosted fallback for incapable hardware.

---

## 6. Data Sources

ETR's edge grows with every quality source. Sources serve one of three **roles**: **Hype** (crowd leg), **Reality** (fundamentals leg), or **Quant** (base-rate leg). We build a **multi-vendor abstraction** so sources can be added/swapped without touching the engine.

| Source | Role | API availability | Effort | Phase |
|--------|------|------------------|--------|-------|
| Reddit | Hype | API (terms-restricted bulk) | Low | 1 |
| StockTwits | Hype | API (cashtag streams) | Low | 1 |
| X / Twitter | Hype | API v2 (cashtags; costly) | Med | 1 |
| **Options flow** (Unusual Whales, CBOE, ORATS, Polygon) | **Reality** | API | Med | **1 (priority)** |
| **SEC EDGAR** (Form 4 insider, 13F, 8-K) | **Reality** | Free API | Med | **1 (priority)** |
| OHLCV + VIX (Polygon, Databento, Tiingo, EODHD) | Quant | API | High (backfill) | 0 |
| Earnings / transcripts (Financial Modeling Prep) | Reality | API | Low | 1 |
| Short interest (Ortex, FINRA) | Reality | API/feed | Med | 1 |
| News (Benzinga, Polygon news, GDELT) | Reality/Hype | API (GDELT free) | Low | 1–2 |
| Discord | Hype | Bot per-server, no content API | High | 2 |
| Telegram | Hype | API (noisy, pump channels) | Med | 2 |
| Google Trends | Hype | Unofficial API | Low | 2 |
| TikTok / YouTube / Bluesky | Hype | Scrape / Data API / firehose | Med–High | 2 |
| Seeking Alpha | Reality (slow/quality) | Scrape/API | Med | 2 |

**Phase-1 priority rationale:** Options flow and SEC EDGAR are the highest-leverage *non-sentiment* additions. Insider selling into retail euphoria, or bearish options positioning against a bullish crowd, are textbook ETR divergences and sharpen the conviction score immediately.

---

## 7. Historical Backfill Policy

A deliberate, asymmetric policy — the single most important data decision in the product.

### 7.1 Quant / base-rate corpus → **full precompute, ships pre-loaded**
- Deterministic and idempotent: a factor or forward return for a past date never changes.
- Compute the entire 30-year corpus once; the SaaS launches with it loaded.
- The live engine only appends **today's fingerprint** each evening after close (a nightly single-day diff).
- This is the correct, intended design and matches how mature base-rate products operate.

### 7.2 Sentiment → **live-forward only, never frozen-backfilled**
- **Not idempotent:** a polarity score is only meaningful relative to the model + LoRA that produced it, and the model improves weekly. Backfilling with today's model would be invalidated by the next LoRA update.
- **Archives are asymmetric:** social history is shallow, gappy, or bulk-forbidden; it cannot align with a 30-year quant corpus.
- **Look-ahead risk:** 2026 slang scoring 2021 text injects vocabulary that didn't exist then, biasing backtests.
- **Policy:** Accumulate sentiment from launch forward. **Store raw text + engagement** (cheap, model-agnostic) so any window can be *re-scored* on demand. **Tag every score** with `model_version` + `lora_version`.

### 7.3 Optional bounded research backfill
- A one-time, bounded (≈2–3 years) sentiment backfill scored with a **frozen, versioned baseline model** (`baseline_v0`), stored **separately** and never mixed with live scores. Purpose: backtest the divergence thesis. Treated as a labeled experiment, not production data.

---

## 8. Compliance, Legal & Trust

### 8.1 Not advice
All outputs are historical frequencies and statistical context for informational/educational purposes — never buy/sell recommendations. Persistent disclaimer surfaced in-product, mirroring industry norms.

### 8.2 Verifiability over black-box
Trust is a feature. Every signal exposes its evidence: the raw posts, the fundamentals, and every matching historical day. Sample size is always shown.

### 8.3 Third-party data & IP
- We compute base-rate factors from **raw inputs we license** (OHLCV/VIX), not from any competitor's derived dataset.
- A competitor's API may be used **only as a validation benchmark**, never scraped, cached for resale, or redistributed.
- All ingestion respects each source's ToS and rate limits; sources with restrictive bulk terms are used within their permitted scope.

### 8.4 Privacy & security
Standard SaaS obligations: tenant isolation, encrypted secrets, OWASP Top 10 controls (detailed in the TDD). On-device Pro mode keeps the user's inference local, a privacy selling point.

---

## 9. Migration & Supersession

- This is a **greenfield rewrite**. No local-first single-binary code carries over as deployment.
- **Algorithms carried forward:** tier routing, emoji tokenizer, DPO/LoRA continuous training, the 20-check safety suite, arbitrage/divergence signal logic.
- Historical `Context/` plans are retained for reference and are superseded by this PRD and the companion TDD.

---

## 10. Success Metrics

| Category | Metric (illustrative) |
|----------|----------------------|
| Signal quality | Forward-return separation between high- vs low-conviction signals; precision of `ShortTarget`/`BuyTarget` vs T+5 outcome |
| Trust | % of signals where users open the drill-down; verification engagement |
| Engagement | DAU/WAU; watchlist alerts acted on; heatmap sessions |
| Conversion | Free → Pro conversion; Pro → Power/Quant; on-device Pro adoption |
| Cost | Server inference cost per active user (target: flat or declining as on-device adoption grows) |
| Corpus | Coverage (symbols × years); nightly diff success rate; base-rate parity vs validation benchmark |

---

## 11. Risks & Open Questions

### 11.1 Risks
- **Data licensing cost/terms** for OHLCV/options at corpus scale.
- **Social API access volatility** (X/Reddit pricing and terms shift frequently).
- **Model drift / safety** — LoRA updates degrading behavior; mitigated by the 20-check gate.
- **GPU cost** for hosted Gemma; mitigated by FinBERT triage + on-device Pro.
- **Regulatory perception** — must stay firmly on the "information, not advice" side.

### 11.2 Open questions (carried to delivery)
1. Final OHLCV/options vendor mix (multi-vendor; selection by coverage/cost/licensing).
2. Mobile framework (React Native recommended for web code reuse vs. Flutter for perf).
3. Tick store engine (ClickHouse recommended vs. TimescaleDB) — see TDD.
4. Exact conviction-score weighting and sample-size confidence curve (to be calibrated against the research backfill).

---

*See the companion [Technical Design Document](./ETR_SaaS_TDD_v1.md) for architecture, schemas, and technology choices.*
