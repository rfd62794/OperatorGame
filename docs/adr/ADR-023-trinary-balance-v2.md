# ADR-023 v2: Trinary Balance Protocol — 9-Culture Geometry Lock

**Status:** ACCEPTED | **Date:** 2026-03-06 | **Supersedes:** ADR-023 v1 | **Author:** RustPro SDD-Edition

---

## Context

ADR-023 v1 established the Nested 3-Layer Trinary RPS concept but contained color identity conflicts with both `genetics.rs` and ADR-022:

| Culture | ADR-023 v1 | ADR-022 | genetics.rs `frequency()` | **Verdict** |
|---|---|---|---|---|
| Gale    | YELLOW (Primary) | YELLOW (Primary) | 288 Hz — airy, high | ❌ Gale is **Secondary/Middle** in nonagon. Wrong layer. |
| Tide    | BLUE (Primary)   | BLUE (Primary)   | 320 Hz — rolling, pulse | ❌ Tide is **Secondary/Middle** in nonagon. Wrong layer. |
| Marsh   | GREEN (Secondary) | GREEN (Secondary) | 384 Hz | ✅ Consistent |
| Crystal | PURPLE (Secondary) | PURPLE (Secondary) | 426 Hz | ⚠️ Reassigned to BLUE (PRIMARY) in this revision |
| Tundra  | FROST (Tertiary) | FROST (Tertiary) | 540 Hz | ✅ Consistent |

Both ADR-022 and ADR-023 v1 wrongly placed Gale and Tide in the Primary loop. The **color-theory primary/secondary/tertiary model** governs: Red, Yellow, Blue are the three true primaries. Mixing adjacent primaries produces secondaries. This revision locks all identities before Sprint 4 implementation.

---

## The Nine Cultures — Identity Lock

| Layer | Culture | Color | Hex Code | Frequency | Dominant Stat | Shape Language |
|---|---|---|---|---|---|---|
| **Inner (Primary)** | Ember   | RED        | #CC2200 | 256 Hz | ATK | Aggressive, Jagged, Angular |
| **Inner (Primary)** | Marsh   | YELLOW     | #E8C000 | 384 Hz | HP  | Rounded, Organic, Bulbous |
| **Inner (Primary)** | Crystal | BLUE       | #1144CC | 426 Hz | DEF | Geometric, Faceted, Sharp |
| **Middle (Secondary)** | Tide    | ORANGE     | #E87000 | 320 Hz | CHM | Fluid, Flowing, Rippled |
| **Middle (Secondary)** | Gale    | GREEN      | #22AA44 | 288 Hz | SPD | Wispy, Irregular, Frayed |
| **Middle (Secondary)** | Tundra  | VIOLET     | #7722CC | 540 Hz | RES | Dense, Compact, Heavy |
| **Outer (Tertiary)** | Orange  | AMBER      | #DD8800 | 336 Hz | MND | Warm, Harvest, Textured |
| **Outer (Tertiary)** | Teal    | TEAL       | #00AAAA | 407 Hz | AGI | Cold, Precise, Sleek |
| **Outer (Tertiary)** | Frost   | ICE BLUE   | #88CCFF | 480 Hz | END | Still, Ancient, Crystalline |
| **Exception** | Void    | EQUAL MIX  | #888888 | 432 Hz | —   | Universal Constant |

### Mixing Logic (Color Theory)

```
Inner primaries mix to produce Middle secondaries:
  Ember   (Red)    + Marsh   (Yellow) → Tide   (Orange)
  Marsh   (Yellow) + Crystal (Blue)   → Gale   (Green)
  Crystal (Blue)   + Ember   (Red)    → Tundra (Violet)

Middle secondaries mix to produce Outer tertiaries:
  Tide    (Orange) + Marsh   (Yellow) → Orange  (Amber)
  Gale    (Green)  + Crystal (Blue)   → Teal    (Teal)
  Tundra  (Violet) + Ember   (Red)    → Frost   (Ice Blue)
```

---

## Color Identity Resolution

### The Conflict

ADR-023 v1 assigned `Gale → YELLOW` and `Tide → BLUE`, placing them in the Primary inner loop. However:

- **Gale** (288 Hz, SPD-dominant) is a **blend** of Marsh+Crystal — it sits _between_ Yellow and Blue on the spectrum, producing **Green**. Secondary layer.
- **Tide** (320 Hz, CHM-dominant) is a **blend** of Ember+Marsh — it sits _between_ Red and Yellow, producing **Orange**. Secondary layer.

### Resolution

Color-theory primary/secondary/tertiary model governs. The three true primary colors (Red, Yellow, Blue) map to the three cultures that are **foundational** to the system — not derivative of any pair.

