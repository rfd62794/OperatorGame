# OPERATOR — Game Design Document (GDD)
> **Version:** 1.0 | **Status:** Tier 1–2 Spec | 2026-03-04
> Working Title: *OPERATOR*

---

## 1. Executive Summary

**OPERATOR** is a single-player, text-driven **dispatch simulator** with idle-game mechanics. The player builds and maintains a mercenary roster, assembles squads for high-stakes contracts, and collects rewards — all while managing injury, death, and economic pressure.

The core emotional loop is:
> *"Can I afford to send my best on a risky mission, or do I play it safe and take the smaller payday?"*

---

## 2. Design Pillars

| Pillar | Description |
|--------|-------------|
| **Risk is Real** | Permanent operator death at a 5% floor. No save-scumming. |
| **Respect Player Time** | Missions progress while the app is closed. Open the app when you're ready, not because you have to. |
| **Composition Matters** | Sending the wrong job on a mission is a genuine strategic mistake, not just a minor penalty. |
| **Readable Systems** | Every formula must be explainable in plain English to the player. No hidden RNG multipliers. |

---

## 3. Core Game Loop

```
┌─────────────────────────────────────────────────────┐
│  1. ROSTER MANAGEMENT                               │
│     Review available Operators (Job / Stats / State)│
├─────────────────────────────────────────────────────┤
│  2. CONTRACT SELECTION                              │
│     Browse missions — weigh requirements vs. risk   │
├─────────────────────────────────────────────────────┤
│  3. SQUAD ASSEMBLY                                  │
│     Assign 1–3 Operators. See success% preview.     │
├─────────────────────────────────────────────────────┤
│  4. DEPLOYMENT                                      │
│     Mission locks in. Timer starts. You can log off.│
├─────────────────────────────────────────────────────┤
│  5. AFTER ACTION REPORT (AAR)                       │
│     Resolve outcome: Reward / Injury / KIA          │
└─────────────────────────────────────────────────────┘
         ↑                                   │
         └───── Spend rewards → Hire new ────┘
```

---

## 4. Game Systems

### 4.1 Operators

Operators are the player's primary resource. They are persistent, named, and **permanently mortal**.

#### Attributes
| Stat | Range | Description |
|------|-------|-------------|
| Strength | 0–100 | Raw force, breaching, physical tasks |
| Agility | 0–100 | Speed, stealth, infiltration |
| Intelligence | 0–100 | Hacking, analysis, planning |

#### Jobs
Jobs apply a flat +10 bonus to one attribute. This makes them specialists, not generalists.

| Job | Bonus | Niche |
|-----|-------|-------|
| **Breacher** | +10 Strength | Extractions, raids, door-kickers |
| **Infiltrator** | +10 Agility | Recon, stealth insertions, escapes |
| **Analyst** | +10 Intelligence | Zero-day exploits, corporate espionage |

#### Operator States
```
Idle ──► Deployed (on mission) ──► Idle (Victory)
                              └──► Injured (cooldown = duration × 2)
                              └──► KIA (removed from roster permanently)
```

### 4.2 Missions

Missions are contracts with defined attribute requirements and a real-time duration.

#### Mission Fields
| Field | Description |
|-------|-------------|
| Name | Flavour identifier |
| req_strength / agi / int | Attribute thresholds the squad must meet |
| Difficulty | 0.0–0.9 scalar penalty on success rate |
| Duration | Wall-clock seconds until completion (30s–300s in MVP) |
| Reward | Payout in credits on Victory |

#### MVP Mission Pool
| Mission | STR | AGI | INT | Diff | Duration | Reward |
|---------|-----|-----|-----|------|----------|--------|
| Bank Heist Recon | 20 | 30 | 10 | 10% | 60s | $500 |
| Corporate Espionage | 10 | 20 | 50 | 25% | 120s | $1,200 |
| Harbour Extraction | 40 | 20 | 10 | 20% | 90s | $800 |
| Zero-Day Exploit | 10 | 10 | 70 | 40% | 180s | $2,500 |
| Black Site Breach | 60 | 40 | 20 | 50% | 300s | $5,000 |

