# SDD-038: UI Architecture & Ground Truth

**Status:** Accepted  
**Authority:** Designer (Robert)  
**Version:** 1.0  
**Created:** March 2026  
**Companion Documents:** GAME_DESIGN.md v2.0 | ADR-038 (Modular Data Layer) | AGENT_CONTRACT.md  

> This document is the single source of truth for all UI layout decisions.  
> No UI constant, font size, or spacing value may be changed without updating this document first.  
> Future agents: read this before touching any file in `src/ui/`.

---

## §1 — Canvas Reality (Moto G 2025, Primary Target)

| Property | Value | Notes |
|----------|-------|-------|
| Physical width | 1080px | Hardware spec |
| Physical height | 2400px | Hardware spec |
| `pixels_per_point` | 2.0 | Set in `src/ui/mod.rs` — do not change |
| Logical canvas width | 540dp | 1080 / 2.0 |
| Logical canvas height | ~1200dp | 2400 / 2.0 |
| Safe area top | 48dp | Status bar / notch |
| Safe area bottom | 56dp | Gesture nav / 3-button bar |
| Side gutter | 8dp | Hardware bezel safety |
| **Usable width** | **524dp** | 540 - (8 × 2) |
| **Usable height** | **~1096dp** | 1200 - 48 - 56 |

**Secondary target:** Desktop window, minimum 800dp wide. All layout must work on both.

---

## §2 — Layout Zones (Fixed, Named)

These zones are fixed. Their dimensions do not change based on content.

```
┌─────────────────────────────────────────────┐
│  STATUS HEADER                    40dp tall  │
├──────────────┬──────────────────────────────┤
│              │                              │
│   SIDEBAR    │      CONTENT AREA            │
│   120dp wide │      396dp wide              │
│              │      (524 - 120 - 8 gutter)  │
│              │                              │
├──────────────┴──────────────────────────────┤
│  LAUNCH BAR                       44dp tall  │
├─────────────────────────────────────────────┤
│  TAB BAR                          56dp tall  │
└─────────────────────────────────────────────┘
```

| Zone | Width | Height | File |
|------|-------|--------|------|
| Status Header | 524dp | 40dp | `src/ui/mod.rs` |
| Sidebar | 120dp | remaining | `src/ui/mod.rs` → extract to `sidebar.rs` |
| Content Area | 396dp | remaining | per-tab render files |
| Launch Bar | 524dp | 44dp | `src/ui/mod.rs` |
| Tab Bar | 524dp | 56dp | `src/ui/mod.rs` |

**Named constants** — define these at the top of `src/ui/mod.rs` and reference everywhere:

```rust
pub const SIDEBAR_WIDTH: f32 = 120.0;
pub const STATUS_BAR_HEIGHT: f32 = 40.0;
pub const LAUNCH_BAR_HEIGHT: f32 = 44.0;
pub const TAB_BAR_HEIGHT: f32 = 56.0;
pub const SIDE_GUTTER: f32 = 8.0;
pub const CONTENT_WIDTH: f32 = 524.0 - SIDEBAR_WIDTH - SIDE_GUTTER; // 396dp
```

---

## §3 — Typography Scale

One size per role. Do not deviate.

| Role | Size | Weight | Color | Used In |
|------|------|--------|-------|---------|
| Sidebar section header | 15.0pt | Strong | `COLOR_PRIMARY` | Sidebar tab group labels |
| Sidebar button label | 15.0pt | Normal | context | Sub-tab buttons |
| Card title (operator name) | 14.0pt | Strong | culture color | Roster card row 1 |
| Card culture label | 11.0pt | Normal | culture color | Roster card row 1 |
| Card sub-status | 11.0pt | Normal | muted | Row 2 (level/stage/pattern) |
| Stats row | 11.0pt | Normal | light gray | Row 4 (STR/AGI/INT/HP) |
| Flavor / subtext | 10.0pt | Normal | gray (160,160,160) | Small labels, tooltips |
| Status bar text | 14.0pt | Normal | white | Header bar |
| Button labels | 12.0pt | Normal | context | STAGE, EQUIP HAT, etc. |

> **SIDEBAR: font size 15.0pt — do not reduce without designer approval**  
> This comment must appear above sidebar font size definitions in code.

---

## §4 — The Card Contract (Roster Operator Card)

Every roster card renders **exactly 5 rows** in this order. No additions without updating this document.

