# Sprint 02 — Dice Drama (Visual Reveal)

**Directive Type**: Implementation  
**Status**: Ready  
**Date**: 2026-03-24  

## 0. Goal
Inject high-tension visual storytelling into the mission resolution flow. The current text-only After-Action Report (AAR) is functional but lacks impact. This sprint introduces animated d20 rolls that physically reveal the mission results, creating a "tension beat" before the outcome is confirmed.

## 1. Scope
- **Dice Engine Integration**: Unify `src/dice.rs` (physics) with the `eframe` update loop.
- **Roll reveal**: Animate 1–3 d20s (matching squad size) when "PROCESS AAR" is clicked.
- **Tension Gating**: Delay the arrival of `pending_aar` results until the dice animation resolves.
- **Prominent Skip**: Provide a first-class "SKIP ANIMATION" button with the same visual weight as the action buttons to avoid forced-delay friction for power users.

## 2. Implementation Specs

### Task A: Animation Loop
- **File**: `src/ui/mod.rs`
- **Fields**: 
    - `dice_engine: Option<DiceEngine>`
    - `processing_deployment_id: Option<Uuid>`
    - `rng: SmallRng`
- **Hook**: In `App::update`, capture `dt = ctx.input(|i| i.stable_dt)` and call `engine.tick(dt, &mut self.rng)`.
- **Constraint**: Call `ctx.request_repaint()` while `dice_engine.is_some()` to maintain 60FPS.

### Task B: Dice Rendering (Painter)
- **File**: `src/ui/dice_render.rs` (New)
- **Method**: Use `egui::Painter` to draw 2D-projected hexagons for D20s.
- **Visuals**: 
    - **Physicality**: Apply `shake`, `squash/stretch`, and `y_offset` from `DieAnimState`.
    - **Reveal**: Show `display` value in the center. Change to `result` only in the Landing phase.
    - **Feedback**: Intensify glow and color on `is_crit()` (Green) or `is_fumble()` (Red).

### Task C: AAR Hook
- **File**: `src/ui/ops.rs`
- **Trigger**: "PROCESS AAR" initializes the engine and rolls.
- **Resolution**: `resolve_deployment()` is called only when `engine.is_resolved()` or "SKIP" is clicked.
- **UI**: The "SKIP ANIMATION" button must be visually prominent (same styling as "PROCESS AAR").

## 3. Verification Plan
- **Automated**: `src/ui/f1b_loop_tests.rs` must include a state-transition test verifying the `dice_engine` properly gates `pending_aar`.
- **Manual**: Verify d20 count matches squad size (1, 2, or 3) on the phone.
- **Manual**: Verify the stutter-decel phase builds tension effectively on physical hardware.
