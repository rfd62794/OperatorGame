# ADR-038: Modular Data Layer & Domain Segregation

## Status
Accepted

## Context
As of Sprint G.5, the `models.rs` file grew to over 1,500 lines, containing operators, missions, expeditions, items, and genetic logic. This created significant "blast radius" for any change, slowed down compilation, and made it difficult for agents to maintain context without exceeding token limits.

## Decision
We formally adopt a modular data layer structure under `src/models/`. Every major domain entity must reside in its own submodule.

### Directory Structure
```text
src/
├── models/
│   ├── mod.rs          # Central exports (pub use self::*)
│   ├── operator.rs     # Slime genome, stats, and state
│   ├── mission.rs      # Mission, Target, and AarOutcome
│   ├── item.rs         # Hats, Gear, and Item catalogues
│   ├── expedition.rs   # Persistent deployment state
│   └── log.rs          # CombatLog and LogEntry definitions
```

### Dependency Rules
1. **Upward Exports**: All modules must be exported through `models/mod.rs` so that the rest of the app can still use `crate::models::XYZ`.
2. **Horizontal References**: Modules may reference each other (e.g., `Mission` references `Operator`), but circular dependencies should be avoided by moving shared logic to the highest level or a dedicated utility module (e.g., `src/combat.rs`).

## Consequences
- **Improved Maintainability**: Changes to "Hats" no longer risk breaking "Mission" serialization if the file is handled atomically.
- **Agent Efficiency**: Agents can read only the relevant domain file (e.g., `mission.rs`) rather than the entire `models.rs`.
- **Compile Times**: Rust can better parallelize the compilation of secondary modules.
