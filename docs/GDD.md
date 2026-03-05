# OPERATOR — Game Design Document (GDD)
> **Version:** 2.0 | **Status:** Tier 1–3 shipped, Sprint 1 genetics live | 2026-03-04
> Working Title: *OPERATOR*

---

## 1. Executive Summary

**OPERATOR** is a single-player **dispatch-and-breed simulator** — a blend of idle-game logistics and biological strategy. The player commands a mercenary operation *and* a slime breeding program, using the two systems to reinforce each other: income from contracts funds the breeding stable; high-tier slimes unlock harder contracts with bigger payouts.

The core emotional loop is:
> *"My best slime just hit Tier 5 — but she's the only one I have at that genetic weight. Do I send her on the Black Site mission, or keep her safe for breeding?"*

---

## 2. Design Pillars

| Pillar | Description |
|--------|-------------|
| **Risk is Real** | Permanent operator/slime death at a 5% floor. No save-scumming. |
| **The Ratchet Effect** | Every breeding session produces offspring at least as strong as their parents. Progress never goes backward. |
| **Composition Matters** | Wrong culture on a mission, wrong job in a squad — both are strategic mistakes with mechanical consequences. |
| **Readable Systems** | Every formula is documented in the GDD and surfaced in the UI. No hidden multipliers. |
| **Respect Player Time** | All timers run wall-clock. Open the app when you're ready, not because you have to. |

---

## 3. The Two Game Layers

### Layer A — The Mercenary Operation (OPERATOR)

Classical dispatch loop. Human operators, contracts, squad composition.

```
┌───────────────────────────────────────────────────┐
│  1. ROSTER MANAGEMENT                             │
│     Review operators (Job / Stats / State)        │
├───────────────────────────────────────────────────┤
│  2. CONTRACT SELECTION                            │
│     Browse missions — weigh requirements vs. risk │
├───────────────────────────────────────────────────┤
│  3. SQUAD ASSEMBLY (War Room)                     │
│     Stage 1–3 operators. See live success% preview│
├───────────────────────────────────────────────────┤
│  4. DEPLOYMENT                                    │
│     Timer starts. Progress bar animates. Log off. │
├───────────────────────────────────────────────────┤
│  5. AFTER ACTION REPORT (AAR)                     │
│     Narrative log generated. Reward / Injury / KIA│
└───────────────────────────────────────────────────┘
               ↑ spend rewards → hire / hatch ↓
```

### Layer B — The Slime Breeding Program (GENETICS)

Biological simulation. Slimes, cultural archetypes, hex-wheel tier system.

```
┌───────────────────────────────────────────────────┐
│  1. HATCH                                         │
│     Seed a new slime from a cultural archetype    │
├───────────────────────────────────────────────────┤
│  2. LEVEL UP                                      │
│     Dispatch slimes on missions → earn XP         │
├───────────────────────────────────────────────────┤
│  3. SPLICE (Breed)                                │
│     Combine two Young+ slimes → offspring with    │
│     blended culture expression + stat ratchet     │
├───────────────────────────────────────────────────┤
│  4. UNLOCK TIERS                                  │
│     Cross-culture breeds push tier higher         │
│     Blooded → Bordered → Sundered → … → Void      │
├───────────────────────────────────────────────────┤
│  5. ISLAND EXPEDITION (Sprint 2+)                 │
│     High-tier slimes unlock roguelike dungeon     │
│     floors with elemental zone bonuses            │
└───────────────────────────────────────────────────┘
```

---

## 4. The Mercenary System (Layer A Detail)

### 4.1 Operators

Named mercenaries with three base attributes and a job archetype.

| Stat | Range | Description |
|------|-------|-------------|
| Strength | 0–100 | Force, breaching, physical tasks |
| Agility | 0–100 | Speed, stealth, infiltration |
| Intelligence | 0–100 | Hacking, analysis, planning |

**Job bonuses (applied at squad assembly):**

| Job | Bonus | Niche |
|-----|-------|-------|
| **Breacher** | +10 STR | Raids, extractions, door-kickers |
| **Infiltrator** | +10 AGI | Recon, stealth insertions |
| **Analyst** | +10 INT | Zero-days, corporate espionage |

**Operator lifecycle:**
```
Idle ──► Deployed ──► Idle (Victory)
                 └──► Injured (cooldown = duration × 2 seconds)
                 └──► KIA (permanently removed from roster)
```

### 4.2 Missions / Contracts

