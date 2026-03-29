# ADR-039: Roster Card Header — Two-Row Layout

**Status:** Accepted  
**Date:** March 2026  
**Context:** Sprint SDD-038 UI Architecture Implementation  
**Deciders:** Designer (Robert)  

---

## Context

The roster operator card requires a header row showing the slime name on the left and STAGE + ▶ buttons on the right. This is a standard "left content, right actions" pattern common in mobile UI.

Multiple attempts over several sessions failed to render the buttons visibly on the Moto G:

### Approaches tried and why they failed

**Attempt 1 — `ui.horizontal` with `add_sized`:**
`add_sized([name_width, 18.0], Label)` allocates exactly the specified size but centers content by default and leaves no room for `with_layout(right_to_left)` to operate in the remaining space.

**Attempt 2 — `ui.columns(2, |cols| { ... })`:**
Appeared correct but `stage_clicked` and `card_clicked` are `bool` variables defined outside the Frame closure. The `ui.columns` closure is nested inside the Frame closure — two levels of closure nesting. Rust's borrow checker allowed the code to compile but the mutation of outer variables from inner closures was silently not propagating. The buttons rendered with zero height (invisible) on device.

**Attempt 3 — `ui.horizontal` with `with_layout(right_to_left)` inside:**
When `ui.horizontal` renders left-to-right and then a nested `with_layout(right_to_left)` block is added, egui allocates all remaining space to the right-to-left block but the left content has already consumed space greedily. Result: buttons rendered off-screen right.

**Root cause confirmed:**
The combination of `ui.set_width(380.0)` (forced exact width) inside a Frame with 8dp inner margin on each side means the actual available width is `380 - 16 = 364dp`. Any layout that doesn't account for this discrepancy overflows. Additionally, mutable variable capture across multiple levels of egui closures is unreliable in this version of egui (0.27.2).

---

## Decision

Split the header into **two separate rows** inside the card frame:

**Row 1a:** Name + Culture label — rendered with `ui.horizontal`  
**Row 1b:** Buttons (STAGE, ▶) — rendered with `ui.with_layout(right_to_left, TOP)`

This avoids:
- Nested closure mutation issues (each row is a separate closure)
- Width allocation competition between name and buttons
- The `add_sized` centering problem

The visual result is name on line 1, buttons on line 2 right-aligned. This is not the ideal single-line layout but is stable, functional, and does not regress.

---

## Consequences

**Positive:**
- STAGE button is visible and functional on Moto G
- No closure capture issues
- Layout is stable across card widths
- Future agents can follow this pattern safely

**Negative:**
- Header occupies two lines instead of one — cards are slightly taller than the SDD-038 §4 diagram shows
- The SDD-038 §4 diagram shows name and buttons on the same row — this implementation deviates from the diagram

**Deferred:**
- Single-row header (name left, buttons right on same line) is the desired end state
- This requires either: (a) Flutter migration which handles this trivially, or (b) a careful egui solution using `painter()` directly to draw button hit areas without closure nesting
- Marked as a future polish task, not a current blocker

---

## Notes for Future Agents

If you are attempting to put left-aligned text and right-aligned buttons on the **same row** in egui:

1. Do NOT use `ui.columns(2)` when the result variables (`stage_clicked`, `card_clicked`) are defined outside multiple levels of closures
2. Do NOT use `add_sized` followed by `with_layout(right_to_left)` — the sized allocation consumes space before the right block can claim it
3. DO use `ui.allocate_ui_with_layout` with an explicit rect if you need precise control
4. DO consider the Flutter migration path (ADR-040) if this pattern is causing repeated pain

*RFD IT Services Ltd. | OperatorGame | ADR-039 | March 2026*
