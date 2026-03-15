# SlimeGarden --- UI Tab Scaffold Design

**Purpose:** Define exact content, data sources, and interactions for each of the 4 main tabs + sub-tabs.  
**Scope:** MVP (Colors, Patterns, Conquest)  
**Data Sources:** GameState (slimes, missions, zones, inventory), Operator stats, Mission outcomes  
**Visual Language:** Dark industrial, color-coded by culture, stat-focused, text-heavy (no animations, MVP)  

---

## TAB 1: ROSTER

**Purpose:** View and manage slime collection. Primary hub for team composition, breeding, and gear assignment.

**Sub-Tabs:**
- Collection (default)
- Breeding

---

### Sub-Tab 1.1: Collection

**Header Bar:**
```
ROSTER: Collection | [Breeding] | Total: 5 slimes
```

**Main Content: Grid of Operator Cards**

**Card Layout (3-4 per row on mobile, 4-6 on desktop):**
```
┌─────────────────────┐
│ [Spark]         Red │
│ Basic Pattern       │
│ Lv: 3  XP: ███░░░  │
│ STR:12 AGI:8 INT:6 │
│ HP: 25   [Stage]    │
└─────────────────────┘
```

**Card Content (per operator):**
- **Name:** Operator name (e.g., "Spark", "Echo")
- **Culture:** Color swatch + culture name (e.g., red swatch + "Ember")
- **Pattern:** Pattern name (e.g., "Spotted", "Striped") — visual identifier only
- **Level & XP:** Current level, XP progress bar (visual gauge to next level)
- **Hard Stats:** STR, AGI, INT (abbreviated, aligned right)
- **HP:** Current HP (abbreviated)
- **Action Button:** [Stage] — toggle to add/remove from active party

**Card Interactions:**
- **Tap card body** → Expand to Detail View (overlay or new panel)
- **Tap [Stage] button** → Toggle party membership (visual highlight: green if staged, gray if not)
- **Long-press card** → Open context menu (Equip, Release, Breed, View Genome)

**Detail View (Expanded):**
```
┌── SPARK (Detail) ──────────────────────────┐
│ Culture: Ember (Red) | Pattern: Spotted   │
│                                            │
│ STATS:                                     │
│  Hard: STR 12 | AGI 8 | INT 6             │
│  Soft: MND 5 | SEN 4 | TEN 3              │
│  HP: 25 | Level: 3 | XP: 45/100           │
│                                            │
│ GENETICS:                                  │
│  Tier: 3 (Rising)                         │
│  Culture Expression: [visual bar]         │
│  Parents: [Father ID] [Mother ID]         │
│                                            │
│ GEAR SLOTS:                                │
│  Weapon: [Scout Fins] (AGI +2)            │
│  Armor: [none]                            │
│  Accessory: [none]                        │
│  [EQUIP GEAR] button                      │
│                                            │
│ ACTIONS: [Breed] [Release] [Close]        │
└────────────────────────────────────────────┘
```

**Party Indicator (Bottom of screen):**
```
Active Party: 2/3
[Spark - Ember Lv3] [Echo - Tide Lv2] [Empty Slot]
[Change Party Composition]
```

**Data Sources:**
- `GameState.slimes: Vec<Operator>` — All operators
- `GameState.party: Vec<Uuid>` — Current active party
- Per operator: `stats`, `genome`, `level`, `xp`, `equipped_gear`, `pattern`

---

### Sub-Tab 1.2: Breeding

**Header Bar:**
```
ROSTER: [Collection] | Breeding | Incubator Timer
```

**Main Content: Breeding Pair Manager**

**Current Pair Section:**
```
┌── CURRENT BREEDING PAIR ──────────────────┐
│                                           │
│ King: [Spark - Ember Lv3]                │
│ Queen: [Echo - Tide Lv2]                 │
│                                           │
│ Predicted Outcome:                        │
│  Culture: Likely Orange (blend)          │
│  Pattern: High chance Spotted (35%)      │
│  Tier: 3 (from parents)                  │
│                                           │
│ Incubation Timer: 2h 15m remaining       │
│ [████████░░░░░░░░░░░░░░░░░░]            │
│                                           │
│ Status: Incubating                       │
│ [CANCEL BREEDING] button (red)           │
│                                           │
└───────────────────────────────────────────┘
```

