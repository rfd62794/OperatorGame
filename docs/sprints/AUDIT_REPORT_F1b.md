# AUDIT REPORT — Sprint F.1b Pre-Directive
date: 2026-03-24
agent: Antigravity (Google DeepMind)

---

## A. Add Slime Regression

- **file:** N/A
- **line:** N/A
- **what_renders:** Nothing renders. There is no "Add Slime" button, command
  string, debug snippet, or any other UI surface in the current code. The phrase
  "add slime" does not appear anywhere in `src/`. The phrase "spawn" appears
  only in `src/garden.rs` in the context of the background garden simulation —
  unrelated to roster management.
- **spawn_function:** None. No `add_slime`, `spawn_slime`, or equivalent
  function exists in `persistence.rs`, `recruitment.rs`, or anywhere in the
  codebase. The closest analogue is `crate::recruitment::purchase_recruit()`
  (called from `manifest.rs:153`) and `crate::recruitment::claim_elders_gift()`
  (called from `manifest.rs:127`) — both accessible from the **Breeding →
  Recruit** sub-panel (rendered by `render_recruit()` in
  `src/ui/manifest.rs:108`).
- **spawn_logic_intact:** partial — `purchase_recruit` and `claim_elders_gift`
  exist in `src/recruitment.rs` and are wired to the Roster → Breeding subtab.
  However the "Add Slime" flow referenced in the sprint spec does not exist as
  a named function or discrete debug path.
- **notes:** The regression description ("renders a command/debug text snippet")
  does not match the current HEAD. What the Roster → Breeding sub-tab
  (`RosterSubTab::Breeding`) actually renders is `render_incubator()`, which
  shows incubating genomes and a "Harvest" button, **not** a recruit/add button.
  The `render_recruit()` function is implemented (`manifest.rs:108`) but there
  is no sub-tab or entry point that routes to it from the current
  `render_sub_tabs()` function (`mod.rs:786`). The "Recruit" LeftTab enum
  variant (`mod.rs:79`) exists but is never set by any button in the sidebar,
  meaning the Recruitment Agency panel is unreachable from the UI. This is
  the root cause of the regression: the Recruit sub-tab was defined but never
  wired into `render_sub_tabs()` → the Roster sidebar only exposes "Collection"
  and "Breeding," not "Recruit." Adding a `sub_tab_button` for
  `RosterSubTab::Recruit` (which does not exist yet as an enum variant — only
  `LeftTab::Recruit` does) is the one-line-class fix required.

---

## B. AAR Result Visibility

- **aar_outcome_struct:** exists — `src/models.rs:444`
- **aar_fields:** `Victory { reward: u64, success_rate: f64, rolls: Vec<D20Result> }`,
  `Failure { injured_ids: Vec<Uuid>, rolls: Vec<D20Result> }`,
  `CriticalFailure { injured_ids: Vec<Uuid>, rolls: Vec<D20Result> }`.
  **No `xp_gained` field exists on any variant.**
- **ui_render_path:** partial/missing. The AAR result is *not* shown to the
  player in a dedicated result panel. After `resolve_deployment()` runs
  (`mod.rs:247`), the outcome is:
  1. Applied to bank / operator states silently.
  2. A one-line `status_msg` is set (e.g. `"✅ 'Bank Heist Recon' — VICTORY (+$500)."`).
  3. A narrative `log_entry` is prepended to the in-memory `combat_log` Vec.
  The player sees the outcome only as the `status_msg` label at the bottom of
  the screen, and as a colored line in the "COMBAT LOG" panel which is only
  visible when `active_tab == Missions`. There is no modal, overlay, or
  dedicated AAR result screen displaying Victory/Failure, individual rolls, or
  XP gained.
- **reachable_on_device:** no — The combat log panel (`mod.rs:513-550`) is
  gated on `self.active_tab == BottomTab::Missions`. If the player switches
  tabs before collecting the AAR, the log panel disappears. The Logs tab
  (`BottomTab::Logs`) reads from the same `combat_log` Vec, so entries are
  still accessible there, but only as plain coloured strings — no structured
  roll summary or XP figure.
