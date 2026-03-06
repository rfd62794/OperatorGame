# ADR-001 — Technology Stack: Rust + Clap + Egui + Serde + Tokio
> **Status:** Accepted | 2026-03-04

## Context
OPERATOR is a dispatch simulator requiring a stateful game loop, persistent roster data, offline-safe mission timers, and a future GUI layer. The initial question was whether to use Python (existing AI toolchain familiarity) or Rust.

## Decision
**Rust (stable 2021 edition)** is the primary and sole language for the MVP.

| Layer | Choice | Rejected Alternatives |
|-------|--------|-----------------------|
| Language | Rust | Python, Go |
| CLI | `clap` v4 (derive macros) | `argh`, `structopt` |
| GUI (Tier 3) | `eframe` / `egui` | `iced`, `tauri`, `slint` |
| Persistence | `serde` + `serde_json` | SQLite, RON, BinCode |
| Async Runtime | `tokio` | `async-std`, `smol` |
| Time | `chrono` (UTC only) | `time`, `std::time` |
| RNG | `rand` 0.8 | `fastrand`, `nanorand` |

## Rationale
- **Rust** eliminates an entire class of state-mutation bugs (double-borrow of roster during deployment) at compile time. For a game where data correctness is the product, this is non-negotiable.
- **`clap` derive macros** produce self-documenting, type-safe CLI parsing with near-zero boilerplate. The `--help` is generated free.
- **`egui`** is immediate-mode GUI — the same "poll every frame" model that makes progress bars trivial. It bundles into a single binary with no runtime dependencies.
- **`serde_json`** keeps save files human-readable (git-diffable) and debuggable without tooling. A corrupt save should be fixable in a text editor.
- **`tokio`** is chosen for future-proofing: Tier 3 will require a repaint loop running concurrently with I/O. Adding async later would require a full rewrite; adding it now costs nothing.

## Consequences
- **Positive:** Single binary distribution, memory safety, fast compile-check feedback loop.
- **Negative:** `tokio` adds compile time overhead at Tier 1 where it's not yet exercised.
- **Accepted risk:** `chrono` UTC timestamps are vulnerable to system clock manipulation in single-player mode. Acceptable for MVP; server-side validation required for multiplayer.
