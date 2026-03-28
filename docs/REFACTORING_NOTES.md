# Refactoring Notes (Phase 0)

## ✅ Module Fragmentation (Completed)
`src/models.rs` (God Module) has been successfully split into a modular directory structure:
- `src/models/operator.rs`: Lifecycle and stats.
- `src/models/mission.rs`: Mission logic and resolution.
- `src/models/item.rs`: Gear and Hats.
- `src/models/expedition.rs`: World-map expeditions.
- `src/models/log.rs`: Stub for G.4 Combat Logs.
- `src/models/mod.rs`: Re-exports (preserves API compatibility).

## 🛑 Logic Leakage Identified (Debt for Phase B)
The following methods in `src/persistence.rs` perform game-rule mutations on the `GameState` struct. These should eventually be moved to a dedicated `src/logic/` or `src/engine/` module:

| Method in `persistence.rs` | Current Responsibility | Proposed Phase B Home |
| :--- | :--- | :--- |
| `purchase_hat` | Validates bank, adds to inventory. | `src/models/item.rs` or `logic.rs` |
| `equip_hat` | Updates operator equipment state. | `src/models/operator.rs` or `logic.rs` |
| `award_xp` | XP delta calculation and stat growth. | `src/models/operator.rs` (logic exists there, but triggered from persistence) |
| `resolve_expedition_reward` | Mutates inventory based on outcome. | `src/models/expedition.rs` |

## 🚀 Guardrails Maintained
- **Behavior**: Zero logic changes during the move.
- **Compatibility**: `SAVE_VERSION` is unchanged; existing `save.json` files remain compatible.
- **Tests**: All unit tests in `models` and `persistence` passing.
