# OPERATOR — Implementation Plan (SDD)
> **Version:** 1.0 | **Status:** Tier 1 + 2 Complete | 2026-03-04

## Overview
This document is the in-repository counterpart to the Spec-Driven Development plan. It records the technical decisions made during scaffolding and serves as the ground truth for any contributor or future agent picking up the project.

---

## Project Structure

```
OperatorGame/
├── Cargo.toml
├── CONSTITUTION.md          ← Governing principles (non-negotiable)
├── SPEC.md                  ← Feature specification (domain contracts)
├── docs/
│   ├── GDD.md               ← Game Design Document
│   └── adr/
│       ├── ADR-001-rust-stack.md
│       ├── ADR-002-timestamp-over-countdown.md
│       ├── ADR-003-atomic-save.md
│       └── ADR-004-success-formula.md
└── src/
    ├── lib.rs               ← Module declarations
    ├── main.rs              ← Tokio entry point + CLI dispatch
    ├── cli.rs               ← Clap command tree
    ├── models.rs            ← Domain types + unit tests
    └── persistence.rs       ← GameState save/load + unit tests
```

---

## Dependencies (Cargo.toml)

| Crate | Version | Features | Purpose |
|-------|---------|----------|---------|
| `serde` | 1 | derive | Serialisation macros |
| `serde_json` | 1 | — | JSON persistence |
| `chrono` | 0.4 | serde | UTC timestamp handling |
| `uuid` | 1 | v4, serde | Stable entity IDs |
| `rand` | 0.8 | small_rng | Mission resolution RNG |
| `clap` | 4 | derive | CLI parsing |
| `tokio` | 1 | full | Async runtime (Tier 3 ready) |

---

## Module Responsibilities

### `src/models.rs`
- **Owns:** All domain types — `Job`, `OperatorState`, `Operator`, `Mission`, `Deployment`, `AarOutcome`
- **Rule:** Zero I/O. Zero UI. No `use std::fs` permitted here.
- **Key methods:**
  - `Job::stat_bonus() → (u32, u32, u32)`
  - `Operator::effective_stats() → (u32, u32, u32)`
  - `Operator::tick_recovery()` — clears Injured state passively on launch
  - `Mission::calculate_success_rate(squad) → f64` — see ADR-004
  - `Deployment::start(mission, op_ids) → Deployment`
  - `Deployment::is_complete() → bool` — see ADR-002
  - `Deployment::resolve(mission, squad, rng) → AarOutcome`

### `src/persistence.rs`
- **Owns:** `GameState`, `load(path)`, `save(state, path)`, `PersistenceError`
- **Rule:** No game logic. If you are computing success rates in persistence.rs, it is wrong.
- **Atomic write:** `.json.tmp` → rename. See ADR-003.

### `src/cli.rs`
- **Owns:** Clap `Cli` struct and `Commands` enum only.
- **Rule:** No business logic. Argument parsing shapes only.

### `src/main.rs`
- **Owns:** Command dispatch, state threading, save-on-exit.
- **Rule:** Load → tick recovery → parse CLI → execute command → save. Every command path must call `save()` at the end.

---

## Tier Roadmap

| Tier | Name | Status | Deliverable |
|------|------|--------|-------------|
| 1 | Headlong (Math Prototype) | ✅ Complete | CLI tool, core formula verified |
| 2 | Persistence Layer | ✅ Complete | Hire/save/load roster cycle working |
| 3 | Egui Dashboard | ⬜ Planned | `src/ui.rs`, `eframe` entry point, War Room view |

---

## Test Coverage

| Module | Tests | Status |
|--------|-------|--------|
| `models` | 8 | ✅ All passing |
| `persistence` | 3 | ✅ All passing |
| `cli` | 0 | ⬜ Manual coverage via smoke test |
| **Total** | **11** | **11/11** |

---

## Accepted Risks

| Risk | Mitigation | ADR |
|------|-----------|-----|
| System clock manipulation | Single-player accepted; server validation for multiplayer | ADR-002 |
| `save.json.tmp` orphan on crash before rename | Harmless — valid `save.json` still intact | ADR-003 |
| Operator stat overflow (u32 + u32) | MVP stat caps (0–100 base) make overflow impossible in practice | — |
