# SDD-050: War Room Dashboard Layout
> **Status:** Draft | 2026-03-29

## §1 — Goal
Implement a functional, responsive dashboard in the **OperatorGame Flutter Shell** that mirrors the **War Room** from the original `egui` framework. This design must provide absolute functional parity while ensuring 120fps reactive performance on mobile hardware.

## §2 — Geometric Specification (Desktop/Mobile)

The dashboard is structured around a central workspace constrained by three fixed-dimension bars.

```text
┌───────────────────────────────────────┐  ← Top Status Bar (40dp)
│ BANK: $500    SCRAP: 250kg    STRESS: 5% │
├─────┬─────────────────────────────────┤
│ (S) │                                 │
│ I   │                                 │
│ D   │                                 │  ← Center Content Area
│ E   │       Main Content Area         │    (GridView / PageView)
│ B   │                                 │
│ A   │                                 │
│ R   │                                 │
├─────┴─────────────────────────────────┤  ← Bottom Navigation (56dp)
│ ROSTER      OPS      MAP      LOGS    │
└───────────────────────────────────────┘
```
- **Top Status Bar**: `40dp`. High-contrast (Black) with HSL diagnostic telemetry.
- **Left Sidebar**: `120dp`. Vertical orientation for contextual sub-tabs (e.g., Active/Staging/Reserves).
- **Bottom Navigation**: `56dp`. Global tab navigation.

## §3 — Unified Command Layer (The Bridge)
Operational state mutations are routed through the `apply_ui_command(UiCommand)` function.

| Command | DTO Payload | Intent |
|---------|-------------|--------|
| `ToggleStage` | `id: String` | Toggles the mission-readiness of an operator. |
| `EquipHat` | `slime_id: String, hat_id: String` | Equips equipment from the sovereign catalog. |
| `LaunchMission` | `mission_id: String, operator_ids: Vec<String>` | Dispatches a squad to the world map. |
| `SyncState` | `None` | Forces a persistence check and state refresh. |

## §4 — Design System & Bio-Branding
The dashboard leverages the **Nonagon culture branding** to provide visual depth to the roster:
- **HSL Tokens**: Each culture (Ember, Tide, etc.) includes a saturated primary and a muted surface-low variant.
- **LifeStage Indicators**: Level-based color coding (Hatchling: Grey, Juvenile: Green, Prime: Gold, Elder: Purple).
- **Glassmorphism**: Backdrop blurs (Sigma 15.0) are used for technical detail sheets to maintain roster context.

---
*RFD IT Services Ltd. | OperatorGame | SDD-050 War Room Dashboard | March 2026*
