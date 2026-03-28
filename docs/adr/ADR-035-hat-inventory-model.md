# ADR-035 — Hat Inventory & Replacement Model
> **Status:** Accepted | 2026-03-27

## Context
When an operator is already wearing a hat (Equipment) and the player assigns a new one, we need to decide whether the old item is destroyed (sunk cost) or returned to the inventory.

## Decision
Old hats are **never destroyed** on replacement. They are returned to the `hat_inventory` on the `GameState` record for future use on other operators.

## Rationale
Player agency and tactical experimentation. Destroying hats creates a "friction points" where a player might be afraid to equip a specialized hat (e.g., +2 INT) if they know they'll eventually want a +3 in that slot and don't want to "lose" the old one. Reusable gear allows for "loadout" based gameplay.

## Consequences
- **Positive:** Encourages experimentation and specialized gear builds.
- **Positive:** Players can build and maintain a complete "wardrobe" of hats over time.
- **Negative:** No "loot sink" for excess low-tier equipment beyond hoarding or future selling.