- **collect_handler:** `src/ui/ops.rs:52` — the `"⚡ PROCESS AAR"` button
  triggers `self.resolve_deployment(dep.id)` (line 71). Timer expiry is
  detected inside `render_active_ops` by checking `progress >= 1.0` (progress
  is computed via `progress_for()`, a wall-clock comparison).
- **notes:** XP is awarded inside `resolve_deployment()` at `mod.rs:293` via
  `dep.award_squad_xp()`, but the amount per operator is discarded in the
  `for (id, _xp, leveled)` destructure at line 294 — `_xp` is intentionally
  ignored. A "LEVEL UP" message is pushed to `combat_log` if `leveled == true`,
  but no numeric XP figure is ever displayed. Sprint F.1b must (a) surface XP
  numbers in the AAR result, (b) add a structured result view, and (c) ensure
  it persists across tab switches (currently the `combat_log` Vec is not
  persisted to `save.json` — it lives only in `OperatorApp` RAM).

---

## C. Roster Card Current Fields

- **current_fields:**
  1. Operator name (bold, in culture color)
  2. Dominant culture name (top-right, small, culture color)
  3. Genome pattern (small, gray)
  4. Level number (`Lv: {N}`)
  5. XP progress bar (`op.total_xp % 100` as a fraction of 100, shown as
     percentage) — *see note below*
  6. STR stat (`op.total_stats().0`)
  7. AGI stat (`op.total_stats().1`)
  8. INT stat (`op.total_stats().2`)
  9. HP (hardcoded placeholder: `25` — see `manifest.rs:234`: `let hp = 25;
     // Placeholder for HP math (ADR-037 logic uses current/max)`)
  10. Status button: `INJURED` (disabled) / `DEPLOYED` (disabled) / `STAGE` or
      `✓ STAGED` (active)
- **level_field_exists:** yes — `Operator.level: u8` at `models.rs:114`
- **xp_field_exists:** yes — `Operator.total_xp: u32` at `models.rs:116`;
  rendered as a progress bar at `manifest.rs:215-216`
- **xp_to_level_exists:** derived — `Operator::xp_to_next()` at `models.rs:157`
  calls `LifeStage::xp_to_next(self.level)`. The XP bar on the card does NOT
  use this: it uses `op.total_xp % 100` as the numerator and `100` as the
  denominator (a fixed, incorrect formula that does not account for the actual
  XP thresholds). This is the bug to fix in F.1b.
- **card_render_fn:** `render_operator_card()` in `src/ui/manifest.rs:173`
  (free function, not a method). Called from `render_manifest()` at `manifest.rs:29`.
- **notes:** The XP bar formula `op.total_xp % 100 / 100` is incorrect. The
  correct formula using existing APIs would be something like
  `(total_xp - xp_at_current_level) / xp_to_next()`. The trivial fix is
  `op.total_xp as f32 / op.xp_to_next() as f32` clamped to `[0, 1]`, which
  would show absolute XP against the next level threshold.

---

## D. Node Map Alignment

- **position_method:** relative — node positions are stored as `(f32, f32)`
  offsets in logical pixels relative to a `(0.0, 0.0)` origin
  (`radial.rs:13`). Radii are `ring * 100 + 20` pixels (120 / 220 / 320 for
  rings 1–3). At draw time (`radar.rs:31-34`), positions are translated by
  `map_center` (the center of the allocated rect) and scaled:
  `pos.x = map_center.x + node.position.0 * scale`.
- **safe_area_applied:** no — The `render_radar()` function (`radar.rs:7`)
  draws directly into `ui.available_size()`, which is the content area *after*
  the sidebar separator. The `egui::CentralPanel` that wraps this content does
  apply the safe-area *margins* (`mod.rs:664-672`: `left: safe_area.left,
  right: safe_area.right`), but **not** `top` or `bottom`. The bottom-tab bar
  is drawn as a floating `egui::Area` on top of the central panel content,
  meaning map nodes at the bottom of ring 3 (y-offset ≈ +320 * scale) can
  be occluded by the 48dp bottom tab bar. Similarly the top status bar (a
  TopBottomPanel) consumes space but is not included in the CentralPanel
  top margin.
