# OperatorGame — Sprint Roadmap

**Status:** Canonical  
**Authority:** Designer (Robert)  
**Version:** 2.0 — Post G.5b Realignment  
**Last Updated:** March 2026  

---

## Completed Sprints

| Sprint | Name | Status |
|--------|------|--------|
| F.1b | Playable Loop Recovery | ✅ Complete |
| G.1 | Mission Stability & Squad Persistence | ✅ Complete |
| G.1b | Difficulty Rebalance (SDD-035) | ✅ Complete |
| G.1c | Structural Rebalance & Squad Hub | ✅ Complete |
| G.2 | Map-Gated Progression | ✅ Complete |
| G.3 | Equipment (Hat) System | ✅ Complete |
| G.4 | Combat Logs | ✅ Complete |
| G.5 | Gauntlet Missions (Multi-Target) | ✅ Complete |
| G.5b | Baseline Restoration & Reconciliation | ✅ Complete |

**Current test floor:** 156 unit tests + 6 integration suites passing, 0 failing  
**Current SAVE_VERSION:** 11  
**Current build:** Deployed on Moto G, functional loop verified  

---

## Near-Term Sprints (Next 4-6 Weeks)

---

### Sprint G.6 — Leveling Feel & First Impression
**Priority: Highest. Do this before anything else.**

The first level-up (Hatchling → Juvenile) is a 33% power spike. It does not currently feel like one. This sprint makes Level 1 → 2 feel strong, visible, and rewarding. Includes: stat display before/after on level-up, XP rate tuning, level-up animation or visual moment in the AAR panel. No new systems. Pure feel and feedback.

**Delivers:** A player who levels up for the first time feels something.

---

### Sprint G.7 — Colors & Genetic Identity
**The depth the loop is missing.**

Implement the 9-culture color system (Primary/Secondary/Tertiary) so every slime has a visible genetic identity. A Spotted Ember slime looks different from a Striped Tide slime. Pattern discovery is tracked in a Pattern Registry. Patterns are visual only for now — the hook exists for future mechanical depth. Requires: culture expression on genome, color rendering on roster card, Pattern enum and discovery system.

**Delivers:** Every slime feels unique. The roster has visual depth. Pattern discovery is a moment.

**Authority:** GAME_DESIGN.md §4, rpgCore genetics reference

---

### Sprint G.8 — Breeding System
**Closes the generational loop.**

Implement the Synthesis Bay. Two operators can be paired to produce offspring. Offspring inherit genetics from both parents. Reagents modify the outcome via the Synthesis Slot (nudge color probability or force a Pattern). Elder Sacrifice mechanic: a Level 10 Elder can be contributed to breeding to pass higher Potential to the offspring. The astronaut's paperwork: "Voluntary Early Retirement with Succession Benefits."

**Delivers:** The generational loop closes. Players can engineer toward the Void Slime. Elder operators have a meaningful endgame role.

**Authority:** GAME_DESIGN.md §4, SDD-036 (Reagents section), rpgCore breeding reference

---

### Sprint G.9 — Map Depth & Node Interaction
**Makes the map a place, not a decoration.**

Tapping a node opens a panel showing its mission pool, resource yields, and unlock status. Active deployments are visible on the map. Ring 2 scouting becomes accessible after Ring 1 completion. The map becomes a record of conquest. Includes minor visual polish — node tap feedback, deployment indicator on occupied nodes.

**Delivers:** The map is interactive. Players can plan from it, not just look at it.

---

### Sprint G.10 — Mini-Boss Gate
**Gives the mid-game a wall worth climbing.**

A special mission unlocks when the player's average roster reaches Prime stage (Level 6-7). Harder than any standard mission. Defeat unlocks a meaningful reward: a new map feature, a breeding option, or a narrative beat about the Elder Void Slime. Design requires its own SDD before implementation.

**Delivers:** The first real milestone. A reason to breed toward Prime.

---

## Medium-Term Sprints (2-3 Months)

---

### Sprint G.11 — First Boss
**The Level 10 soft cap moment.**

Available when the player has at least one Elder operator. Signals the transition from early game to mid game. Defeat changes something about the world state — a locked region opens, a narrative beat fires, the Elder Void Slime becomes accessible as a story element. Full SDD required before implementation.

**Delivers:** The game has an act break.

---

### Sprint G.12 — Elder Passives
**Level 10 has meaning beyond the soft cap.**

Each culture's Elder operator gains a passive ability. Design TBD per culture — requires a separate design session before implementation. The passive should reflect the culture's character (Ember = aggressive, Gale = swift, Tide = adaptive, etc.).

**Delivers:** Elder operators are worth keeping, not just sacrificing.

---

### Sprint G.13 — Equipment Expansion
**More slots, more depth.**

Armor slots (Chest, Hands, Feet) as planned in G.9+. Each slot has a small item pool. Items are earned from missions and unlocked via the map, not purchased from a shop. The astronaut calls it "standard PPE requisition."

**Delivers:** The equipment system has depth without complexity.

---

## Long-Term Goals (Vision Layer)

These are not sprints yet. They are design targets that should inform architecture decisions now.

---

### Visual Squad Track (Beastie Bay DX / Slime Garden Reference)

The dispatched squad visibly moves along a path toward their mission objective. Sequential Gauntlet encounters are spatial — the squad marches from waypoint to waypoint. Failure means a visible retreat. The Gauntlet's sequential target model already aligns with this architecture.

**Not scheduled. Architecture should support it.**

---

### The Void Slime Endgame

Breeding toward an organism expressing all nine cultures simultaneously. The Elder Void Slime is the template. The narrative beat of discovering this is the game's emotional core. The player builds their own way home.

**Not scheduled. Every breeding sprint builds toward it.**

---

### JNI Live Insets

The current safe area insets in `platform.rs` are hardcoded stubs. A future sprint implements the JNI call to read actual device insets from `WindowMetrics`. Low priority until the layout is stable enough that inset precision matters.

---

### Screenshot Automation Refresh

The `OperatorDeviceTools` PowerShell automation was built against an earlier layout. The SidePanel navigation and new sub-tabs have changed the coordinate space significantly. A dedicated sprint to re-map tap coordinates and validate the automation pipeline. Low priority but worth doing before any public release.

---

### Play Store Release Prep

Asset blockers remain: 512×512 app icon, 1024×500 feature graphic, screenshots. These block promotion from Internal Testing to broader release. Not a code sprint — a production sprint. When the loop feels like a vertical slice, this becomes urgent.

---

## Sequence Rationale

The order matters:

1. **G.6 (Leveling Feel)** first because the loop exists but doesn't hook. No amount of new content fixes a weak feedback moment.
2. **G.7 (Colors)** second because visual identity makes the roster feel real. Every slime needs to be someone.
3. **G.8 (Breeding)** third because it closes the loop. Without breeding, the game is a treadmill. With it, it's a generational strategy game.
4. **G.9 (Map Depth)** fourth because once breeding exists, the map becomes the progression layer that connects it all.
5. **G.10 (Mini-Boss)** fifth because a milestone needs content to be worth reaching.

Resist the urge to jump to long-term goals. The short-term sprints build the foundation those goals require.

---

*RFD IT Services Ltd. | OperatorGame | Roadmap v2.0 | March 2026*
