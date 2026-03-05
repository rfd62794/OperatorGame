# OPERATOR — Project Constitution
> **Status:** LOCKED v1.0 | 2026-03-04

## 1. Governing Principles

| Principle | Mandate |
|-----------|---------|
| **KISS** | Every data structure must be explainable in one sentence. No lifetimes where a clone suffices at this stage. |
| **Correctness > Speed** | Use `serde_json` for persistence; Parquet / BinCode only when profiling proves a bottleneck. |
| **Real-Time Safety** | Timers are stored as UTC completion timestamps, NOT as countdown values. Closing the app must not corrupt state. |
| **No Raw `unwrap()`** | All `Result` / `Option` types use `?` or explicit `match`. `unwrap()` is forbidden in production paths. |
| **Separation of Concerns** | `models.rs` knows nothing about the UI. `ui/` knows nothing about persistence logic. |

## 2. Technology Constraints (Non-Negotiable)

| Layer | Choice | Locked? |
|-------|--------|---------|
| Language | Rust (stable, 2021 edition) | ✅ |
| CLI | `clap` v4 | ✅ |
| GUI (Tier 3) | `egui` / `eframe` | ✅ |
| Persistence | `serde` + `serde_json` | ✅ |
| Async Runtime | `tokio` (full features) | ✅ |
| Time | `chrono` (UTC only — no naïve datetimes) | ✅ |
| RNG | `rand` crate | ✅ |

## 3. Scope Boundaries (MVP)

**IN scope:**
- Operator roster (hire, view stats, check state)
- Mission pool (static definitions for MVP)
- Squad assembly (1–3 operators)
- Timestamp-based mission execution (offline-safe)
- After-Action Report (success / injury / death outcomes)
- JSON save file (`save.json`) for roster + bank

**OUT of scope (post-MVP):**
- Procedural mission generation
- Multiplayer / networking
- Economy balancing / loot tables (placeholder only)
- egui dashboard (Tier 3 — Phase 2 milestone)

## 4. Auditability
All major decisions will be logged as ADRs in `docs/adr/`.
No feature merges without a corresponding ADR or spec update.
