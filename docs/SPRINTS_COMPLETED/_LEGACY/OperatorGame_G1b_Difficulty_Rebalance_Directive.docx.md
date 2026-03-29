**OperatorGame**

Sprint G.1b — Difficulty Rebalance

*Authority: SDD-035 (DIFFICULTY\_BALANCE\_SDD.md)  |  Directive Type: IMPLEMENTATION*

*Pre-flight: run tests, report count, stop if any fail*

**⛔ STOP:** Run the full test suite before any changes. Report the passing count. Stop if anything fails.

**⚠** *This directive implements SDD-035 exactly. Do not deviate from the locked values in §5. If implementation reveals a math error, stop and report to architect — do not self-correct the SDD values.*

# **0\. Goal**

Apply the difficulty rebalance defined in SDD-035. Current mission parameters were derived from assumed stat values that do not match real operator output. A fresh Level 1 Hatchling has an effective stat of 2-3, not 15\. All DC values and stat requirements must be rederived from ground truth.

This is a values-only sprint. No new systems, no new UI, no architectural changes. Every change is a number replacement traceable to SDD-035 §5.

# **1\. Authority: SDD-035 Locked Values**

All implementation in this directive derives from the locked reference table in SDD-035 §5. Reproduced here for reference:

| Tier | DC Range | Primary Req | Level Gate | Target (2x L1) |
| :---- | :---- | :---- | :---- | :---- |
| Starter | 4 – 6 | 3 – 5 | L1 | \~90% |
| Standard | 6 – 8 | 8 – 12 | L1 | \~60% |
| Advanced | 10 – 14 | 15 – 20 | L3 | \~10% |
| Elite | 12 – 15 | 45 – 55 | L6 | \~60% |

Key changes from previous values:

* Standard level gate lowered from L3 to L1 — squad composition gates access, not level

* Standard DC lowered from 10-12 to 6-8 — two L1 Hatchlings can now attempt Standard at \~60%

* Elite DC corrected from 15-20 to 12-15 — L6 Prime squad hits target 60% success rate

* All Req values derived from real stat output (base 5-8 × 0.48 \= effective 2-4 per operator)

# **2\. Scope**

| File | Change | Task |
| :---- | :---- | :---- |
| src/world\_map.rs | Update DC and Req values for all 14 missions to match SDD-035 §5 | A |
| src/models.rs | Update MissionTier DC ranges and min\_roster\_level for Standard tier | A |
| src/ui/contracts.rs | Verify success chance display reflects updated values — no logic change expected | B |
| tests/g1\_stability.rs | Update test assertions to match new DC/Req values | C |
| docs/sdd/DIFFICULTY\_BALANCE\_SDD.md | Read only — this is the authority document, do not modify | — |

**⚠** *Do not modify persistence.rs, ops.rs, radar.rs, manifest.rs, or any file not listed above. This sprint touches only values, not logic.*

# **3\. Tasks**

## **Task A — Update Mission Values in world\_map.rs and models.rs**

File: src/world\_map.rs

For each of the 14 static missions, update base\_dc and primary\_req to fall within the SDD-035 §5 ranges for their tier. Distribute values across the range — do not set every mission in a tier to identical numbers. Variation within the range creates mission diversity.

Example distribution guidance:

* Starter missions: DC 4, 5, 5, 6 — Req 3, 4, 4, 5

* Standard missions: DC 6, 7, 7, 8 — Req 8, 9, 11, 12

* Advanced missions: DC 10, 11, 13, 14 — Req 15, 16, 18, 20

* Elite missions: DC 12, 14 — Req 45, 55

File: src/models.rs — MissionTier enum or associated constants

Update Standard tier min\_roster\_level from 3 to 1\. Confirm Advanced remains L3 and Elite remains L6.

**⚠** *Do not change mission names, rewards, narrative text, or any field other than base\_dc, primary\_req, and min\_roster\_level. One field at a time, cross-reference SDD-035 after each tier is done.*

## **Task B — Verify Success Display**

File: src/ui/contracts.rs

No logic changes expected. After updating world\_map.rs values, confirm the success chance display recalculates correctly for the new DCs. Specifically verify:

* A staged squad of two L1 operators against a Standard mission (DC 6-8) shows RISKY or GOOD ODDS — not DESPERATE or GUARANTEED

* An unstaged mission still shows UNSTAFFED

* Elite missions show DANGEROUS or DESPERATE for a L1 squad — correctly discouraging early attempts

If the display is wrong, the issue is in calculate\_success\_chance() — report to architect before changing anything. Do not self-patch the formula.

## **Task C — Update Test Assertions**

File: tests/g1\_stability.rs

Several existing test anchors assert specific DC or Req values. Update assertions to match SDD-035 §5 locked values. Do not remove any tests — only update expected values.

Specifically:

* G1.4 (tier DC ranges) — update expected DC ranges to 4-6, 6-8, 10-14, 12-15

* G1.5 (success chance math) — update any hardcoded DC inputs in success chance tests to use new Standard DC of 6-8

* G1.7 (apex difficulty) — Elite DC is now 12-15, not 20\. Update assertion

**⚠** *All 212 prior tests must still pass after this sprint. Zero regressions. Report pre-sprint and post-sprint test counts explicitly.*

# **4\. Verification**

### **Automated**

* Pre-sprint test count reported

* Post-sprint test count matches or exceeds pre-sprint count

* Zero failing tests

* G1.4, G1.5, G1.7 assertions updated and passing with new values

### **Manual — build and deploy to Moto G after Task A**

1. Open Quest Board — confirm Starter and Standard missions visible from Level 1

2. Stage one L1 operator — Standard mission should show RISKY or DANGEROUS (\~35-45%)

3. Stage two L1 operators — Standard mission should show RISKY or GOOD ODDS (\~55-65%)

4. Attempt a Standard mission with two L1 operators — outcome should not feel like a wall

5. Confirm Elite missions show low odds for a L1 squad — DESPERATE is correct

6. Complete two missions — confirm level-up fires and stat values change on card

# **5\. Completion Checklist**

* Pre-sprint test count reported

* All 14 missions updated with SDD-035 §5 values — DC and Req within spec ranges

* Standard tier level gate set to L1 in models.rs

* Success display shows RISKY/GOOD ODDS for two L1 operators on Standard mission

* Elite missions show DESPERATE for L1 squad

* All prior tests passing — zero regressions

* Build deployed to Moto G — manual verification steps completed

* No files modified outside approved scope

# **6\. Notes for Agent**

**⚠** *This sprint is values-only. If you find yourself writing new logic, new functions, or new UI — stop. That is out of scope. Report what you found and wait for architect direction.*

The authority for every number in this sprint is SDD-035. If a value you want to use is not in SDD-035, it does not go in this sprint.

The success chance formula in calculate\_success\_chance() is not being changed. Only the inputs (DC, Req) are changing. If the formula produces unexpected outputs with new inputs, report it — do not patch the formula.

Distribute DC and Req values naturally within each tier range. Missions should feel varied, not identical clones of the same difficulty. The ranges in SDD-035 §5 are bounds, not single values.

*RFD IT Services Ltd.  |  OperatorGame  |  Sprint G.1b  |  March 2026*
