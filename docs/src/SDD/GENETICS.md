# SDD-005: Genetics & Biological Sovereignty

## 1. The "Slime = Operator" Unification (Biological Sovereignty)
Under the "Biological Sovereignty" amendment (ADR-037), the legacy `Operator` struct has been entirely deprecated. All "Personnel" logic, including stats, loadouts, and mission deployment states, is now intrinsically tied to the `SlimeGenome`. The Slime is the Operator. Their biological identity determines their physical capabilities.

## 2. The DNA Bitmask & Base Stats
A Slime's core identity is generated from a `u32` DNA bitmask, which governs its genetic expression across the 9-point chromatic wheel. This cultural heritage directly maps to their physical archetype:
- **Strength (STR):** Correlates with Ember (Red / 256Hz). High physical power and resilience.
- **Agility (AGI):** Correlates with Gale (Yellow / 288Hz). High speed and evasion.
- **Intelligence (INT):** Correlates with Tide (Blue / 320Hz). High analytical capability and harmonic resonance.

Base stats for a Tier-0 recruit range from 5–8, serving as the biological foundation before Gear modifiers are applied.

## 3. The 9-Point Chromatic Framework
The planetary ecosystem is governed by a 9-Point Color Wheel, mapping cultural archetypes to specific frequencies and visual logic:

### Primary Triad
- **Ember (Red):** 256Hz. Dominant Stat: STR. Element: Fire.
- **Gale (Yellow):** 288Hz. Dominant Stat: AGI. Element: Wind.
- **Tide (Blue):** 320Hz. Dominant Stat: INT. Element: Water/Electric.

### Secondary Triad (Hybrids)
- **Orange:** 293.66Hz. Logic / Engineering.
- **Marsh (Green):** 384Hz. Survival / Toxin.
- **Crystal (Purple):** 426Hz. Tank / Armor.

### Tertiary Triad (Outer Rim)
- **Amber:** 480Hz. Industry.
- **Teal:** 512Hz. Support.
- **Tundra (Frost):** 540Hz. Stasis / Preservation.

### The Void Exception
- **Void (White):** 432Hz. The Elder's baseline frequency. The "Heart of Nature." All elements ×0.75, rare mutations ×4.

## 4. Stat Consolidation & Gear Dampening
Instead of generic armor, Equipment acts as a "Resonance Dampener," stabilizing a Slime's frequency for specific task parameters. 
- `total_strength() = base_strength + gear_bonus`
- `total_agility() = base_agility + gear_bonus`
- `total_intelligence() = base_intelligence + gear_bonus`
