# SDD-042: Flutter Card Contract (Pass 1 - "Stitch Minimal")
> **Status:** Draft | 2026-03-29

## §1 — Goal
Replicate the **Roster Card** logic from `manifest.rs` using Flutter's layout engine, solving the 540dp horizontal clipping and closure-capture issues forever.

## §2 — Layout Blueprint (The "Stitch Minimal" Card)

Every card in the "Pass 1" implementation must render these **5 rows** in order.

```text
┌─────────────────────────────────────────┐  ← Card: Margin(8.0), Rounded(12.0)
│ DustyMarsh          [STAGE]  [>]        │  Row 1: Header (Name left, buttons right)
│ Lv.1 · HATCHLING · Marbled             │  Row 2: Context (Row of Pills/Labels)
│ ████████████░░░░░░░░░░░░░░░░░░░░░░░    │  Row 3: XP Bar (LinearProgressIndicator)
│ STR:4  AGI:5  INT:5  HP:20             │  Row 4: Stats (Single row, mono font)
│ [+ EQUIP HAT] or [🎩 Scout Hood]        │  Row 5: Hat Action (ElevatedButton)
└─────────────────────────────────────────┘
```

## §3 — Row Specifications

| Row | Widget | Logic / State |
|-----|--------|---------------|
| **1 — Header** | `Row` + `Spacer` | Name on left (Bold); `STAGE` and `>` buttons on far right. No wrapping allowed. |
| **2 — Context** | `Row` | Segmented text or `Chip` widgets for Level, LifeStage, and Pattern. |
| **3 — XP Bar** | `LinearProgressIndicator` | Proportional to `current_xp / xp_to_next`. 4dp height. |
| **4 — Stats** | `Text` | Monospaced or formatted string for STR/AGI/INT/HP. |
| **5 — Hat** | `ElevatedButton` | Displays current hat or prompt to equip. |

## §4 — Implementation Constraints
1.  **Framework**: Standard `Flutter` Material 3 widgets.
2.  **State**: `StatefulWidget` (minimalist for PoC).
3.  **Colors**: Background `Color(0xFF1A1A22)`, Staged border `Colors.green`.
4.  **Responsive**: The `Row` in Row 1 must use `MainAxisAlignment.spaceBetween` to ensure buttons are pinned to the right edge of any device width.

---
*RFD IT Services Ltd. | OperatorGame | SDD-042 Flutter Card Contract | March 2026*
