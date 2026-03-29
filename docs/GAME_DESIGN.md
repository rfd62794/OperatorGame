# OperatorGame: Game Design Document

## 1. Vision Statement
OperatorGame is a tactical strategy game about breeding and deploying adorable slimes on increasingly absurd "Gauntlet" missions. Players manage a high-stakes roster of bio-engineered operators, optimizing their genetics and equipment to survive sequential hazards on a living planet map.

## 2. Core Pillars
- **Strategy**: Squad composition and mission tactics (risk management) are the keys to victory.
- **Genetics**: Breeding and inheritance are the primary long-term progression engines.
- **Risk**: Injuries and partial success create tension; every roll matters.
- **Charm**: Whimsical slimes in absurd, over-the-top situations (Experimental & Whimsical tone).

## 3. Core Loop
1.  **Deploy Squad**: Select up to 3 operators for a gauntlet mission (1–3 targets).
2.  **Run Mission**: Resolve sequential encounters (fail-fast logic).
3.  **Earn Rewards**: Collect Scrap (Scrap), XP, and rare genetics based on targets defeated.
4.  **Breed/Train**: Use rewards to breed new slimes, train stats, or buy equipment.
5.  **Repeat**: Face tougher missions and reach the ultimate win conditions.

## 4. Progression Systems
- **Early Game (G.1–G.3)**: Learn roster management, equipment basics (Hats), and low-tier missions.
- **Mid Game (G.4–G.8)**: Expand the roster, breed T1/T2 operators, and prepare for the **Boss Encounter**.
- **Endgame (G.9+)**: Infinite scaling, T3 legendary breeding, and regional leaderboards.

## 5. Win Conditions

### Mid-Game Victory: Boss Defeat
- **The Matriarch**: A massive Corrupted Slime King guarding the planet's core.
- Requires a squad of T2 operators with optimized equipment.
- Defeating the boss clears the primary "campaign" and unlocks **Endgame Scaling**.

### Endgame: Infinite Scaling
- Missions scale in difficulty indefinitely (Tier 10, Tier 100, etc.).
- Global leaderboards track the highest tier reached.
- Goal: Breed the perfect T3 operator with maxed stats and rare traits.

## 6. Mechanics

### Roster Management
- **Operators**: Biological slimes with stats (**STR, AGI, INT**), **Health**, and **Genetics**.
- **States**: `Idle`, `Deployed` (on mission), or `Injured` (in recovery).
- **Traits**: Permanent biological bonuses inherited through breeding.

### Mission System (Gauntlet)
- **Sequential Resolution**: Squads face targets one-by-one. Failure on any target stops progress.
- **Partial Success**: Players retain rewards and XP for all targets defeated before failure/retreat.
- **Injury Risk**: Cumulative "Nat 1" rolls on checks increase the risk of downtime injuries.

### Equipment & Customization
- **Slots**: Currently `Hat`. Future: `Chest`, `Hands`, `Feet`.
- **Bonuses**: Flat or percentage increases to stat checks or exploration success.

### Genetics & Breeding (G.6)
- **Trait Tiers**: T0 (Common) through T3 (Legendary).
- **Inheritance**: Offspring inherit dominant and recessive traits from parents.
- **Mutation**: Rare chance for a trait to jump a tier or mutate into a unique variant.

### Injury & Recovery
- **Downtime**: Injured operators are locked for a real-time recovery period.
- **Recovery Cost**: Can be accelerated with Scrap or specialized training.

## 7. Economy
- **Scrap (Scrap)**: Primary currency for items, breeding, and roster expansion.
- **XP**: Earned per mission target; spent on training TP (Training Points).
- **Sinks**: Breeding fees, high-tier gear, and recovery acceleration.

## 8. UI/UX Principles
- **Mobile-First (400×800)**: The primary target for all UI layouts.
- **44dp Touch Targets**: Minimum interactive size for buttons, tabs, and toggles.
- **Instant Response**: 5-second feedback loop for development ensures a snappy, responsive feel.
- **Bottom-Tab Navigation**: Always-on primary navigation for Roster, Contracts, Map, and Shop.

## 9. Narrative Tone
- **Tone**: Experimental, Whimsical, and tactical.
- **Humor**: High-stakes missions described with absurd, charming descriptions.
- **Example**: "Negotiate the liquidation of the Jiggly Collective." (Slimes are "just sleeping" on failure).

## 10. Inspirations & Precedents
- **Beastie Bay DX**: Sequential exploration tracks and party-based discovery.
- **RPG Core**: Genetic profile structures and trait inheritance logic.
- **Slime Garden**: Procedural dungeon tracks and progression visualization.

## 11. Success Metrics
- **Engagement**: Players reach the Boss within 2–4 weeks.
- **Variety**: Each playthrough features 15+ unique name/trait combinations.
- **Retention**: Post-boss player activity remains >20% via infinite scaling.
