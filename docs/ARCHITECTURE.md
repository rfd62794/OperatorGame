# OperatorGame Architecture Overview

## Repository Structure (`src/`)
- **`main.rs`**: Entry point (eframe/egui initialization).
- **`lib.rs`**: Module exports and top-level traits.
- **`platform.rs`**: Safe-area insets, mobile emulation logic, and layout constants.
- **`persistence.rs`**: Save/Load logic and game state migrations.
- **`models/`** (Modular Data Layer):
  - `operator.rs`: Tactical and biological state.
  - `mission.rs`: Mission resolution and rewards.
  - `item.rs`: Equipment (Hats, Gear).
  - `expedition.rs`: World map expeditions.
  - `log.rs`: Operational logs.
- **`ui/`** (UI Layer):
  - `mod.rs`: Main dashboard and routing.
  - `manifest.rs`: Roster card grid.
  - `ops.rs`: Active deployment tracking.
  - `radar.rs`: World map visualization.

## Event Flow
1. **Selection**: User stages operators for a mission.
2. **Launch**: `Deployment::start` creates a timed record.
3. **Resolution**: `resolve_deployment` performs D20 checks on the squad's aggregate stats.
4. **Log**: Post-mission narratives are generated and stored in a rolling 50-entry buffer.

## Mobile Emulation Loop
The project uses a high-velocity feedback loop for mobile-first development:
- `.\run_mobile.ps1`: 400x800 resolution, 2.0x DPI scaling, Android-standard safe areas.
- Target Loop Time: **<30 seconds** (edit -> compile -> verify -> close).
