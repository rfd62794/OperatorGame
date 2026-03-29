# ADR-041 — Flutter Migration Strategy (Basic vs. Beauty)
> **Status:** Accepted | 2026-03-29

## Context
The decision has been made to pivot from `egui` to **Flutter** for the mobile UI layer. To ensure a stable transition, we are adopting a staged implementation that prioritizes functional verification (the Rust bridge) before adding aesthetic polish.

## Decision: The Two-Pass Execution

### Pass 1: "Stitch Minimal" (Functional Baseline)
**Goal**: Verify the `flutter_rust_bridge` and layout stability on the Moto G target.
- **Layout**: Single-row header (Name + Buttons) which avoids all previous `egui` clipping issues.
- **Styling**: Minimalist "Stitch" aesthetic (dark background, culture-colored name).
- **Logic**: One-way state propagation from Rust `GameState` to Flutter `StatefulWidgets`.
- **Interaction**: STAGE and Chevron (>) buttons must trigger Rust commands.

### Pass 2: "High-Fidelity Vision" (Aesthetic Beauty)
**Goal**: Elevate the UI to a premium, modern mobile experience.
- **Elevation**: Material 3 elevated cards with 16dp rounding.
- **Gradients**: Culture-specific background glows/gradients.
- **Micro-Animations**: Pulse effects on "Staged" cards; fluid "vial" XP bars.
- **Typography**: Expressive typographic hierarchy using modern sans-serif fonts (e.g., Inter or Outfit).

## Rationale
Building the "Beauty" phase first risks obfuscating bridge failures or performance bottlenecks. By establishing "Basic" as a stable baseline, we create a fallback state that is guaranteed to work on-device.

## Consequences
- **Positive**: The first Flutter deployment will solve the `egui` layout constraints (clipping/wrapping) immediately.
- **Positive**: Provides a clear separation between "Functional" (Rust/Dart bridge) and "Designer" (Flutter UI) concerns.
- **Immediate**: The developer and designer can work in parallel once the PoC is verified.