**Old assignments voided. New assignments locked:**
- Gale → GREEN (Middle Secondary, Marsh+Crystal blend)
- Tide → ORANGE (Middle Secondary, Ember+Marsh blend)
- Marsh → YELLOW (Inner Primary — was Secondary in ADR-022 v1)
- Crystal → BLUE (Inner Primary — was Secondary in ADR-022 v1)

---

## The 9-Point Nonagon Wheel (Clockwise)

```
          Ember (Red / 256Hz)
         /                   \
  Frost                       Tide
 (IceBlue/480Hz)           (Orange/320Hz)
    |                               |
  Tundra                         Orange
 (Violet/540Hz)               (Amber/336Hz)
    |                               |
  Gale                           Marsh
 (Green/288Hz)               (Yellow/384Hz)
         \                   /
          Teal (Teal / 407Hz)
              |
           Crystal (Blue / 426Hz)
```

**Nonagon adjacency (each culture touches exactly 2 neighbours):**

| Culture | Left Neighbour | Right Neighbour |
|---|---|---|
| Ember   | Frost    | Tide   |
| Tide    | Ember    | Orange |
| Orange  | Tide     | Marsh  |
| Marsh   | Orange   | Teal   |
| Teal    | Marsh    | Crystal|
| Crystal | Teal     | Gale   |
| Gale    | Crystal  | Tundra |
| Tundra  | Gale     | Frost  |
| Frost   | Tundra   | Ember  |

---

## Opposite Pairs (9-point, across-centre)

> [!IMPORTANT]
> **9-point odd-wheel geometry: no culture has a single exact true opposite.**
>
> On an odd-count polygon, the across-centre position always falls **between** two adjacent
> cultures. For culture at position `n`, near-opposites are at `(n+4) mod 9` and `(n+5) mod 9`.
> The `is_opposite()` function must treat **both** as valid Sundered pairs for any given culture.
>
> Before implementing `is_opposite()`, decide whether `Sundered` requires the *shorter arc*
> near-opposite, the *longer arc*, or **either**. Recommend: **either** (both fire Sundered)
> — this is the more permissive and player-friendly interpretation.

**Nonagon near-opposite pairs (positions 0–8 clockwise: 0=Ember, 1=Tide, 2=Orange, 3=Marsh, 4=Teal, 5=Crystal, 6=Gale, 7=Tundra, 8=Frost):**

| Culture | Position | Near-Opp A (+4 mod 9) | Near-Opp B (+5 mod 9) |
|---|---|---|---|
| Ember   | 0 | Teal (4)    | Crystal (5) |
| Tide    | 1 | Crystal (5) | Gale (6)    |
| Orange  | 2 | Gale (6)    | Tundra (7)  |
| Marsh   | 3 | Tundra (7)  | Frost (8)   |
| Teal    | 4 | Frost (8)   | Ember (0)   |
| Crystal | 5 | Ember (0)   | Tide (1)    |
| Gale    | 6 | Tide (1)    | Orange (2)  |
| Tundra  | 7 | Orange (2)  | Marsh (3)   |
| Frost   | 8 | Marsh (3)   | Teal (4)    |

**Tundra reconciliation:** Tundra's near-opposites are **Orange** and **Marsh** — both equally valid for Sundered detection. The previous table incorrectly listed a single "true opposite"; now corrected.

---

## The 63-Relationship RPS Table

Each culture beats exactly 4, loses to 4. No ties. Void is immune.

### Pressure Modifiers

| Relationship | Modifier |
|---|---|
| Advantage (beats) | × 1.25 |
| Neutral | × 1.00 |
| Disadvantage (loses) | × 0.75 |
| Void (either combatant) | × 1.00 (immune) |

### Full Lookup Table

#### Inner Loop — Intra-Primary RPS (3 relationships)
| Attacker | Defender | Modifier | Reason |
|---|---|---|---|
| Ember   | Marsh   | 1.25 | Ember→Marsh→Crystal→Ember |
| Marsh   | Crystal | 1.25 | Inner RPS cycle |
| Crystal | Ember   | 1.25 | Inner RPS cycle |

#### Inner → Middle — Primary Outward Pressure (6 relationships)
| Attacker | Defender | Modifier | Reason |
|---|---|---|---|
| Ember   | Tide    | 1.25 | Primary beats adjacent secondary |
| Ember   | Tundra  | 1.25 | Primary beats adjacent secondary |
| Marsh   | Tide    | 1.25 | Primary beats adjacent secondary |
| Marsh   | Gale    | 1.25 | Primary beats adjacent secondary |
| Crystal | Gale    | 1.25 | Primary beats adjacent secondary |
| Crystal | Tundra  | 1.25 | Primary beats adjacent secondary |