- **assumed_dimensions:** the scale formula is `map_dim / 600.0` where
  `map_dim = frame_size.x.min(frame_size.y)`. The map is designed assuming a
  600×600 logical-pixel square is available. Ring 3 reaches 320dp from center
  → outer diameter ≈ 640dp. On Moto G portrait, usable width after
  `ctx.set_pixels_per_point(2.0)` is `1080px / 2.0 = 540dp`. The map is
  already scaled down (`scale = 540/600 = 0.90`), placing ring 3 at
  `320 * 0.90 = 288dp` from center.
- **moto_g_safe_area:** The safe area fallback (`platform.rs:37-43`) is
  `top: 48.0, bottom: 56.0, left: 0.0, right: 0.0`. The JNI live-inset call
  is a **TODO** stub (`platform.rs:160-161`); actual Moto G values are not
  read at runtime. With the bottom tab bar at 48dp + safe-area bottom at 56dp,
  the bottom 104dp of the screen is consumed by chrome. The map center is
  placed at the center of the *remaining* vertical space, which may not be
  vertically centered relative to the *visual* map area — this creates the
  visual misalignment.
- **broken_transform:** The `bottom_tab_rect()` positions the tab bar at
  `origin.y + screen_height` (`platform.rs:133`), i.e. at the bottom of the
  safe rect. However the `CentralPanel` does not subtract 48dp (tab bar height)
  from its available height. The map's `frame_size.y` therefore includes the
  area under the tab bar, making the map taller than it appears, and pushing
  its visual center upward — nodes in the lower rings appear clipped or
  overlapped by the tab bar.
- **notes:** The tab bar height constant (`48.0` at `platform.rs:129`) should
  be subtracted from the central panel's effective height, or the map's
  available-size calculation should account for it. Safe area for Moto G is a
  hardcoded stub that may not reflect actual device insets (notch, gesture nav
  vs 3-button, etc.).

---

## E. Slime Detail View

- **tap_handler_exists:** no — `render_operator_card()` returns a `bool`
  indicating whether the `[STAGE]` button was clicked. There is no separate tap
  or click handler on the card frame itself; no `interact(..., Sense::click())`
  is applied to the card's outer `egui::Frame`. Clicking a card body (not the
  button) does nothing.
- **detail_view_exists:** no — There is no detail panel, modal, side panel,
  or route to a detail view anywhere in the codebase. The `selected_slime_id`
  field exists on `OperatorApp` (`mod.rs:59`) and is set by the
  (currently-disabled, commented-out) garden click handler (`mod.rs:441-447`),
  but it is not consumed by any render path — reading `selected_slime_id` does
  not open any view.
- **available_stats:** From `Operator` (models.rs) and derivable fields:
  - `genome.name`, `genome.id`
  - `genome.dominant_culture()`, `genome.pattern`
  - `genome.genetic_tier()`, `op.life_stage()`
  - `level`, `total_xp`, `xp_to_next()`
  - `stat_xp: [u32; 9]` (per-culture XP pools)
  - `op.total_stats()` → `(STR, AGI, INT, MND, Sensory, Tenacity)`
  - `genome.base_hp`, `genome.base_atk`, `genome.base_spd` (via
    `SlimeProfileCard::from_operator()` in `world_map.rs:743`)
  - `state` (Idle / Deployed / Injured / Training)
  - `equipped_gear: Vec<Gear>`
  - `synthesis_cooldown_until`
- **notes:** `SlimeProfileCard` (`world_map.rs:719`) is a pre-built display
  struct with `name`, `id_short`, `dominant`, `tier_label`, `stage_label`,
  `hp`, `atk`, `spd`, `accent_color`, `status`, and `cooldown_secs`. It is
  never rendered by any current UI path. Sprint F.1b can use it as the data
  source for the new detail view without additional modeling work.

---

## F. Logs Tab

- **current_render:** partial — `LogsSubTab::MissionHistory` renders the
  in-memory `combat_log: Vec<String>` as colored text lines
  (`mod.rs:712-723`). Color is determined by string content: green if
  `contains("VICTORY")`, red if `contains("CRITICAL")`, yellow otherwise.
  `LogsSubTab::CultureHistory` renders the static placeholder string
  `"Awaiting deployment and culture synchronization..."` (`mod.rs:725`).
- **log_field_name:** `combat_log` — this is a field on `OperatorApp`
  (the egui app struct, `mod.rs:55`), **not** on `GameState`. It is **not
  serialized to `save.json`** — it is lost on every app restart.
