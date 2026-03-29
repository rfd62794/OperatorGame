# ADR-044 — Flutter UI Parity Strategy
> **Status:** Draft | 2026-03-29

## Context
Initial Flutter shell prototypes for the **OperatorGame** focused on high-fidelity, speculative UI designs that diverged from the mechanical reality of the original Rust `egui` framework. This divergence increased bridge complexity and made it difficult for developers to reason about the shared game state.

## Decision: Adopt Functional Parity
We have decided to pivot the Flutter shell's UX strategy to **100% Functional Parity** with the native `egui` dashboard ("War Room").

### 1. Geometric Mirroring
The Flutter layout will strictly adhere to the following core constraints from the sovereign core:
- **Status Bar (Top)**: 40dp (Standardized telemetry for Bank, Scrap, and Operational Stress).
- **Sidebar (Left)**: 120dp (Vertical sub-tabs for contextual navigation).
- **Bottom Navigation**: 56dp (Main tab hierarchy: Roster, Ops, Map, Logs).

### 2. Functional Component Parity
Components like the **SlimeCard** must mirror the 5-row functional specification established in `manifest.rs`, including life-stage color coding and staging toggles, rather than adopting "modern" high-fidelity abstractions.

### 3. Unified Command Layer
All state mutations will be routed through a `UiCommand` enum on the bridge, matching the command structure already handled by the Rust `GameState` persistence logic.

## Rationale
Adopting parity reduces the cognitive load required to maintain two disparate UI paradigms. It ensures that the Flutter shell remains a direct, functional projection of the underlying Rust game engine, allowing developers to reuse their understanding of the `egui` layout in the mobile context.

## Consequences
- **UX Constraint**: Creative experimentation in the Flutter shell is now constrained by the requirement to provide a familiar "dashboard" experience to existing users.
- **Bridge Simplification**: The `SlimeView` DTO and `apply_ui_command` functions directly map to existing Rust models, leading to faster implementation cycles.
- **Consistency**: Updates to the core game mechanics can be surfaced in both UI frameworks simultaneously using the same command patterns.