#### Middle Loop — Intra-Secondary RPS (3 relationships)
| Attacker | Defender | Modifier | Reason |
|---|---|---|---|
| Tide    | Gale    | 1.25 | Tide→Gale→Tundra→Tide |
| Gale    | Tundra  | 1.25 | Middle RPS cycle |
| Tundra  | Tide    | 1.25 | Middle RPS cycle |

#### Middle → Outer — Secondary Outward Pressure (6 relationships)
| Attacker | Defender | Modifier | Reason |
|---|---|---|---|
| Tide    | Orange  | 1.25 | Secondary beats adjacent tertiary |
| Tide    | Frost   | 1.25 | Secondary beats adjacent tertiary |
| Gale    | Orange  | 1.25 | Secondary beats adjacent tertiary |
| Gale    | Teal    | 1.25 | Secondary beats adjacent tertiary |
| Tundra  | Teal    | 1.25 | Secondary beats adjacent tertiary |
| Tundra  | Frost   | 1.25 | Secondary beats adjacent tertiary |

#### Outer Loop — Intra-Tertiary RPS (3 relationships)
| Attacker | Defender | Modifier | Reason |
|---|---|---|---|
| Orange  | Teal    | 1.25 | Orange→Teal→Frost→Orange |
| Teal    | Frost   | 1.25 | Outer RPS cycle |
| Frost   | Orange  | 1.25 | Outer RPS cycle |

#### Outer → Inner — Tertiary Inward Resistance (6 relationships)
Tertiaries resist primaries: the ancient layer absorbs primal force.
| Attacker | Defender | Modifier | Reason |
|---|---|---|---|
| Orange  | Marsh   | 1.25 | Tertiary resists primary |
| Orange  | Crystal | 1.25 | Tertiary resists primary |
| Teal    | Ember   | 1.25 | Tertiary resists primary |
| Teal    | Marsh   | 1.25 | Tertiary resists primary |
| Frost   | Crystal | 1.25 | Tertiary resists primary |
| Frost   | Ember   | 1.25 | Tertiary resists primary |

#### Cross-Layer Losses (all disadvantage relationships = inverse of above)
All 27 advantage relationships above have a corresponding × 0.75 reverse.
Total directed relationships: **27 advantages + 27 disadvantages = 54 unique pairs + 9 neutral (same-layer cross) = 63 total**.

> **Note for implementation**: `get_rps_modifier(attacker, defender)` returns 1.25 if attacker beats defender, 0.75 if defender beats attacker, 1.0 otherwise (same culture, or Void involved).

---

## GeneticTier — Updated for 9-Culture Wheel

The hexagon GeneticTier geometry is replaced by nonagon geometry. Thresholds and names remain but active culture counts expand:

| Tier | Active Cultures | Condition (9-culture wheel) |
|---|---|---|
| Blooded    | 1 | Single dominant (unchanged) |
| Bordered   | 2 | Adjacent on nonagon (wheel neighbours) |
| Sundered   | 2 | Near-opposite pair — either `(n+4) mod 9` or `(n+5) mod 9` |
| Drifted    | 2 | Two active, skip-one (not adjacent, not near-opposite) |
| Threaded   | 3 | Any three active |
| Convergent | 4–5 | Four or five active (expanded from 4 only) |
| Liminal    | 6–7 | Six or seven active (expanded) |
| Void       | 8–9 | All or near-all active (expanded from 6) |

### Updated Opposite Pairs for Sundered Detection

**Decision: `Sundered` fires when either near-opposite is active** — the permissive interpretation.
Do NOT implement the restrictive "only shorter arc" variant without a design review.

See the corrected opposite table above. Key: every culture has exactly **two** near-opposites
on the 9-point wheel (none has a single exact one).

> [!NOTE]
> **Why odd-n polygons have no single true opposite:** On any odd-count polygon, `n/2` is not
> an integer, so the across-centre position always falls between two adjacent vertices. Even-polygon
> assumptions ("each vertex has exactly one opposite") produce bugs on odd wheels. This note
> is kept explicit here so future implementers don't inadvertently revert to even-polygon logic.

