# Repository Guide: OperatorGame Rust-to-Flutter Interop

This document establishes the source-of-truth mapping for **DartPro** to audit the Rust business logic during UI development.

## 1. Core Logic Locations (Rust)
The **OperatorGame** core handles all deterministic game systems. Use these paths for reference:

| Domain | Rust Path | Description |
| :--- | :--- | :--- |
| **Persistence** | `src/persistence.rs` | Logic for `load`/`save` of `save.json`. |
| **Operator Models** | `src/models/operator.rs` | Definitions of `Operator`, `SlimeState`, and XP math. |
| **Genetics** | `src/genetics.rs` | `SlimeGenome`, `LifeStage`, and culture wheel logic. |
| **Combat** | `src/combat/mod.rs` | Damage calculation and status effect processing. |
| **Items** | `src/models/item.rs` | Gear and Hat stat bonuses. |

## 2. The Bridge Boundary
The bridge acts as a translation layer. It does not contain game logic; it only flattens Rust structures into Dart-friendly View Models (DTOs).

- **Bridge Definitions**: `operator_game_flutter/rust/src/api/simple.rs`.
- **Codegen Target**: `operator_game_flutter/lib/src/rust/`.

## 3. Data Flow (Option A)
1. **Request**: UI (Dart) → calls Bridge (Dart) → calls FFI (C/Rust).
2. **Execution**: Bridge (Rust) → calls Core Logic (Rust).
3. **Mappig**: Core Result (Rust) → mapped to `SlimeView` (Rust) → returned to Dart.

## 4. Key Models for DartPro to Study
- **`operator::Operator`**: The main persistent unit. Note the `stat_xp` array (9 cultures).
- **`genetics::Culture`**: The 9-point wheel (Ember, Tundra, etc.).
- **`genetics::LifeStage`**: Maps Level -> Stage (Hatchling, Juvenile, etc.).
- **`persistence::GameState`**: The over-arching container for the roster and bank.

## 5. Coding Standards
- **Sovereign Rust**: Do NOT modify the core `src/` directory for UI needs.
- **DTO Only**: Use the bridge to provide pre-calculated values (e.g., `xp_progress: 0.75` instead of raw XP numbers).
