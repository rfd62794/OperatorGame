# ADR-040: Flutter Migration — Evaluated, Deferred

**Status:** Deferred (revisit after G.8 Breeding)  
**Date:** March 2026  
**Context:** Recurring egui layout frustration during SDD-038 implementation  
**Deciders:** Designer (Robert)  

---

## Context

During the SDD-038 UI Architecture implementation sprint, repeated layout failures with egui's closure-based widget system prompted a serious evaluation of migrating the UI layer to Flutter.

The specific pain points that triggered this evaluation:

- `ui.columns(2)` silently failing to propagate mutable variable changes across closure boundaries
- No visual editor — every iteration requires a full Android build and deploy (~2-3 minutes)
- egui's immediate mode model makes "left content, right buttons on same row" surprisingly complex
- Multiple agents attempted the same pattern in different ways and all produced invisible buttons
- The layout model assumes developers can reason about egui's allocation order, which requires deep familiarity with the library

---

## What Was Evaluated

**Flutter + flutter_rust_bridge architecture:**

The proposed split:
- **Keep in Rust:** All game logic — genetics, breeding, mission resolution, D20 combat, progression, persistence, GameState
- **Move to Flutter:** All UI — roster cards, mission board, map view, AAR panel, Quartermaster, navigation

The connection layer: `flutter_rust_bridge` auto-generates Dart bindings for Rust functions. Dart code calls Rust logic directly with type safety.

**Flutter advantages for this project:**
- `Row(children: [Expanded(child: Text(name)), ElevatedButton(...)])` solves the STAGE button problem in 3 lines
- Hot reload on device — see layout changes in seconds, not minutes
- Dart is readable coming from Python background
- `ListView.builder` handles scrollable card lists natively
- `CustomPainter` supports the Beastie Bay DX squad track visual goal
- Single codebase covers Android, iOS, desktop, web

**Estimated conversion effort:** 4-6 weeks to replace egui UI with Flutter UI while keeping all Rust game logic intact.

---

## Decision

**Defer.** Do not migrate now.

### Reasons for deferral:

1. **The loop is not yet complete.** G.7 (Colors), G.8 (Breeding) are the next two sprints. These add the genetic visual identity and the generational loop — the core depth the game needs. Converting the UI layer before the content layer is complete means converting twice.

2. **The immediate problem is solved.** The STAGE button is now visible and functional. The loop is playable. The pain was real but the blocker is removed.

3. **Flutter requires Dart knowledge.** The Coursera "Developing Mobile Apps with Flutter" specialization is the recommended learning path. Starting the course in parallel with G.7-G.8 content work means Flutter knowledge will be ready when the conversion makes sense.

4. **4-6 weeks is not a small investment** at the current pace. Spending that time on G.7-G.8 delivers more player-visible value than a UI framework swap that leaves the game identical from the player's perspective.

---

## Revisit Conditions

Revisit this decision after G.8 (Breeding) if any of the following are true:

1. **The squad track visual goal becomes near-term.** Beastie Bay DX style squad movement along dungeon paths is structurally difficult in egui. If this becomes G.9 or G.10 work, Flutter becomes the clear choice.

2. **Another egui layout session takes more than 2 hours without resolution.** Today's session took significantly longer. A second occurrence of the same pattern is a signal to switch.

3. **Desktop tool development becomes a priority.** Flutter's desktop support is excellent. If rfditservices.com tools or OpenAgent gets a UI, Flutter is the better choice than egui for that too.

4. **The game needs iOS.** egui/eframe has limited iOS support. Flutter is first-class on iOS.

---

## Learning Path (Parallel, Non-Blocking)

While G.7-G.8 proceed in egui, begin Flutter learning:

- **Course:** "Developing Mobile Apps with Flutter" specialization on Coursera
- **Goal:** Functional Flutter knowledge by the time G.8 ships
- **Milestone:** A simple Flutter app calling a Rust function via flutter_rust_bridge running on the Moto G

This means the conversion, if it happens, starts from competence rather than learning-while-building.

---

## Notes

The core Rust game logic is architecture-agnostic and will not need to change regardless of which UI framework is chosen. The genetics system, mission resolution, persistence layer, world map — all of this is valuable work that transfers directly.

*RFD IT Services Ltd. | OperatorGame | ADR-040 | March 2026*