```rust
// Sprint 4 implementation contract for is_near_opposite()
// Arc-symmetric: measures the shortest arc distance on the 9-point wheel.
// Both arc distances 4 and 5 are Sundered-qualifying (no special-casing needed).
pub fn is_near_opposite(a: Culture, b: Culture, wheel: &[Culture; 9]) -> bool {
    let pos_a = wheel.iter().position(|&c| c == a).unwrap();
    let pos_b = wheel.iter().position(|&c| c == b).unwrap();
    let dist     = (pos_a as i32 - pos_b as i32).unsigned_abs() as usize;
    let min_dist = dist.min(9 - dist);   // shorter arc
    min_dist == 4 || min_dist == 5       // both arcs fire Sundered
}
// Coverage: 9 pairs × 2 directions = 18 ordered pairs trigger Sundered.
// No special-casing. No asymmetry. Arc distance is commutative.
```

---

## Implementation Notes (Sprint 4 Checklist)

```
[ ] src/genetics.rs
    - Add Orange, Teal, Frost to Culture enum
    - Update WHEEL: [Culture; 9]  (3 new entries, exact order: Ember=0, Tide=1, Orange=2,
      Marsh=3, Teal=4, Crystal=5, Gale=6, Tundra=7, Frost=8)
    - Update CultureExpression: [f32; 9]
    - Update all match arms (+3 arms throughout file)
    - Replace is_adjacent() with 9-point nonagon adjacency (each culture touches pos±1)
    - Replace is_opposite() with is_near_opposite() using min_dist arc formula above
    - Update frequency() for 3 new cultures (Orange=336Hz, Teal=407Hz, Frost=480Hz)
    - Update params() for 3 new cultures
    - Update name() for 3 new cultures
    - BreedingResolver::resolve_culture() loop: 6 → 9
    - GeneticTier::from_expression(): update Sundered arm to call is_near_opposite()

[ ] TRINITY BONUS SCOPE — LOCKED DECISION:
    ADR-022 Trinity bonus was written for 6-culture squads (any 1 primary + 1 secondary +
    1 tertiary). With 9 cultures and 3 loop-completing triads, two options exist:

    Option A — INNER LOOP ONLY: Trinity fires only when squad contains
      Ember + Marsh + Crystal (the three inner-loop primaries).
      More exclusive, meaningful as a skill-expression mechanic.

    Option B — ANY COMPLETE LOOP: Trinity fires when squad completes any
      of the three loops (Inner, Middle, or Outer triad).
      More accessible, rewards diverse squad building.

    **Sprint 4 decision: OPTION B (any complete loop).** Reasoning: restricting
    Trinity to the inner loop alone makes it effectively a high-tier gate (all three
    cultures must be Inner primaries). Option B rewards the full ecosystem of loop
    strategies and keeps Trinity achievable at any tier level. Three distinct Trinity
    bonus thresholds may be added in Sprint 5 (inner > middle > outer strength).

[ ] src/combat.rs
    - Implement get_rps_modifier(attacker, defender) -> f32
    - Implement has_trinity_bonus(squad) using Option B (any complete loop)
    - Apply apply_trinity_bonus() as +2 flat modifier to all D20 rolls for that squad
    - Update culture_zone_mode() for 9-culture adjacency

[ ] src/world_map.rs
    - Distribute 9 cultures across 19 nodes (was 6)
    - seed_expedition_targets(): add Orange, Teal, Frost targets

[ ] src/models.rs
    - Expedition::resolve(): wire culture_zone_mode() (see TODO Sprint 4 comment)

[ ] Tests
    - Update test_culture_expression_normalises (array size 6→9)
    - Update test_genetic_tier_* (new thresholds and Sundered using min_dist)
    - Add get_rps_modifier tests (6 cases minimum, one per relationship class)
    - Add is_near_opposite tests (verify symmetry, all 9 Sundered pairs)
    - Add has_trinity_bonus tests (Inner loop, Middle loop, Outer loop)
```

---

## Migration — Existing Saves

Existing slimes have `CultureExpression([f32; 6])`. Migration to `[f32; 9]`:

1. Zero-pad to 9 slots: `[e0, e1, e2, e3, e4, e5, 0.0, 0.0, 0.0]`
2. Re-normalise (sum is still 1.0 — padding zeros don't change it)
3. `GeneticTier` recomputed from new active count — tier may shift up if thresholds change

**All existing slimes remain valid.** No data is lost. Tier labels may change.

---

## Consequences

**Positive:**
- All color assignments now consistent with color theory
- 63 directed relationships provide genuine tactical depth
- Nonagon geometry is self-documenting (adjacency = mixing parentage)
- Three-tier RPS creates natural counterplay at every rank

**Negative:**
- Breaking change to genetics.rs (all match arms +3)
- Sprint 4 is the largest single migration in the project
- UI complexity: 9 cultures + Void requires new color wheel widget

**Risks:**
- Tier redistribution on save migration may surprise players
- Tundra→VIOLET reassignment may conflict with existing art assets
- Trinity bonus (ADR-022) needs re-scoping for 9-culture squads
