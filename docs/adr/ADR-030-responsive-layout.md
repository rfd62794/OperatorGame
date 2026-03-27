# ADR-030 — Stitch Design System / Responsive Layout
> **Status:** Accepted | 2026-03-27

## Context
The initial "Stitch" UI implementation used manual `ui.horizontal` splits to create the navigation sidebar. This led to two critical issues:
1. **Vertical Truncation:** `ui.horizontal` shrink-wraps its contents vertically and does not naturally fill the available `CentralPanel` height, causing the sidebar to "float" or clip.
2. **Responsive Complexity:** Handling safe areas and breakpoints (Moto G vs. Desktop) required fragile manual layout math for every tab.

## Decision
1. **SidePanel Adoption:** Replaced manual layout with `egui::SidePanel::left`. This forces the sidebar to occupy the full available height and provides a clean separation of concerns for the navigation logic.
2. **600px Breakpoint:** Standardized the `Compact` vs. `Standard` layout toggle at 600px width.
3. **Card Grid Refactor:** Manifest and Mission views now use a responsive grid that adapts columns based on the available width within the `CentralPanel`.

## Rationale
`egui::SidePanel` is specifically designed for persistent navigation. By letting the framework manage the sidebar dimensions, we ensure that content areas (like the Roster list) always utilize the full remaining viewport. This also simplifies safe-area calculations on mobile, as `CentralPanel` automatically accounts for the panels that surround it.

## Consequences
- **Positive:** Sidebar always fills the vertical axis.
- **Positive:** UI navigation is now a first-class citizen in the layout pass.
- **Negative:** Increased indentation depth in `mod.rs` due to nested panel closures (to be addressed in Sprint G.2 refactor).
