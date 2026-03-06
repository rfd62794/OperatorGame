# SDD-006: Atomic Persistence & State Serialization

## 1. Overview
OPERATOR relies on a rugged, crash-proof persistence layer. To support the "double-tap" APK pipeline and sudden mobile session terminations, the game state must be serialized into a single `session.json` file.

## 2. Portable Save Paths
The relative path calculation binds `save.json` to the directory containing the executable rather than the Current Working Directory (CWD).
*   `std::env::current_exe()` ensures that whether the game is launched via terminal, desktop shortcut, or eventually an APK intent, the `session.json` lives exactly next to the binary.

## 3. The Unified `GameState` Struct
The entire world is held in a single struct that implements `serde::Serialize` and `serde::Deserialize`.

```rust
pub struct GameState {
    pub bank: u32,                  // The $100 Seed Money / Biomass reserve
    pub slimes: Vec<SlimeGenome>,   // The Unified Roster
    pub inventory: Inventory,       // Global resource storage (Scrap, Reagents)
    pub missions: Vec<Mission>,     // Persistent mission generation
    pub deployments: Vec<Deployment>,// Operations in progress when saved
}
```

## 4. The `Inventory` Sub-Struct
The `Inventory` operates as the central ledger for all non-currency resources (primarily Scrap/Metal) under the new Dual-Currency loop.
*   The `Inventory` is serialized directly into the root `GameState`.
*   As the Tech Tree expands, repairs pull values out of the `Inventory::metal` field, forcing the user to commit state changes via atomic writes.

## 5. Atomicity (The `.tmp` Rename)
The state is written to a temporary `.tmp` file and atomically renamed to `save.json` to prevent corruption if the game panics mid-write. This fulfills the "No Ghost States" operational requirement.
