# FinBERT Research Upgrades & Modernization Strategy

As the foundation of our "Eat The Rich" sentiment pipeline, FinBERT is an excellent domain-specific starting point for classifying financial text. However, to capture the extreme variance, sarcasm, and nuanced signaling found in modern retail trading forums (like r/wallstreetbets or StockTwits), we must radically upgrade the NLP pipeline. 

This document outlines strategies for improving, augmenting, or replacing FinBERT with modern Small/Medium Language Models (SLMs/LLMs), algorithmic tokenization, and dynamic reinforcement learning loops.

---

### 1. Modern Model Replacements & Hardware Strategy
Using newer, more powerful foundations than the original BERT architecture, scaled to fit diverse trader environments.

#### A. The Gemma & Phi Strategy (Hardware-Tiered Deployment)
FinBERT (based on BERT) excels at formal language but fails to understand retail forum slang (e.g., "diamond hands", emojis, deep sarcasm). Upgrading to models like **Gemma 4** or **Phi-3** allows us to output structured reasoning constraints rather than static classifications.

Because retail, high-net-worth, and institutional traders utilize computing environments of vastly different scales, the model deployment is tiered:

- **Tier 1: Medium Consumer (8GB - 12GB VRAM | e.g., RTX 3060 / 4060)**
  - **Models:** Gemma 2B, Phi-3 Mini (3.8B).
  - **Format:** 4-bit or 8-bit quantized. 
  - **Use Case:** High-speed, basic polarity scoring. Can run easily on standard gaming laptops or mid-tier desktops alongside trading platforms.

- **Tier 2: High-End Desktop / Power User (24GB VRAM | e.g., RTX 3090 / 4090)**
  - **Models:** Gemma 4 9B, Llama 3 8B.
  - **Format:** 8-bit or 16-bit.
  - **Use Case:** Advanced sarcasm detection and complex reasoning output. Provides highly nuanced trading signals with acceptable latency for algorithmic execution on enthusiast-grade home servers.

- **Tier 3: Institutional / Pro-Workstation (48GB+ VRAM | e.g., RTX 6000 Ada, Multi-GPU)**
  - **Models:** Gemma 4 27B-31B (Dense or MoE).
  - **Format:** Pure 16-bit or fine-tuned FP8 MoE architectures.
  - **Use Case:** "The Oracle." Full multi-agent deliberation, comprehensive fundamental + sentiment cross-referencing, zero compression degradation. Designed for elite, latency-insensitive strategy evaluation.

#### B. Phased Development & Continuous Training
To continually improve the MLP (Multi-Layer Perceptron) processing and logic reasoning, we will implement a rolling, three-phase continuous training loop:

1. **Phase 1: Foundation SFT (Supervised Fine-Tuning):** We initially tune the base model on a high-quality, human-labeled set of Reddit/Stocktwits data to establish core sentiment boundaries and output formatting (e.g., structured JSON).
2. **Phase 2: Reinforcement Learning (RL):** We connect the model's classifications to actual forward-looking stock performance metrics (reward signals) to optimize its predictive weight dynamically.
3. **Phase 3: Continuous LoRA Adaptations:** Market slang shifts rapidly (e.g., a new meme stock generates entirely new vocabulary). We will train localized Low-Rank Adapters (LoRA) on a 7-day rolling basis to hot-swap "meme-regime" weights into the base model instantly without full redeployment.

#### C. Evaluation: DPO vs. PPO for Financial RL
To achieve Phase 2, we must choose between the standard Proximal Policy Optimization (PPO) and Direct Preference Optimization (DPO). 

**PPO (Proximal Policy Optimization)**
- **How it works:** Requires training a separate "Reward Model" alongside the main Policy/Actor model. The reward model scores the outputs during active training.
- **Pros:** Highly flexible. Excellent for open-ended trading simulations where environments are complex and non-linear.
- **Cons:** Extremely VRAM intensive. PPO requires loading up to 4 distinct models simultaneously during training (Actor, Critic, Reward, Reference). Notoriously unstable to train, often suffering from "reward hacking" (where the model finds a loophole in the math rather than learning trading wisdom).

**DPO (Direct Preference Optimization)**
- **How it works:** Removes the separate reward model entirely. Instead, it relies on strict datasets of "Preferred" vs. "Rejected" outputs. The math directly updates the policy to maximize the margin between the two. 
- **Pros:** *Much* faster and stable. Requires exactly half the VRAM of PPO since it only loads the Policy and Reference models. It is perfectly suited for retrospective market alignment: we pair a successful predictive sentiment (Preferred) against an incorrect prediction (Rejected) based on actual T+1 Day stock price validation.
- **Cons:** Less flexible for dynamic environment exploration, requiring rigorous and strictly structured pair-generation pipelines offline.

**Verdict:** We will utilize **DPO** (or GRPO for MoE architectures) because it is highly stable, computationally lightweight for local rigs, and directly leverages the historical "ground truth" of the stock market as empirical preference pairs.

---

### 2. Algorithmic & Tokenizer Upgrades
Transforming how the model literally ingests financial culture.

#### A. Gemma-Optimized Visual Tokenization
In modern retail trading, emojis carry vastly more weight than text (e.g., "GME 🚀" contains no financial verbs but signals extreme momentum).
- Standard tokenizers (like SentencePiece used in Gemma) often fracture complex Unicode emojis into 3 or 4 meaningless byte-fallback tokens, stripping their semantic density.
- **Upgrade Path:** We will manually add high-impact retail emojis (🚀, 💎, 🙌, 🐻, 🐂, 📉) as discrete, single tokens in the Gemma vocabulary.
- We will then resize the model's embedding matrix so the model immediately maps these symbols to distinct, heavy semantic tensors. 

#### B. Contrastive Learning for Stance Detection
Instead of basic classification, we mathematically optimize the embedding space. Using **Supervised Contrastive Learning (SupCon)**:
- We train the algorithm to cluster sentiments tightly. This drastically reduces false positives where the model confuses "bad fear" (bearish) with "FOMO fear" (bullish).

---

### 3. Complementary Analysis Mechanisms
For the final Hype-O-Meter, the model must sit within an ensemble:

1. **Joint Topic-Emotion Modeling (BERTopic):** Clusters posts first (e.g., "CEO Scandal" vs "Earnings Beat") to separate *why* sentiment is happening from *what* the sentiment is.
2. **Fine-Grained Emotion Detection:** Differentiating between structural Anger ("Fraud!") and structural Fear ("Crash selloff").
3. **Domain-Specific Named Entity Recognition (NER):** Slicing merged sentiment accurately (e.g., grading positive for Tesla but negative for Rivian within the same post axis).

---

### Summary of Forward Architecture
The NLP Engine will evolve into a **Composite Analytics Pipeline**:
1. Intercept stream text, apply Gemma-optimized emoji mapping.
2. Run lightweight NER to isolate targets.
3. Feed targets to a Hardware-Tiered Gemma/Phi model, loaded with the most recent weekly LoRA adapter.
4. The model continuously improves weekly using purely retrospective DPO paired datasets derived from live stock resolution.
