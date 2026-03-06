# OPERATOR — Implementation Plan (SDD)
> **Version:** 2.0 | **Status:** Tiers 1–3 + Sprint 1 complete | 2026-03-04
>
> This document is the **Engineering Bible's project map**. It records module responsibilities,
> current dependencies, decision references, and test coverage. If you are touching code, start here.

---

## Project Structure

```
OperatorGame/
├── Cargo.toml
├── docs/
│   ├── README.md                ← Navigation index (start here)
│   ├── GDD.md                   ← Design Bible (game feel, systems, tone)
│   ├── SPEC.md                  ← Engineering Bible (entity contracts, formulas)
│   ├── CONSTITUTION.md          ← Governing principles (non-negotiable)
│   ├── SLIME_RUST_BLUEPRINT.md  ← rpgCore → Rust transplant spec
│   ├── sdd/PLAN.md              ← This file (module map, dependencies)
│   └── adr/
│       ├── ADR-001-rust-stack.md
│       ├── ADR-002-timestamp-over-countdown.md
│       ├── ADR-003-atomic-save.md
│       └── ADR-004-success-formula.md
└── src/
    ├── lib.rs          ← Module declarations
    ├── main.rs         ← Entry point + CLI dispatch
    ├── cli.rs          ← Clap command tree (shapes only, no logic)
    ├── models.rs       ← Human operator domain types + tests
    ├── genetics.rs     ← Slime genome engine + breeding resolver + tests
    ├── persistence.rs  ← GameState save/load (atomic) + tests
    ├── log_engine.rs   ← AAR narrative generation + tests
    └── ui.rs           ← egui War Room dashboard
```

---

## Dependencies (`Cargo.toml`)

| Crate | Version | Features | Purpose |
|-------|---------|----------|---------|
| `serde` | 1 | derive | Serialisation macros |
| `serde_json` | 1 | — | JSON persistence |
| `chrono` | 0.4 | serde | UTC timestamps (ADR-002) |
| `uuid` | 1 | v4, serde | Stable entity IDs |
| `rand` | 0.8 | small_rng | Mission resolution + breeding RNG |
| `clap` | 4 | derive | CLI parsing |
| `tokio` | 1 | full | Async runtime (Tier 3+) |
| `eframe` | 0.27 | — | egui native window (Tier 3) |

---

## Module Responsibilities

### `src/models.rs`
- **Owns:** `Job`, `OperatorState`, `Operator`, `Mission`, `Deployment`, `AarOutcome`
- **Rule:** Zero I/O. Zero UI. No `use std::fs`.
- **Key functions:** `effective_stats()`, `calculate_success_rate()`, `Deployment::resolve()`, `seed_missions()`

### `src/genetics.rs`
- **Owns:** `Culture`, `CultureExpression`, `GeneticTier`, `LifeStage`, `SlimeGenome`, `BreedingResolver`, `generate_random()`
- **Rule:** Zero I/O. Zero UI. All RNG is caller-injected (`&mut impl Rng`). No hidden state.
- **Key functions:** `BreedingResolver::breed()`, `resolve_culture()`, `resolve_stats()`, `genetic_tier()`, `life_stage()`
- **Invariant:** `CultureExpression.0.iter().sum() ≈ 1.0` — enforced by `normalise()` on every write

### `src/persistence.rs`
- **Owns:** `GameState`, `load(path)`, `save(state, path)`, `PersistenceError`
- **Rule:** No game logic. If you compute success rates here, it is wrong.
- **Atomic write:** `.json.tmp` → `fs::rename` (ADR-003)
- **Backward compat:** `slimes` field uses `#[serde(default)]` — old saves deserialize cleanly

### `src/log_engine.rs`
- **Owns:** `MissionType`, `generate_narrative()`, `format_log_entry()`
- **Rule:** Pure functions only. No I/O. No state.
- **Template pools:** 5 per MissionType, `{op}` token for name injection

### `src/cli.rs`
- **Owns:** `Cli` struct and `Commands` enum. Parser shapes only.
- **Rule:** Zero business logic. One `parse_*` helper per custom type.

### `src/main.rs`
- **Owns:** `fn main()`, command dispatch, state threading.
- **Rule:** Load → tick_recovery → parse CLI → execute → save. Every path calls `save()` except `Gui` (which handles its own saves internally).

### `src/ui.rs`
- **Owns:** `OperatorApp` (eframe `App` impl), `run_gui(state, path)`
- **Panels:** Roster (left), Operations (centre), Contracts (right), Combat Log (resizable bottom), Launch bar (fixed bottom)
- **Polling:** `ctx.request_repaint_after(Duration::from_millis(100))` — no background threads

---

## Tier + Sprint Roadmap

| Milestone | Status | Key deliverable | Tests |
|-----------|--------|-----------------|-------|
| Tier 1: MVP CLI | ✅ | Core formula + CLI loop | 8 |
| Tier 2: Persistence | ✅ | Hire/save/load roster | +3 |
| Tier 3: War Room | ✅ | `eframe` three-column dashboard | 0 (visual) |
| Tier 3b: Story Engine | ✅ | AAR narrative + combat log panel | +3 |
| Sprint 1: Genetics | ✅ | `genetics.rs`, hatch/splice CLI, slimes persist | +9 |
| Sprint 2: D20 Combat | 🔄 | `combat.rs`, D20Roll, DC ladder, replace flat% | TBD |
| Sprint 3: Expedition | 🔄 | `world_gen.rs`, procedural dungeon floors | TBD |

---

## Test Coverage

| Module | Tests | Status |
|--------|-------|--------|
| `models` | 8 | ✅ |
| `persistence` | 3 | ✅ |
| `log_engine` | 3 | ✅ |
| `genetics` | 9 | ✅ |
| `cli` | 0 | manual smoke |
| `ui` | 0 | visual |
| **Total** | **23** | **23/23** |

---

## ADR Index

| ADR | Decision | Applies to |
|-----|---------|-----------|
| [001](../adr/ADR-001-rust-stack.md) | Rust over Python/Go | Entire codebase |
| [002](../adr/ADR-002-timestamp-over-countdown.md) | `completes_at` over active timers | `models.rs`, `persistence.rs` |
| [003](../adr/ADR-003-atomic-save.md) | `.tmp` → rename saves | `persistence.rs` |
| [004](../adr/ADR-004-success-formula.md) | Per-attribute scoring | `models.rs` |
| 005 *(pending)* | Culture hex-wheel genetics | `genetics.rs` |

---

## Accepted Risks

| Risk | Mitigation | ADR |
|------|-----------|-----|
| System clock manipulation | Single-player accepted; add server validation for multiplayer | ADR-002 |
| `save.json.tmp` orphan on crash | Valid `save.json` remains intact; orphan is harmless | ADR-003 |
| Stat u32 overflow | Caps (0–100 base) make overflow impossible in practice | — |
| `CultureExpression` precision drift | `normalise()` called after every blend; tested with ε < 1e-5 | — |