| Field | Description |
|-------|-------------|
| Name | Flavour identifier |
| STR / AGI / INT threshold | Stat the squad must collectively exceed |
| Difficulty | 0.0–0.9 penalty scalar on base success% |
| Duration | Wall-clock seconds (30s–300s in MVP) |
| Reward | Credits on Victory |

**MVP Mission Pool:**

| Mission | STR | AGI | INT | Diff | Duration | Reward |
|---------|-----|-----|-----|------|----------|--------|
| Bank Heist Recon | 20 | 30 | 10 | 10% | 60s | $500 |
| Corporate Espionage | 10 | 20 | 50 | 25% | 120s | $1,200 |
| Harbour Extraction | 40 | 20 | 10 | 20% | 90s | $800 |
| Zero-Day Exploit | 10 | 10 | 70 | 40% | 180s | $2,500 |
| Black Site Breach | 60 | 40 | 20 | 50% | 300s | $5,000 |

### 4.3 Success Formula

```
per_attr_score   = min(squad_total / threshold, 1.0)
                   [0.0 if threshold > 0 and squad contributes nothing]
average_score    = (str_score + agi_score + int_score) / 3.0
success_chance   = average_score × (1.0 − difficulty)
```

| Roll result | Outcome |
|-------------|---------|
| `< success_chance` | **Victory** — full reward |
| `≥ success_chance` AND `< 0.95` | **Failure** — all operators injured |
| `≥ 0.95` | **Critical Failure** — one operator KIA |

> The 5% Critical Failure floor is permanent, documented, and shown to players. No squad is ever "safe."

---

## 5. The Genetics System (Layer B Detail)

### 5.1 Cultural Archetypes

Six cultures arranged on a **hexagon wheel**. Position determines genetic tier when blended.

```
          GALE (speed · blue)
    EMBER               CRYSTAL
 (attack · red)      (tank · white)
    MARSH               TIDE
 (balanced · green) (electric · blue)
          TUNDRA (endure · cool)

Opposites: Ember↔Crystal | Gale↔Tundra | Marsh↔Tide
```

**Culture stat modifiers:**

| Culture | HP | ATK | SPD | Rare trait |
|---------|----|-----|-----|------------|
| Ember   | ×0.8 | ×1.4 | ×1.1 | 5% |
| Gale    | ×0.9 | ×0.9 | ×1.4 | 6% |
| Marsh   | ×1.0 | ×0.9 | ×1.3 | 4% |
| Crystal | ×1.4 | ×0.8 | ×0.7 | 8% |
| Tundra  | ×1.1 | ×0.9 | ×0.8 | 5% |
| Tide    | ×1.0 | ×1.0 | ×1.2 | 7% |
| Void    | ×1.2 | ×1.2 | ×1.2 | 25% |

### 5.2 The Genetic Tier Ladder (The Core Progression)

Tier is determined by how many cultures are "active" (expression ≥ 5%) in a slime's genome, and their hexagon relationship.

| Tier | Name | Active cultures | How |
|------|------|-----------------|-----|
| 1 | **Blooded** | 1 | Pure breed |
| 2 | **Bordered** | 2 | Adjacent on hex |
| 3 | **Sundered** | 2 | Opposite on hex |
| 4 | **Drifted** | 2 | Skip-one on hex |
| 5 | **Threaded** | 3 | Any three |
| 6 | **Convergent** | 4 | Any four |
| 7 | **Liminal** | 5 | Any five |
| 8 | **Void** | 6 | All six expressed |

> **Design intent:** Void-tier slimes require 7+ generations of deliberate cross-breeding. They are the game's ultimate flex — rare enough to feel legendary, attainable enough to keep players breeding.

### 5.3 The Ratchet Effect (Anti-Frustration Core)

Every stat inherits slightly higher than the best parent:

```
HP  → max(parent_A, parent_B) × +10% toward cap
ATK → average(parent_A, parent_B) × +10% toward cap
SPD → max(parent_A, parent_B) × 0.95 × +10% toward cap

Cap = base_stat × culture_modifier × 2.0
```

Stats can never regress below the peak parent. Void parentage amplifies mutation rate to ≥15%.

### 5.4 Slime Lifecycle

| Level | Stage | Dispatch? | Breed? | Mentor? |
|-------|-------|:---------:|:------:|:-------:|
| 0–1 | Hatchling | ✗ | ✗ | ✗ |
| 2–3 | Juvenile | ✓ (low risk) | ✗ | ✗ |
| 4–5 | Young | ✓ | ✓ | ✗ |
| 6–7 | Prime | ✓ | ✓ | ✗ |
| 8–9 | Veteran | ✓ (high risk) | ✓ | ✗ |
| 10 | Elder | ✓ (critical) | ✓ | ✓ |