- **log_entry_type:** `String` — each entry in `combat_log` is a plain
  formatted string produced by `format_log_entry(&mission.name, &outcome,
  &narrative)` from `src/log_engine.rs` (called at `mod.rs:278`).
- **color_data_present:** no — entries are plain strings; color is derived
  by keyword search at render time, not stored in the log entry itself.
- **cap_enforced:** yes — `if self.combat_log.len() > 50 {
  self.combat_log.truncate(50); }` at `mod.rs:280`. Cap is 50 entries.
- **notes:** Because `combat_log` lives only in RAM (not in `GameState`), the
  Logs tab is empty on every cold start. Sprint F.1b must decide whether to
  (a) move log storage into `GameState` (persist to `save.json`), or (b) at
  minimum surface a "No log entries — mission history resets on app restart"
  placeholder. The `log_engine.rs` module (`src/log_engine.rs`, 8.7 KB)
  contains `format_log_entry` and `generate_narrative` — these are already
  functional. The data pipeline works; persistence is missing.

---

## Regressions Found (additional issues noticed during audit)

1. **HP on roster card is a hardcoded placeholder (`25`).**
   `manifest.rs:234`: `let hp = 25; // Placeholder for HP math (ADR-037 logic
   uses current/max)`. The actual base HP is `op.genome.base_hp` (a `f32`).
   No current/max HP tracking exists — this is a known gap, not a regression
   per se, but it misleads the player.

2. **XP bar uses incorrect formula.** `manifest.rs:215`:
   `let xp_pct = (op.total_xp % 100) as f32 / 100.0;` — this cycles between
   0–99% based on total_xp modulo 100, ignoring the actual per-level XP
   thresholds defined in `LifeStage::xp_to_next()`. Trivially fixable.

3. **Recruit sub-tab is unreachable.** `LeftTab::Recruit` enum variant exists
   (`mod.rs:79`) and `render_recruit()` is implemented (`manifest.rs:108`),
   but there is no sub-tab button in `render_sub_tabs()` that sets
   `roster_sub_tab` to a "Recruit" variant (which doesn't exist on
   `RosterSubTab` — only `Collection` and `Breeding` are defined). This is
   the root of the "Add Slime" regression.

4. **`combat_log` not persisted.** The log is cleared on every app restart
   (see Task F). The Logs tab appears empty to any player who exits and
   re-enters the app.

5. **`apply_daily_upkeep` contains a `println!("DEBUG: ...")` statement**
   (`persistence.rs:232`). This is dead debug output that should be removed
   before any public build.

6. **Garden background simulation is entirely commented out.** (`mod.rs:428-458`):
   `// Background Garden (Temporarily disabled due to UI layering issues)`.
   The `Garden` struct is still instantiated and ticked by dead code paths.

7. **`selected_slime_id` is set but never consumed.** `OperatorApp.selected_slime_id`
   (`mod.rs:59`) and the garden click handler that sets it are both inert.

8. **Map tab: only one sub-tab ("Zones") exists.** The sidebar for `BottomTab::Map`
   only renders a "Zones" button. No dispatch or expedition sub-tab is
   present, even though the expedition system is implemented.

---

## Readiness for F.1b

- **blockers:**
  1. No `RosterSubTab::Recruit` variant exists — adding the recruit flow
     requires both a new enum variant in `platform.rs` *and* wiring it in
     `render_sub_tabs()`. This is the Add Slime root fix.
  2. `AarOutcome` has no `xp_gained` field — surfacing XP in the AAR result
     view requires either adding the field or passing XP through the result
     chain separately.
  3. `combat_log` is not in `GameState` — the Logs tab cannot persist data
     across restarts without moving the log store to `GameState` and adding it
     to `save.json` serialization.
  4. No detail view scaffold exists — F.1b must create the tap handler,
     route, and render function from scratch.
  5. Node map alignment requires subtracting the tab-bar height (48dp) from the
     map's available vertical space, or anchoring map render to a smaller safe
     rect that excludes both the top bar and the bottom tab bar.

- **safe_to_proceed:** yes — all five blockers are surgical changes with clear
  file/line addresses identified in this report. No architectural refactors are
  required. The audit confirms F.1b scope is executable as specced.