### 4.3 Success Formula

```
Per-attribute score  = min(squad_total / req_threshold, 1.0)
                       (0.0 if req > 0 and squad contributes nothing)

Average score        = (str_score + agi_score + int_score) / 3.0

Success chance       = average_score × (1.0 - difficulty)

Roll                 = random f64 in [0.0, 1.0)
```

#### Outcome Table
| Condition | Outcome |
|-----------|---------|
| `roll < success_chance` | **Victory** — full reward paid |
| `roll ≥ success_chance` AND `roll < 0.95` | **Failure** — all operators Injured for `duration × 2` |
| `roll ≥ 0.95` | **Critical Failure** — one random operator KIA |

> **Design note:** The 5% Critical Failure floor is intentional and communicated to players. Perfect squads still face 1-in-20 odds of losing someone.

### 4.4 Economy

| Action | Credit Effect |
|--------|--------------|
| Start of game | $0 |
| Mission Victory | +reward |
| Hire Operator | −hire_cost *(Tier 2 — TBD)* |
| Injury Downtime | Opportunity cost only (no fee) |
| Operator KIA | Loss of all invested stats |

*Tier 1 hire is free — economic gate added in Tier 2 balance pass.*

---

## 5. Tier Roadmap

### Tier 1 — Headlong (MVP CLI) ✅ Complete
**Theme:** "Does the math feel fair?"
- Hardcoded seed missions
- CLI commands: roster / hire / missions / deploy / aar / status
- Atomic JSON persistence
- 11 unit tests passing

### Tier 2 — Persistence Layer ✅ Complete
**Theme:** "Does it feel like a career?"
- Roster persists across sessions
- Bank balance persists
- Hire via CLI
- Offline-safe timers (app restarts don't pause missions)

### Tier 3 — Egui Dashboard ⬜ Planned
**Theme:** "The Mafia Wars dopamine loop"
- `eframe` window with three-column "War Room" layout
- **Roster panel:** Operator cards with stat bars, state badge, injury countdown
- **Missions panel:** Contract cards with "Deploy" button → Squad Picker modal
- **Operations panel:** Live progress bars from `completes_at` timestamp
- Polling loop: `ctx.request_repaint_after(Duration::from_secs(1))`

### Tier 4 — Balance & Content ⬜ Post-MVP
- Procedural mission generation
- Hire cost economy (credits gate recruitment)
- Operator XP and level-up (stats grow with mission count)
- Equipment slots (passive stat bonuses)
- Mission types: Recon (low-risk, low-reward) vs. Black Op (high-risk, high-reward)

### Tier 5 — Narrative Layer ⬜ Stretch Goal
- Operator backstories and morale system
- Mission logs / after-action narrative text
- "World events" that modify the mission pool weekly

---

## 6. Player Experience Goals

| Goal | Mechanism |
|------|-----------|
| "One more mission" loop | Short missions (60–90s) completable in a coffee break |
| Emotional attachment to operators | Named characters, permanent death, stat investment |
| Strategic depth without complexity | Three-attribute system anyone can understand |
| No FOMO | App-closed progress — you play when you want |

---

## 7. Out of Scope (MVP Boundaries)

The following are explicitly **not** in scope for Tiers 1–2:

- Multiplayer / trade market
- Procedural generation
- Sound design
- Localization
- Mobile native ports
- Economy balancing (no hire cost in Tier 1)

---

## 8. Glossary

| Term | Definition |
|------|------------|
| **Operator** | A named mercenary on the player's roster |
| **Contract / Mission** | A timed task with attribute requirements and a payout |
| **Squad** | 1–3 Operators assigned to a single Mission |
| **Deployment** | The active record of a Squad on a Mission, with a `completes_at` timestamp |
| **AAR** | After Action Report — the resolution screen where outcomes are collected |
| **KIA** | Killed In Action — permanent removal from roster |
| **Difficulty** | A 0.0–0.9 scalar penalty applied after the attribute scoring step |