**If Timer Expired (Ready to Hatch):**
```
┌── EGG READY TO HATCH ─────────────────────┐
│                                           │
│ Hatchling Stats (Preview):                │
│  Culture: Orange (Orange + Spotted)      │
│  STR: 11 | AGI: 9 | INT: 7               │
│  HP: 26                                   │
│                                           │
│ [🎉 NEW PATTERN DISCOVERED!]             │
│ Pattern Regent awarded: "Spotted"        │
│                                           │
│ [HATCH SLIME] button (green)             │
│                                           │
└───────────────────────────────────────────┘
```

**Change Breeding Pair Section (if no active pair):**
```
┌── START NEW BREEDING PAIR ────────────────┐
│                                           │
│ Select King:                              │
│  [Spark - Ember Lv3]▼ (dropdown)         │
│                                           │
│ Select Queen:                             │
│  [Echo - Tide Lv2]▼ (dropdown)           │
│                                           │
│ [PREVIEW] [START BREEDING] buttons       │
│                                           │
└───────────────────────────────────────────┘
```

**Pattern Regent Inventory (Below Breeding):**
```
┌── PATTERN REGENTS ────────────────────────┐
│ (Consumable discovery tokens)            │
│                                           │
│ Spotted: ×1  [Use on Spark]              │
│ Striped: ×2  [Use on Echo]               │
│ Speckled: ×0                             │
│ Rare: ×1     [Use on Jumper]             │
│                                           │
└───────────────────────────────────────────┘
```

**Data Sources:**
- `GameState.slimes: Vec<Operator>` — Available slimes for pairing
- Current breeding state: pair IDs, timer (chrono UTC), predicted outcome
- `GameState.regents: HashMap<PatternName, usize>` — Regent inventory

---

## TAB 2: MISSIONS

**Purpose:** View active missions, select new quests, and dispatch squads. Central hub for progression and conquest.

**Sub-Tabs:**
- Active (default)
- Quest Board

---

### Sub-Tab 2.1: Active

**Header Bar:**
```
MISSIONS: Active | [Quest Board] | 2 in-progress, 1 ready to return
```

**Main Content: List of Active Deployments**

**Deployment Card (per active mission):**
```
┌─────────────────────────────────────────────┐
│ DEPLOYMENT #47: Scout the Tundra           │
│ Zone: Crystal Expanse (Ring 2)             │
│ Squad: [Spark - Ember Lv3] [Echo - Tide]  │
│                                             │
│ Difficulty: 7/10                           │
│ Status: In Progress                        │
│ Time Remaining: 45 minutes                 │
│ Progress: [██████████████░░░░░░░░░░]       │
│                                             │
│ Expected Rewards:                           │
│  Gel: 75 | Scrap: 10 | Reagent: 3         │
│  Possible Culture: Marsh (rare)            │
│                                             │
│ Actions: [View Details] [Cancel] [Recall] │
└─────────────────────────────────────────────┘
```

**If Mission Complete (Ready to Return):**
```
┌─────────────────────────────────────────────┐
│ ⭐ MISSION COMPLETE: Scout the Tundra     │
│ Zone: Crystal Expanse (Ring 2)             │
│ Squad: [Spark] [Echo]                      │
│                                             │
│ OUTCOME: Victory!                          │
│                                             │
│ Squad Health: Spark 18/25 | Echo 22/28    │
│ XP Gained: Spark +45 | Echo +45            │
│                                             │
│ LOOT:                                       │
│  Gel: 75 | Scrap: 10 | Reagent: 3         │
│  NEW SLIME JOINED: "Jumper" (Gale culture)│
│                                             │
│ AAR (Auto-Generated):                      │
│ "Spark and Echo advanced into the expanse,│
│  disrupting a Tundra nest. Unexpectedly, │
│  Jumper (a Gale scout) joined the squad." │
│                                             │
│ [PROCESS & RETURN] button (green)          │
│                                             │
└─────────────────────────────────────────────┘
```

**No Active Missions:**
```
┌─────────────────────────────────────────────┐
│ No active missions. Ready to dispatch!      │
│                                             │
│ [Go to QUEST BOARD] button                 │
│                                             │
└─────────────────────────────────────────────┘
```

**Data Sources:**
- `GameState.deployments: Vec<Deployment>` — Active missions
- Per deployment: squad members, zone, difficulty, timer (wall-clock), expected rewards
- Outcomes: `AarOutcome` with loot, XP, recruited slimes, narrative

---

### Sub-Tab 2.2: Quest Board

**Header Bar:**
```
MISSIONS: [Active] | Quest Board | Filter: [All] [Ring 1] [Ring 2] [By Culture]
```

**Main Content: Available Missions (Grid or List)**

