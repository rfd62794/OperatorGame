# GDD-007: Command Glossary (Cheatsheet)

> **"A precise command is a stable resonance."**

This glossary lists all available CLI commands for the Astronaut's terminal. Use these to manage your roster and missions when the GUI is offline or for high-speed batch operations.

## 1. Tactical Commands
*   **`operator missions`**: List all currently available missions on the Ripple map.
*   **`operator deploy <mission_id> <squad_ids...>`**: Dispatches a squad to a mission. 
    *   *Note: You can use short-ID prefixes (e.g., `operator deploy 8f a1 b2`).*
*   **`operator aar`**: Resolves completed missions. Generates After-Action Reports and awards resources (Gel/Scrap/Bank).
*   **`operator status`**: Displays the global Bank total, Roster size, and active Deployment ETA.

## 2. Biological Commands (Genetics)
*   **`operator slimes`**: List all slimes in the Bio-Manifest with their full genetic profiles and IDs.
*   **`operator hatch <name> <culture>`**: Hatches a new slime genome (requires Biomass).
*   **`operator splice <parent_a> <parent_b> <child_name>`**: Executes a Splicing event between two parents. (Requires the Bio-Incubator Tech).
*   **`operator incubate`**: Periodic command to harvest slimes that have finished their incubation period.

## 3. System Commands
*   **`operator gui`**: Launches the primary eGUI War Room interface (Default).
*   **`operator help`**: Displays the standard CLI help documentation.

## 4. Interaction Summary
| Action | Command | Resource Used |
|---|---|---|
| Deploy | `deploy` | Startled Level (+5%) |
| Hatch | `hatch` | Bank ($25) |
| Harvest | `incubate` | Startled Level (+10%) |
| Resolve | `aar` | None |
