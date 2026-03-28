# ADR-037 — Three-Operator Squad Cap
> **Status:** Accepted | 2026-03-27

## Context
The success chance mathematics, mission difficulty requirements (SDD-035), and overall game balance were derived assuming a specific maximum number of operators per deployment. 

## Decision
A hard architectural constraint of **3 operators per deployment squad** is enforced at the data model and UI layers.

## Rationale
Restricting the squad size to 3 allows for a manageable complexity in "affinity matching" (matching operator cultures to mission requirements) while keeping the UI grid readable on mobile devices. All balance targets (e.g., Solo L1 = 75% success on Starter, Squad of 3 L6 = 60% success on Elite) are calibrated against this cap.

## Consequences
- **Positive:** Predictable balance math for future content tiers.
- **Positive:** Optimized UI layout for 360dp mobile screens.
- **Negative:** Any future increase to the squad cap (e.g., 4 or 5 operators) will require a full rebalance of all mission requirements and a new ADR.