**Quest Card:**
```
┌─────────────────────────────────────────────┐
│ QUEST #12: Defend the Hive                 │
│ Zone: Ember Foundry (Ring 1, Ember culture)│
│                                             │
│ Difficulty: 5/10                           │
│ Duration: ~20 min (wall-clock)             │
│                                             │
│ REQUIREMENTS:                               │
│  STR ≥ 10 | AGI ≥ 6 | INT ≥ 5             │
│  ⭐ Affinity Bonus: Ember culture (+5%)    │
│  Party Size: 1-3 slimes                    │
│                                             │
│ REWARDS:                                    │
│  Gel: 50 | Scrap: 5 | Reagent: 1         │
│  Possible Culture: Ember (60% Lv1 join)   │
│                                             │
│ Your Party:                                 │
│  [Spark: STR12✓ AGI8✓ INT6✓] ← Ready      │
│  [Echo: STR8✗ AGI7✓ INT5✓]   ← Low STR   │
│  [Jumper: STR11✓ AGI9✓ INT4✓] ← Ready    │
│                                             │
│ [SELECT SQUAD] [DISPATCH] buttons          │
│                                             │
└─────────────────────────────────────────────┘
```

**Squad Composition UI (Modal/Overlay when [SELECT SQUAD] clicked):**
```
┌─ SELECT SQUAD FOR: Defend the Hive ─────┐
│ Requirements: STR≥10 AGI≥6 INT≥5        │
│ Party Size: 1-3 slimes                  │
│                                          │
│ Available:                               │
│ [✓] Spark - Ember Lv3                   │
│     STR:12✓ AGI:8✓ INT:6✓ | Ready      │
│                                          │
│ [ ] Echo - Tide Lv2                     │
│     STR:8✗ AGI:7✓ INT:5✓ | Low STR     │
│                                          │
│ [✓] Jumper - Gale Lv2                   │
│     STR:11✓ AGI:9✓ INT:4✓ | Ready      │
│                                          │
│ Selected: 2/3                            │
│ Status: Ready to dispatch ✓              │
│                                          │
│ [DISPATCH NOW] [CANCEL] buttons          │
│                                          │
└──────────────────────────────────────────┘
```

**Data Sources:**
- `GameState.missions: Vec<Mission>` — Available quests
- Per mission: name, zone, `req_strength/agility/intelligence`, difficulty, duration, rewards, affinity bonus
- Player's `GameState.party` — For validation against requirements

---

## TAB 3: MAP

**Purpose:** View world zones, affinity bonuses, and progression gates. Strategic overview.

**Sub-Tabs:**
- Zones (default)

---

### Sub-Tab 3.1: Zones

**Header Bar:**
```
MAP: Zones | Ring Filter: [All] [Ring 1] [Ring 2] [Ring 3] | Unlocked: 3/8
```

**Main Content: Zone List (Radial Ring Structure)**

**Ring 1 (Heartlands) — Unlocked:**
```
┌── RING 1: HEARTLANDS ─────────────────────┐
│                                            │
│ ✓ Ember Foundry (Ember culture)          │
│   Difficulty: 5/10                       │
│   Affinity Bonus: Ember +5%              │
│   Unlocked: Yes                          │
│   [View Quests] button                   │
│                                            │
│ ✓ Tide Shallows (Tide culture)           │
│   Difficulty: 5/10                       │
│   Affinity Bonus: Tide +5%               │
│   Unlocked: Yes                          │
│                                            │
│ ✓ Gale Peaks (Gale culture)              │
│   Difficulty: 5/10                       │
│   Affinity Bonus: Gale +5%               │
│                                            │
└────────────────────────────────────────────┘
```

**Ring 2 (The Wilds) — Partially Unlocked:**
```
┌── RING 2: THE WILDS ──────────────────────┐
│                                            │
│ ✓ Crystal Caverns (Crystal culture)      │
│   Difficulty: 7/10                       │
│   Affinity Bonus: Crystal +5%            │
│   Unlocked: Yes                          │
│   [View Quests] button                   │
│                                            │
│ 🔒 Marsh Mire (Marsh culture)            │
│    Difficulty: 7/10                      │
│    Affinity Bonus: Marsh +5%             │
│    Unlock Requirement: Breed 2ndary color│
│    (Need: Orange or Green slime)         │
│    [Learn More] button                   │
│                                            │
│ 🔒 Tundra Wastes (Tundra culture)        │
│    Difficulty: 8/10                      │
│    ...                                    │
│                                            │
└────────────────────────────────────────────┘
```

