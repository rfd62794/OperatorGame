# OperatorGame: Game Design Document

## 1. Vision Statement
OperatorGame is a tactical strategy game about breeding and deploying bio-engineered slimes for a corporate field audit. Players play as an astronaut crash-landed on a slime-infested world, forced to manage a high-stakes roster of "Operators" for an interstellar logistics company with terrible benefits. The ultimate goal is to breed a **Void Slime** capable of serving as an escape vessel to return home.

## 2. Core Pillars
- **Corporate Absurdism**: The player is a mid-level auditor in an impossible situation.
- **Strategy**: Squad composition and mission tactics (risk management) are the keys to survival.
- **Genetics**: Breeding and inheritance are the primary engines for reaching the endgame.
- **Risk**: Injuries and partial success create tension; every roll is a business expense.
- **Charm**: Whimsical slimes in cold, bureaucratic situations.

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

## 5. Win Condition: The Void Vessel

### The Ultimate Goal: Breeding the Void Slime
- The player’s only way off the planet is to breed an **Elder Void Slime**.
- This requires multiple generations of genetic refining to unlock the `Void` allele.
- **Mid-Game Milestone**: Identifying the first Void genetic marker in a wild node.
- **Victory**: Successfully incubating a Void Slime and launching the "Relocation Protocol."

### Endgame (Post-Victory)
- After the primary win condition, the player can continue to manage the facility for high-tier corporate "Efficiency Ratings."
- Infinite scaling exists as a way to test the limits of T3 genetic perfection, but the narrative focus remains on the escape.

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
- **Tone**: Corporate-Absurdist. The humor is dry, bureaucratic, and cynical.
- **Context**: The player is an employee of *AstroLogistics Corp*, performing a "Hazardous Environment Audit" while privately trying to find a way home.
- **Example**: "Audit the logistics of the Jiggly Collective." On failure: "Personnel underwent unscheduled dormancy; insurance claim denied."

## 10. Inspirations & Precedents
- **Beastie Bay DX**: Sequential exploration tracks and party-based discovery.
- **RPG Core**: Genetic profile structures and trait inheritance logic.
- **Slime Garden**: Procedural dungeon tracks and progression visualization.

## 11. Success Metrics
- **Engagement**: Players discover the Void genetic marker within 2 weeks.
- **Variety**: Each playthrough features 15+ unique name/trait combinations.
- **Victory**: At least 30% of players reach the "Relocation Protocol" (Void Escape).