```
┌─────────────────────────────────────────────┐  ← frame: 8dp inner margin
│ [Name]culture    [Culture]     [STAGE] [▶]  │  Row 1: Header
│ Lv: N  HATCHLING  Marbled                   │  Row 2: Sub-status
│ ████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░  │  Row 3: XP bar (4dp, no %)
│ STR:4  AGI:5  INT:5  HP:20                  │  Row 4: Vitals
│ [+ EQUIP HAT] or [🎩 Scout Hood]            │  Row 5: Hat action
└─────────────────────────────────────────────┘
```

**Row specifications:**

| Row | Content | Font | Notes |
|-----|---------|------|-------|
| 1 — Header | Name (14pt strong, culture color) + Culture label (11pt) left; STAGE button + ▶ button right | 14pt / 11pt | `horizontal_wrapped` |
| 2 — Sub-status | `Lv: N` + Stage name (stage color) + Pattern name | 11pt | Single `horizontal` row |
| 3 — XP bar | Progress bar | 4dp height | No percentage text. No label. |
| 4 — Vitals | STR:N  AGI:N  INT:N  HP:N | 11pt | Single `horizontal` row |
| 5 — Hat | Button showing hat name or "+ EQUIP HAT" | 12pt | Full width button |

**Card frame:**
- Inner margin: 8dp all sides
- Corner rounding: 4dp
- Staged fill: `rgb(30, 50, 40)`
- Default fill: `rgb(26, 26, 34)`
- Staged stroke: `Color32::GREEN`, 1px
- Default stroke: `from_gray(60)`, 1px

**Card width:** `CONTENT_WIDTH - (SIDE_GUTTER * 2)` = 380dp  
The card must not exceed this width. `ui.set_max_width(380.0)` enforced inside the frame.

---

## §5 — Spacing Constants

```rust
pub const CARD_INNER_MARGIN: f32 = 8.0;   // Inside every card frame
pub const CARD_GAP: f32 = 4.0;            // Vertical gap between cards
pub const SECTION_GUTTER: f32 = 12.0;     // Between logical sections
pub const SIDE_GUTTER: f32 = 8.0;         // Horizontal screen edge safety
```

**Target card density:** 4-5 operator cards visible simultaneously on Moto G without scrolling.

---

## §6 — Stage Color Palette

Each life stage has a fixed muted color for its label. Do not change these.

| Stage | Color (RGB) | Hex |
|-------|-------------|-----|
| Hatchling | (160, 160, 160) | `#A0A0A0` |
| Juvenile | (140, 200, 140) | `#8CC88C` |
| Young | (100, 200, 180) | `#64C8B4` |
| Prime | (220, 180, 80) | `#DCB450` |
| Veteran | (200, 140, 60) | `#C88C3C` |
| Elder | (180, 120, 220) | `#B478DC` |

---

## §7 — File Ownership

Each UI concern has exactly one owner file. Cross-file rendering is not permitted.

| Concern | Owner File | Notes |
|---------|------------|-------|
| App shell, tab routing, layout zones | `src/ui/mod.rs` | Constants defined here |
| Sidebar navigation | `src/ui/sidebar.rs` | Extract from mod.rs — Sprint G.7 pre-work |
| Roster cards, operator detail, recruit | `src/ui/manifest.rs` | Card Contract authority |
| Active ops, AAR panel | `src/ui/ops.rs` | |
| Quest board, contract cards | `src/ui/contracts.rs` | |
| Map rendering, node visuals | `src/ui/radar.rs` | |
| Squad staging view | `src/ui/squad.rs` | |
| Hat shop, equip flow | `src/ui/quartermaster.rs` | |

---

## §8 — What Is Explicitly Prohibited

These patterns caused the UI regressions. They are banned:

- Hardcoded pixel coordinates anywhere in UI code
- `min_size` constraints on sidebar buttons
- `apply_interaction_scale()` being called in the update loop (currently commented out — leave it that way)
- Font sizes below 10pt anywhere visible to the player
- Sidebar width below 120dp
- `pixels_per_point` values other than 2.0 for Android target
- Any agent changing a constant in this document without updating the document first

---

## §9 — Implementation Sprint (UI Ground Truth)

When the implementation directive is written, the file scope will be:

| File | Change |
|------|--------|
| `src/ui/mod.rs` | Add named constants (§2), remove all magic numbers |
| `src/ui/sidebar.rs` | **NEW** — extract sidebar rendering from mod.rs |
| `src/ui/manifest.rs` | Enforce card width = 380dp, enforce 5-row contract |
| `src/platform.rs` | Align `TAB_BAR_HEIGHT` constant with §2 values |

This sprint happens **before G.7** content work begins.

---

*RFD IT Services Ltd. | OperatorGame | SDD-038 UI Architecture | March 2026*