**Ring 3 (The Beyond) — Locked:**
```
┌── RING 3: THE BEYOND ────────────────────┐
│                                           │
│ 🔒 All zones locked                      │
│ Unlock Requirement: Reach Tier 5+ slimes │
│                                           │
└───────────────────────────────────────────┘
```

**Zone Detail (Tap Zone Card):**
```
┌─ CRYSTAL CAVERNS (Zone Detail) ──────────┐
│                                           │
│ Culture: Crystal | Difficulty: 7/10     │
│ Ring: 2 (The Wilds)                     │
│ Affinity Bonus: Crystal +5%             │
│                                           │
│ AVAILABLE QUESTS:                         │
│  1. Scout Crystal Vein (⭐ New)          │
│  2. Defend the Nest                      │
│  3. Harvest Resonant Shards              │
│  [View All Quests] button                │
│                                           │
│ ECOLOGY:                                  │
│ - Primary Culture: Crystal               │
│ - Danger: Resonance Disturbance          │
│ - Resources: Gel, Reagent                │
│                                           │
│ RECRUITMENT ODDS:                         │
│ - Common: Crystal Lv1 (60%)              │
│ - Rare: Marsh (10%)                      │
│ - Very Rare: Orange Lv2 (2%)             │
│                                           │
│ [BACK] [Go to Quest Board] buttons       │
│                                           │
└───────────────────────────────────────────┘
```

**Data Sources:**
- `GameState.world_map.zones: Vec<Zone>` — All zones with affinity, difficulty, unlocked status
- `GameState.zones_unlocked: Vec<ZoneId>` — Which zones are accessible
- Per zone: culture, ring, affinity bonus, available quests, recruitment table

---

## TAB 4: LOGS

**Purpose:** View mission history, culture progression, and narrative outcomes. Long-term record.

**Sub-Tabs:**
- Mission History (default)
- Culture History

---

### Sub-Tab 4.1: Mission History

**Header Bar:**
```
LOGS: Mission History | [Culture History] | Total Missions: 23 (18 Win, 5 Loss)
```

**Main Content: Scrollable Log of Past Missions**

**Mission Log Entry (Compact):**
```
┌─────────────────────────────────────────────────┐
│ ✓ MISSION #23: Defend the Hive                 │
│   Zone: Ember Foundry | Victory               │
│   Squad: Spark (Ember), Echo (Tide), Jumper   │
│   Date: 2026-03-14 14:30 UTC                  │
│   Loot: Gel +50 | Scrap +5 | Reagent +1      │
│   [View AAR] button                           │
│                                                 │
└─────────────────────────────────────────────────┘
```

**Full AAR (Tap [View AAR]):**
```
┌─ MISSION AAR: Defend the Hive ───────────────┐
│                                               │
│ Squad: Spark (STR12), Echo (STR8), Jumper    │
│ Zone: Ember Foundry (Diff 5/10)              │
│ Outcome: VICTORY                             │
│                                               │
│ COMBAT LOG:                                   │
│ Round 1:                                      │
│  Spark attacks: [d20: 18] + [STR: 12] = 30  │
│  vs Enemy DC: 10 → HIT!                      │
│  Enemy attacks: [d20: 9] + [Mod: 5] = 14    │
│  vs Spark DEF: 12 → MISS!                    │
│                                               │
│ Round 2:                                      │
│  Echo attacks: [d20: 7] + [STR: 8] = 15    │
│  vs Enemy DC: 10 → HIT!                      │
│  ...                                          │
│                                               │
│ NARRATIVE (Auto-Generated):                   │
│ "Spark and Echo advanced into the foundry,   │
│  disrupting the Ember nests. Their tactics   │
│  proved effective; Jumper's Gale affinity    │
│  provided unexpected advantage. The colony   │
│  was secured, and a young scout (Jumper)     │
│  recognized their strength and joined them." │
│                                               │
│ REWARDS:                                      │
│  Gel: 50 | Scrap: 5 | Reagent: 1            │
│  XP: Spark +45 | Echo +45 | Jumper +40      │
│  New Recruit: Jumper (Gale, Lv1)            │
│                                               │
│ [CLOSE] button                                │
│                                               │
└───────────────────────────────────────────────┘
```

**Data Sources:**
- `GameState.deployments` (completed) — Past missions
- Per mission: `AarOutcome` with rolls, narrative, rewards, recruits

---

### Sub-Tab 4.2: Culture History

**Header Bar:**
```
LOGS: [Mission History] | Culture History | Cultures Encountered: 5/9
```

**Main Content: Culture Acquisition Timeline**

