# SDD-004: Ripple World Map Topology

## 1. Overview
The "Ripple" World Map uses a radial graph design with 19 nodes distributed across concentric rings using polar coordinates. This ensures that the map is a resonant structure rather than an arbitrary scatter, placing the player's tactile anchor (The Elder's Meadow) at the literal core of the generation algorithm.

## 2. The Topological Spec (Rings & Distribution)

| Ring | Radius (r) | Count | DC Base | Culture Tier |
|---|----|----|----|---|
| 0 (Hub) | 0.0 | 1 | 0 | Elder (Void) |
| 1 (Inner) | 120.0 | 6 | 10 | Primary (Ember, Gale, Tide) |
| 2 (Middle) | 220.0 | 6 | 20 | Secondary (Marsh, Crystal, Tundra) |
| 3 (Outer) | 320.0 | 6 | 30 | Mixed |

### The Hub: Node 0
* **Name**: The Hidden Meadow
* **Role**: The Tactical Anchor. Contains the Elder.
* **Resonance Aura**: +2. This harmonic bonus propagates outward through adjacency.
* **Culture**: Void (neutral center)

### Ring 1: The Heartlands
* Composed of the Primary cultures: Ember (Red), Gale (Cyan), Tide (Blue).
* Contains firmly held nodes where Faction Influence begins at `0.85` or `85%`.

### Ring 2: The Wilds
* Composed of the Secondary cultures: Marsh (Green), Crystal (Violet), Tundra (Ice).
* Offset by `30°` (`30.0.to_radians()`) for a staggered "bloom" effect on the radar UI.

## 3. The "Hoot and Holler" Mechanic
The `startled_level` captures the accumulated noise of the Astronaut's actions in the Meadow. The map listens to interaction volume:
* **Dispatch Action**: +0.05
* **Incubate Action**: +0.10
* **Repair Action**: +0.25 (Metal grinding)

*Design Note: The Startled Level forces consequence. High resonance will eventually cause dynamic events or increased difficulty.*

## 4. UI Radar Rendering
* `scale` clamps at 320 radius.
* **Node rendering**: Uses exact `{Culture}::accent()` color from `genetics.rs`.
* **Pulsing**: Nodes actively contested (`influence < 0.3`) pulsate visually using `(time * 5.0).sin() * 2.0`.
