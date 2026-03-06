# SDD-005: Genetics & Biological Sovereignty

## 1. The "Slime = Operator" Unification (Biological Sovereignty)
Under the "Biological Sovereignty" amendment (ADR-037), the legacy `Operator` struct has been entirely deprecated. All "Personnel" logic, including stats, loadouts, and mission deployment states, is now intrinsically tied to the `SlimeGenome`. The Slime is the Operator. Their biological identity determines their physical capabilities.

## 2. The DNA Bitmask & The 3+3 Stat Split
A Slime's core identity is generated from a `u32` DNA bitmask, which governs its genetic expression across the 9-point chromatic wheel. This cultural heritage directly maps to their physical archetype and mentality.

The biological profile uses a **3+3 Stat Split**, separating Mission Execution from Garden/Repair Efficiency.

### HARD STATS (Physical) - Mission Combat / DC Checks
- **Strength (STR):** Correlates with Ember (Red / 256Hz). High physical power and resilience in combat.
- **Agility (AGI):** Correlates with Gale (Yellow / 288Hz). High speed and evasion.
- **Intelligence (INT):** Correlates with Tide (Blue / 320Hz). Tech interaction and hacking.

### SOFT STATS (Mental) - Garden & Repair Efficiency
- **Mind (MND):** Drives the efficiency and speed of Ship Repairs / Research.
- **Sensory (SEN):** Detection of rare nodes and optimal interaction with the Ripple topography.
- **Tenacity (TEN):** Startle Resistance. The slime's ability to resist the stress of Hooting and maintain geometric stability.

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
Instead of generic armor, Equipment acts as a "Resonance Dampener," stabilizing a Slime's frequency for specific task parameters. Currently, Gear primarily boosts Hard Stats.
- `total_strength() = base_strength + gear_bonus`
- `total_agility() = base_agility + gear_bonus`
- `total_intelligence() = base_intelligence + gear_bonus`
- `total_mind() = base_mind + gear_bonus`
- `total_sensory() = base_sensory + gear_bonus`
- `total_tenacity() = base_tenacity + gear_bonus`