**Culture Card (per culture discovered):**
```
┌─ EMBER (Primary Culture) ─────────────────┐
│ Status: Active (5 slimes in roster)       │
│ First Encountered: Mission #1             │
│ Date: 2026-03-01                          │
│                                            │
│ Slimes with Ember:                         │
│  • Spark (Pure Ember, Lv3)                │
│  • Echo (Ember/Tide blend, Lv2)           │
│  • [Recruit from Mission #5]              │
│                                            │
│ Affinity Bonus (Ember Zones): +5%         │
│                                            │
└────────────────────────────────────────────┘
```

**Locked Culture Card (not yet encountered):**
```
┌─ VOID (Tertiary Culture) ──────────────────┐
│ Status: Locked                             │
│ Rarity: Legendary                          │
│                                             │
│ Unlock Path:                                │
│  Reach Tier 8 with any slime               │
│  Breed Tier 7 + Tier 7 → Void possible    │
│                                             │
│ Hint: "The resonance of pure entropy..."   │
│                                             │
└────────────────────────────────────────────┘
```

**Culture Progression Chart (Visual):**
```
CULTURE DISCOVERY TIMELINE:
2026-03-01: Ember (primary)
2026-03-02: Tide (primary)
2026-03-03: Gale (primary)
2026-03-05: Orange (secondary: Ember + Tide)
2026-03-08: Crystal (secondary: Tide + Gale)
2026-03-??: [Locked] Marsh (secondary)
2026-03-??: [Locked] Tundra (tertiary)
2026-03-??: [Locked] Void (tertiary)
```

**Data Sources:**
- `GameState.patterns_discovered: HashSet<String>` — Cultures encountered
- Per culture: first encounter date, slimes with that culture, affinity bonus, unlock requirements

---

## Data Flow Summary

### State Mutations

**Roster Tab:**
- Stage slime → `GameState.party` updated
- Hatch egg → New `Operator` added to `GameState.slimes`, regent consumed
- Equip gear → Operator's `equipped_gear` updated

**Missions Tab:**
- Dispatch squad → New `Deployment` added to `GameState.deployments`, timer starts
- Mission completes → `AarOutcome` processed, slimes gain XP, loot added, zones unlocked, regents/recruits granted

**Map Tab:**
- No mutations (read-only view)

**Logs Tab:**
- No mutations (read-only view)

---

## UI Constants & Styling

**Screen Dimensions (Portrait, Moto G 2025):**
- Screen width: 412dp
- Safe area top: 48dp
- Safe area bottom: 56dp
- Usable height: ~1900dp

**Layout:**
- Top bar: Fixed, 60dp (OPERATOR: COMMAND DECK + resources)
- Sidebar: 100dp (sub-tabs)
- Content area: Remaining width (312dp on mobile)
- Bottom bar: Fixed, 48dp (4 tabs)

**Card Dimensions (Mobile):**
- 3-4 cards per row on 312dp → ~90-100dp per card
- Slime cards: 100×120dp (portrait orientation)

**Typography:**
- Header: 16pt, bold
- Body: 12pt, regular
- Stats: 10pt, monospace

**Color Palette:**
- Background: `#0f0f14` (dark)
- Panel: `#1a1a22` (slightly lighter)
- Text: `#ffffff` (white)
- Culture colors: Red (Ember), Blue (Tide), Green (Gale), etc. (from GDD)
- Status: Green (success), Red (failure), Yellow (warning)

---

## Acceptance Criteria for Tab Scaffold

✓ Roster: Collection grid displays all slimes with stats, patterns, and party toggle  
✓ Roster: Breeding sub-tab shows current pair, incubator timer, hatch UI, regent inventory  
✓ Missions: Active sub-tab lists deployments with timer and completion UI  
✓ Missions: Quest Board lists available quests with stat requirements and affinity bonus  
✓ Missions: Squad composition validator checks requirements before dispatch  
✓ Map: Zone list shows rings, unlock status, and affinity bonuses  
✓ Map: Zone detail view shows available quests and recruitment odds  
✓ Logs: Mission history shows past missions with AAR detail  
✓ Logs: Culture history shows discovered cultures and acquisition timeline  
✓ All tabs have clear data sources (GameState fields) mapped  
✓ All interactions (stage, dispatch, breed, hatch) are defined  
✓ UI is optimized for portrait mobile (3-4 items per row, touch-friendly)  

---

## Next Step

This scaffold defines **what each tab shows** and **what data feeds it**. The next phase (Phase E) will implement the **game mechanics** (breeding, dispatch, leveling) that populate and update these views.

Once Phase E is complete, the UI will be fully functional: users can recruit, breed, dispatch, and see their progress reflected across all tabs.
