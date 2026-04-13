# Eat The Rich - Investigation 1
## How Market Sentiment Drives Stock Prices

### 1. The Core Theory: Behavioral Finance vs. Market Economics
Traditional market economics operates under the Efficient Market Hypothesis (EMH), suggesting that stock prices perfectly reflect all available information and intrinsic value. However, the reality of modern trading—especially with the rise of retail traders and social media—proves that psychological factors often override fundamentals.

**Key Behavioral Finance Concepts:**
- **Network Theory & Contagion (Sociology/Econophysics):** Ideas borrowed from epidemiology and sociology to model how financial panic or FOMO spreads through a retail network like a virus.
- **Swarm Intelligence & Ant Colony Optimization (Biology/Computer Science):** Just as ants leave pheromone trails for others to find food, early retail traders leave "DD (Due Diligence)" trails on Reddit. If a critical mass follows, the swarm alters the stock price, creating a self-fulfilling prophecy.
- **Affect Heuristic (Psychology):** Investors making decisions based on their current emotional state (fear, joy, anger) rather than cognitive evaluation.
- **Econophysics:** Applying statistical mechanics to financial markets to model crowd behavior and momentum as physical forces.
- **Irrational Exuberance & Panic:** Prices swing well beyond their fundamental values due to collective greed (FOMO - Fear Of Missing Out) or collective fear.
- **Herd Mentality:** Traders follow the actions of the majority, amplifying momentum. If a stock trends on Reddit, the sheer volume of "followers" can trigger massive price spikes regardless of the company's financial health (e.g., Gamestop, AMC).
- **Confirmation Bias:** Investors seek information that confirms their existing beliefs, creating echo chambers in finance forums that artificially sustain momentum. 

In a sentiment-driven paradigm, the "true" value of a stock is simply what the masses *believe* it is worth at any given millisecond.

---

### 2. Mathematical Modeling of Market Sentiment
To build an intensively analytical and math-heavy system, we must bridge the gap between qualitative text (social media posts) and quantitative signals (time-series trading parameters).

**A. NLP and Sentiment Scoring Pipeline**
1. **Lexical & Transformer-Based Processing:** We cannot rely on basic sentiment dictionaries (like VADER). Words like "crazy" or "sick" can be extremely bullish in retail forums but negative in traditional contexts. 
2. **FinBERT:** A pre-trained NLP model fine-tuned on financial text. We will use this (or a custom-trained LLM) to classify forum posts as Bullish, Bearish, or Neutral, outputting a probability distribution for each. *(Note: See `FinBERT_Research_upgrades.md` for advanced model upgrades and integration strategies).*

**B. Integration into Time-Series Models**
Once we have continuous sentiment scores for a given stock, we structure them as mathematical inputs:
- **Index Generation:** $S_{t} = \sum_{i=1}^{n} (w_i \cdot P_i)$ where $S_t$ is the sentiment score at time $t$, $w_i$ is the weight of the post (based on upvotes, user reputation, or volume), and $P_i$ is the polarity score [-1, 1].
- **Stochastic Calculus / Drift:** Market prices are often modeled using Geometric Brownian Motion: $dS = \mu S dt + \sigma S dW$. We can modify the "drift" ($\mu$) and "volatility" ($\sigma$) terms to dynamically adjust based on the derivative of our sentiment index, $\frac{dS_{sentiment}}{dt}$.
- **ARIMAX / Machine Learning:** Expanding AutoRegressive Integrated Moving Average (ARIMA) to include our sentiment index as an exogenous variable. Alternatively, using **LSTMs (Long Short-Term Memory)** neural networks, feeding both OHLCV (Open, High, Low, Close, Volume) data and Sentiment vectors into the network to predict next-day price boundaries.

---

