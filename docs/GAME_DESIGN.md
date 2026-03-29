# OperatorGame — Authoritative Game Design Document

**Status:** Canonical  
**Authority:** Designer (Robert)  
**Version:** 2.0 — Post G.5b Realignment  
**Supersedes:** All prior GAME_DESIGN.md versions, agent-generated GDD drafts  

---

## §1 — Vision Statement

OperatorGame is a mobile-first tactical genetics game about an interstellar logistics field agent who has accidentally become a slime breeder while waiting for a rescue ship his company will never send.

The player manages a roster of slime "operators" — dispatching them on field assignments, reviewing their performance, leveling them up, and breeding the next generation. The astronaut has no idea he is doing any of this. He thinks he is managing a remote workforce. The UI reflects his corporate worldview entirely. The slimes are doing their best.

The long-term goal: breed a Void Slime — an organism expressing all nine genetic cultures simultaneously — to serve as the biological engine for an escape vessel. The corporation is not coming. You are building your own way home.

---

## §2 — Tone: Corporate-Absurdist

The humor comes from **corporate indifference as the primary antagonist**, not from the slimes being cute (though they are).

**The astronaut's frame:**
- Slimes are "Operators" — field personnel under temporary contract
- Breeding is "Workforce Development" or "Talent Pipeline"
- Genetic mutations are "Performance Differentiators"
- The AAR (After-Action Report) is standard HR documentation
- Injuries are "Occupational Health Incidents"
- The Elder Slime who protected him on landing is his "Senior Field Consultant"

**The player's frame:**
- This is a genetics lab
- These are living organisms with complex heredity
- The "performance reviews" are literally D20 rolls against mission difficulty
- The "talent pipeline" produces organisms with emergent genetic expressions
- The escape plan is biological engineering

The gap between these two frames is where the game lives.

**Influences:** Crashlands (Butterscotch Shenanigans) for cosmic absurdity with genuine stakes. Stardew Valley for the loop of care and patience rewarded. Kairosoft (Beastie Bay DX) for the satisfying grid of a managed ecosystem.

---

## §3 — The Core Loop

```
RECRUIT → EQUIP → SCOUT/DISPATCH → AAR → LEVEL UP → BREED → REPEAT
```

Each generation of slimes is stronger than the last. The player is not grinding — they are deliberately engineering a better workforce, one breeding pair at a time, toward a genetic expression that can power an escape vessel.

**Loop beat by beat:**

1. **Recruit** — hire a new operator from the Recruitment Agency. Cost: Scrap. The astronaut signs the paperwork. A slime hatches.

2. **Equip** — visit the Quartermaster. Purchase a Hat. The astronaut files a gear requisition. The slime wears a hat.

3. **Scout** — send operators to scout new locations on the asteroid. Unlocks new mission pools and map nodes. The astronaut calls it "territory assessment." It is exploration.

4. **Dispatch** — assign a squad (max 3) to a Field Assignment. The astronaut reviews the contract. The squad faces sequential Gauntlet encounters. D20 resolution against mission DC.

5. **AAR** — collect the After-Action Report. See outcomes, XP gained, resources earned, injuries sustained. The astronaut files the paperwork. Slimes level up.

6. **Breed** — pair two operators in the Synthesis Bay. Offspring inherit genetics from both parents. Reagents modify the outcome. Patterns unlock on discovery. The astronaut thinks this is "succession planning."

7. **Repeat** — with stronger slimes, harder missions become accessible. The map expands. The genetic pool deepens. The Void Slime gets closer.

---

## §4 — The Genetics System

### 4.1 — Nine Cultures (Colors)

Three tiers of genetic expression:

| Tier | Cultures | Notes |
|------|----------|-------|
| Primary | Ember, Gale, Tide | Foundation genetics. All recruits express at least one. |
| Secondary | Marsh, Crystal, Tundra | Emerge from Primary combinations. |
| Tertiary | Orange, Teal, Frost | Emerge from Secondary combinations. Rare. |

Culture determines base stat affinities, mission performance bonuses (future), and visual color identity.

### 4.2 — Patterns

Patterns are the second axis of genetic identity. A slime is not just "Ember" — it is "Spotted Ember" or "Striped Ember" or "Marbled Ember."

**Pattern discovery:**
- Specific culture combinations during breeding reveal Patterns
- Each Pattern is discovered once and persists in the player's Pattern Registry
- Discovering a new Pattern is a moment — it should feel like finding something

**Pattern architecture:**
- Pattern is a field on the genome: `pattern: Pattern` where `Pattern` is an enum
- Currently: visual identity only
- Future hook: Pattern carries a passive modifier, mission affinity, or breeding bonus
- The field exists. The hook is there. Complexity is deferred.

**Reagents and the Synthesis Slot:**
- Reagents are a single resource earned from missions and scout yields
- During breeding, the player can spend Reagents in the Synthesis Slot to:
  - Nudge color probability toward a desired culture expression
  - Force a specific Pattern expression for that pairing
  - Each modification has a Reagent cost
- Simple, single currency, no crafting chains
- The astronaut calls it "performance enhancement supplements"

### 4.3 — The Void Slime

The endgame genetic target. A slime expressing all nine cultures simultaneously.

The Elder Void Slime — the one who protected the astronaut when he crashed — is the template. It already exists. It is level 10. It has been waiting.

The player does not start with the Elder Void Slime in their roster. It is a story beat, revealed through progression. The question of whether the Elder knows it is the template is an open narrative question — and a good one.

---

## §5 — Progression: The 10-Level Arc

