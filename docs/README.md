# Eat The Rich — SaaS Architecture Docs

This folder holds the canonical product and technical specifications for the **greenfield SaaS rewrite** of Eat The Rich (ETR).

## Documents

| Doc | Purpose |
|-----|---------|
| [ETR_SaaS_PRD_v1.md](./ETR_SaaS_PRD_v1.md) | **Product Requirements** — vision, thesis, personas, tiers, feature roadmap, data-source strategy, backfill policy, success metrics, risks. |
| [ETR_SaaS_TDD_v1.md](./ETR_SaaS_TDD_v1.md) | **Technical Design** — modular-monolith architecture, Base-Rate Engine, sentiment cascade, conviction fusion, data layer, platform/infra, schemas, security. |

## Status & supersession

These documents define a **clean break** from the original local-first, single-binary design. They **supersede** the following historical plans, which are retained in [`../Context/`](../Context/) for reference only:

- `Context/ETR_PRD_Implementation_Plan_v2.md`
- `Context/ETR_PRD_Implementation_Plan_v3.md`
- `Context/FinBERTxGemma_PRD_Implementation_Plan_v1.md`
- `Context/FinBERT_Research_upgrades.md`
- `Context/eattherich_investigation1.md`

The **algorithms** described in those documents (hardware-tiered routing, emoji tokenization, DPO/LoRA continuous training, the 20-check safety suite, arbitrage signal logic) remain valid and are carried forward. The **deployment architecture** (local-first single binary) does not.

## Core thesis (one line)

> Detect when the retail crowd's sentiment diverges from both fundamental reality **and** the historical base rate of "what happened next" — and surface that divergence as a single, confidence-scored conviction signal across web, desktop, and mobile.

## Scope of v1 docs

- **Deep:** Phase 0 (historical base-rate corpus + SaaS skeleton) and Phase 1 (conviction engine).
- **Outline:** Phase 2 (trust UI, discovery scanner, crypto/alternatives) and Phase 3 (natural-language queries, on-device Pro mode GA).