### 3. Best Platforms for Financial Sentiment Scrapes
Based on the current landscape, the most potent sources of pure retail sentiment are:
1. **Reddit (r/wallstreetbets, r/stocks, r/pennystocks):** The absolute epicentre for meme-stock culture and high-risk momentum. This is the primary driver of retail "Hype" and "Hate".
2. **StockTwits:** Often called the "Twitter for Investors." Users tag posts with $TICKER, and it is built specifically for real-time financial sentiment. This is incredibly high-signal.
3. **Twitter / X (using Cashtags e.g., $AAPL):** Extreme volume but can be very noisy. Requires heavy filtering to remove bot spam.
4. **Seeking Alpha / Yahoo Finance Comments:** Slower moving, but reflects the sentiment of traditional retail and "boomer" investors.

---

### 4. Continuous Evaluation & Scraping Strategy
To achieve a "100% pure sentiment analyzer," we need a robust, real-time ingestion pipeline.

**Scraping Strategy:** (Note: API connections will require access keys which should be securely passed into the application via a `.env` file or a centralized secure configuration manager).
- **Reddit:** Since the Reddit API changes, we will use an official Developer API key for polling (or use third-party scraping networks like Apify to bypass endpoint limits) targeting hot threads and the "Daily Discussion" threads where sentiment changes by the minute.
- **StockTwits:** StockTwits offers a developer API that is highly localized to ticker sentiment. We can hook into their streams.
- **Real-Time Streaming vs. Batching:** For inter-day trading, sentiment must be evaluated on short intervals. We will build a cron-driven script that pulls the latest 1,000 mentions of a ticker every 10 minutes. As we optimize the infrastructure, we will shorten this polling interval to evaluate sentiment every 1 minute.

**The Output Metric / The "Hype-O-Meter":**
In addition to base polarity, the index will evaluate:
- **Sentiment Dispersion:** The standard deviation of the sentiment. Are people uniformly bullish, or is there massive polarization (high disagreement)? Polarization often precedes massive volatility spikes.
- **Mention Dominance:** The ticker's mention volume relative to the total mentions of all tickers on the platform.
- **Upvote/Karma Weighting:** A post with 10k upvotes from a 5-year-old account has exponentially higher weight than a post with 1 upvote from a newly created bot account.
We will calculate an ongoing *Sentiment Velocity* and *Sentiment Acceleration*. It’s not just about how many people are bullish; it’s about *how fast* the bullish volume is increasing.
- $Velocity = \Delta Sentiment / \Delta Time$
- $Acceleration = \Delta Velocity / \Delta Time$
Stocks crossing an acceleration threshold trigger an alert.

---

### 5. The Two-Layer Architecture Forward Plan
**Layer 1: The Sentiment Engine (Pure Emotion)**
- A continuous microservice that scrapes Reddit and StockTwits.
- Cleans data, tokenizes, and feeds into FinBERT.
- **Augmented NLP Stack:** In tandem with FinBERT, we will run **Joint Topic Modeling** (to detect *why* a stock is hyped, e.g., "earnings" vs "short squeeze"), **Named Entity Recognition (NER)** (to ensure the sentiment applies to the correct underlying asset), and **Stance Detection** (to determine if the text is in favor or against a specific market position).
- Outputs a normalized time-series database of "Hype vs Hate" metrics per ticker.

**Layer 2: The Fundamentals Overlay (The Arbitrage Machine)**
- When Layer 1 flags a ticker as "Maximum Hype" or "Maximum Hate", Layer 2 activates.
- It pulls P/E ratios, EPS growth, debt-to-equity, and recent SEC filings (via YFinance / Alpha Vantage).
- **The Trade Logic:**
  - *Scenario A:* High Hype + Terrible Fundamentals = Overbought. A potential short/put-option play once momentum breaks.
  - *Scenario B:* High Hate + Strong Fundamentals = Oversold/Undervalued. A potential buy/call-option play when fear subsides.
  - *Scenario C:* High Hype + Good Fundamentals = Ride the momentum.

### Next Steps & Review
Review these theoretical frameworks and mathematical strategies. Once you approve the direction, we can begin designing the actual architecture and code for the **Data Ingestion & Sentiment Scraping Layer**.