**XP curve:** `xp_to_next_level = (level + 1) × 100`
**Elder bonus:** 20% extra chance of rare accessory on offspring.

### 5.5 Special Stage × Tier Interactions

| Combo | Tag | Effect |
|-------|-----|--------|
| Sundered + Prime | `volatile_peak` | Mission bonus TBD |
| Liminal + Elder | `threshold_legacy` | Mentoring bonus TBD |
| Any Void tier | `primordial_X` | Amplified mutation for all offspring |

---

## 6. The War Room Dashboard (Tier 3) ✅

The `operator gui` command opens a **three-column egui window**:

| Column | Content |
|--------|---------|
| **Unit Roster** | Operator cards: stat badge, state indicator, injury countdown, stage/STAGE buttons |
| **Active Operations** | Animated progress bars (wall-clock math, 100ms repaint), ⚡ PROCESS AAR button |
| **Available Contracts** | Mission cards with difficulty color coding, SELECT button |

**Bottom panels:**
- **Launch bar:** staged squad → "🚀 LAUNCH MISSION" button with live success% preview
- **Combat Log:** scrollable narrative AAR entries, color-coded by outcome, resizable

---

## 7. The Narrative Engine (Story Layer) ✅

After every AAR, a flavor narrative line is generated from the outcome and mission type.

**Mission type classification** (from dominant stat requirement):
- STR-dominant → Assault pool ("The door never had a chance…")
- AGI-dominant → Stealth pool ("Ghost bypassed the heat sensors…")
- INT-dominant → Cyber pool ("The zero-day executed flawlessly…")
- Balanced → General pool

**Operator name injection:** `{op}` → first squad member's name.

---

## 8. Tier Roadmap

| Tier | Name | Status | Theme |
|------|------|--------|-------|
| 1 | Headlong (MVP CLI) | ✅ | "Does the math feel fair?" |
| 2 | Persistence Layer | ✅ | "Does it feel like a career?" |
| 3 | War Room Dashboard | ✅ | "The Mafia Wars dopamine loop" |
| 3b | Story Engine | ✅ | "Did that feel like something?" |
| S1 | Genetics Engine | ✅ | "Is this game alive?" |
| S2 | D20 Combat Core | 🔄 Sprint | "Did my squad actually fight?" |
| S3 | Island Expedition | 🔄 Sprint | "Is there somewhere to go?" |
| 4 | Balance & Economy | ⬜ Post-sprint | "Is this a game loop?" |
| 5 | Mobile / Cross-platform | ⬜ Stretch | "Can I check this on my phone?" |

---

## 9. Economy (Current + Planned)

| Action | Credit Effect |
|--------|--------------|
| Mission Victory | +reward |
| Hire Operator | Free (Tier 4: add cost gate) |
| Hatch a Slime | Free (Tier 4: cost per culture rarity) |
| Injury Downtime | Opportunity cost only |
| Operator/Slime KIA | Loss of all invested stats |

---

## 10. Out of Scope (MVP Tiers 1–S1)

- Multiplayer / trade market
- Sound design
- Localization
- Economy hire-cost gate (Tier 4)
- Island Expedition (Sprint 3)
- Mobile native APK (Stretch)

---

## 11. Glossary

| Term | Definition |
|------|-----------|
| **Operator** | A named human mercenary with STR/AGI/INT stats and a job |
| **Slime** | A biological entity with a genome (culture expression, stats, traits) |
| **Culture** | One of 7 elemental archetypes (Ember/Gale/Marsh/Crystal/Tundra/Tide/Void) |
| **CultureExpression** | A 6-float vector summing to 1.0, representing genetic blending |
| **Genetic Tier** | 1–8 classification based on how many cultures are "active" in a genome |
| **Contract / Mission** | A timed task with attribute requirements and a payout |
| **Squad** | 1–3 Operators assigned to a Mission |
| **Deployment** | Active record of a Squad on a Mission, with a `completes_at` timestamp |
| **AAR** | After Action Report — outcome resolution |
| **KIA** | Killed In Action — permanent removal |
| **Ratchet Effect** | The breeding rule that prevents stat regression across generations |
| **Splice** | The act of breeding two slimes to produce an offspring genome |
| **Void-tier** | A slime expressing all 6 cultures — the highest genetic tier |