### 5.1 — Stage Ladder

| Level | Stage | Stat Multiplier | Notes |
|-------|-------|----------------|-------|
| 1 | Hatchling | 0.6x | Fresh recruit. Fragile. Low upkeep. |
| 2-3 | Juvenile | 0.8x | Learning. Standard injury rates. |
| 4-5 | Young | 1.0x | Baseline. Full deployment capability. |
| 6-7 | Prime | 1.2x | Peak performance. Breeding window opens. |
| 8-9 | Veteran | 1.1x | Battle-hardened. Slight decline from Prime. |
| 10 | Elder | 1.0x + passive | Soft cap. Upkeep exempt. Elder passive active. |

**Level 1 must feel strong.** The first level-up (Hatchling → Juvenile) is a 33% power spike. This must be visible in the UI and felt in mission performance. If it doesn't feel like something, the loop has no early hook.

### 5.2 — The Elder Mechanic

At Level 10, an operator becomes an Elder. Elders:
- Are upkeep-exempt
- Have a culture-specific passive ability
- Can be **Sacrificed in Breeding** — contributing their genetics to an offspring at a higher Potential tier than normal breeding would produce

This is the Respec mechanic. You are not respeccing the Elder. You are investing them into the next generation. The Elder is gone. The offspring carries their potential forward.

The astronaut's paperwork for this: "Voluntary Early Retirement with Succession Benefits."

### 5.3 — Boss Gates

**Mini-Boss (Level 5-7 range):**
- A special mission that becomes available when the player's average roster reaches Prime stage
- Harder than any standard mission
- Defeat unlocks a map feature, a breeding option, or a narrative beat
- Not a wall — a milestone

**First Boss (Level 10 soft cap):**
- Available when the player has at least one Elder operator
- Signals the transition from early game to mid game
- Defeat is meaningful — it should change something about the world state
- Design TBD — needs its own SDD when the time comes

---

## §6 — Equipment

**Current MVP: Hats only**

One slot per operator. Four hats. Flat stat bonuses. Scrap currency. Map-unlock gated.

| Hat | Bonus | Cost | Unlock |
|-----|-------|------|--------|
| Scout Hood | +2 AGI | 50 Scrap | Center (always) |
| Knight Helm | +2 STR | 100 Scrap | Ember Flats |
| Mage Hood | +2 INT | 100 Scrap | Tide Basin |
| Commander Cap | +1 ALL | 250 Scrap | Gale Ridge |

**Future:** Armor slots (Chest, Hands, Feet) planned for G.9+. No additional equipment in scope until then.

---

## §7 — The Map

### 7.1 — Current State

19 nodes across 3 rings. Center node (Hidden Meadow) always unlocked. Ring 1 unlocked via Scouting missions. Ring 2 and Ring 3 gated behind Ring 1 completion.

Scouting a node:
- Unlocks that node permanently
- Grants one-time heritage resource yield (Biomass/Scrap/Reagents)
- Opens culture-specific mission pool for that node
- May unlock a Hat in the Quartermaster (Ring 1 nodes)

### 7.2 — Map Interaction (Needs Work)

The map exists and renders. Node color reflects unlock state (gray = locked, culture color = unlocked). The interaction layer is thin. Future work:
- Tapping a node shows its mission pool and resource yields
- Node state reflects active deployments
- Map becomes a record of conquest, not just a visual

### 7.3 — Visual Squad Track (Long-Term Goal)

Inspired by Beastie Bay DX and Slime Garden's dungeon tracks:

The dispatched squad visibly moves along a path toward their mission objective. Sequential Gauntlet encounters are spatial — the squad marches from waypoint to waypoint. Failure means a visible retreat. Success means advance.

This is **not a near-term sprint.** It is a design target that should inform architecture decisions now so the foundation supports it when the time comes. The Gauntlet's sequential target model already aligns with this — each Target is a waypoint.

---

## §8 — Resource Economy

| Resource | System | Source | Sink |
|----------|--------|--------|------|
| Scrap | Recruitment + Equipment | General contracts, scout yields | Hiring operators, purchasing Hats |
| Biomass | Breeding | Scout yields (Marsh primary), missions | Incubation base cost |
| Reagents | Breeding (Synthesis Slot) | Scout yields (Crystal primary), missions | Pattern/color modification per breeding |

**Design rule (ADR-034):** Resources do not cross systems. Scrap does not fund breeding. Biomass does not fund equipment. Cross-system spending requires a design revision.

---

## §9 — Open Design Questions

These are acknowledged unknowns. They are not blocking current development but must be resolved before the sprints that need them.

1. **Elder Passive Abilities** — what does each culture's Elder passive actually do? Needs design before Level 10 is reachable.
2. **Mini-Boss design** — what is it, narratively and mechanically?
3. **First Boss design** — same question. Needs its own SDD.
4. **Pattern mechanical hooks** — when Patterns gain gameplay meaning, what are they? Design deferred but field exists.
5. **The Elder Void Slime reveal** — at what point does the player learn about it? What triggers the narrative beat?
6. **Does the Elder know?** — open narrative question. Leave it open for now.

---

## §10 — What This Game Is Not

Explicitly out of scope until otherwise designed:

- Real-time combat or action gameplay
- Multiplayer or social features (beyond future leaderboards)
- Premium currency or pay-to-win mechanics
- Complex crafting chains
- More than one equipment slot per operator (until G.9+)
- Class-gating on missions via hat type
- More than 3 operators per squad

---

*RFD IT Services Ltd. | OperatorGame | Game Design Document v2.0 | March 2026*
