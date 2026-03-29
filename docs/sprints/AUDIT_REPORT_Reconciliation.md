# Audit Report: Reconciliation (Read-Only)

**Date**: 2026-03-28  
**Status**: INTERNAL AUDIT  
**Agent**: Antigravity  

## Executive Summary

This report identifies significant divergences between the current codebase state (Sprints G.3–G.5 history) and the project's "locked" design authority (**AGENT_CONTRACT**, **ADR-034**, **SDD-036**). The most critical findings are the unapproved re-architecture of the `models` layer and the unauthorized renaming of the primary equipment currency.

---

## 🏗️ Structural Divergences

### 1. Models Layer (God Module Refactor)
- **Constraint (AGENT_CONTRACT)**: `src/models.rs` is the core data struct file.
- **Current State**: `src/models/` is a directory containing `item.rs`, `mission.rs`, `operator.rs`, `expedition.rs`, `log.rs`, and `mod.rs`.
- **Impact**: Violates the structural contract and likely complicates JNI/Android builds if not carefully handled.
- **Status**: **NON-COMPLIANT**

### 2. File Organization
- **Current State**: `src/world_map/` is a directory. `AGENT_CONTRACT` expects `src/world_map.rs`.
- **Status**: **NON-COMPLIANT**

---

## 💰 Taxonomic Divergences (Currency)

### 1. MTL vs Scrap
- **Decision (ADR-034)**: Equipment currency is strictly **Scrap**.
- **Current State**: The UI (`contracts.rs`, `quartermaster.rs`) and model logic (`item.rs`) refer to this resource as **MTL**.
- **Impact**: Breaks alignment with project-wide terminology and ADR-034's strict partitioning.
- **Status**: **NON-COMPLIANT**

---

## 🎮 Feature & Design Divergences

### 1. G.3 Equipment (Hats)
- **Specification (SDD-036)**: 4 hats (Scout Hood, Knight Helm, Mage Hood, Commander Cap) with specific Scrap costs (50, 100, 100, 250).
- **Current State**: Hats are implemented, but they use `MTL` and the naming/costs may have diverged during the "Agent version" of G.3. 
- **Status**: **REVIEW REQUIRED** (Needs code verification against exact costs/stats).

### 2. G.5 Gauntlet (Multi-Target Missions)
- **Status**: **UNAUTHORIZED SCOPE**
- **Analysis**: The `Mission` struct now uses a `Vec<Target>` sequential resolution model. While mechanically sound, this was not in the roadmap and appeared without a directive or SDD. It broke several G.2-era integration tests.
- **Status**: **PENDING REVISION/ROLLBACK**

---

## 🧪 Test Floor Status

- **Requirement (AGENT_CONTRACT)**: 219 passing, 0 failing, 0 skipped.
- **Current State**: `cargo test` returns **failures** in `ui::f1b_loop_tests` and `combat::tests`.
- **Primary Failure**: `index out of bounds` in deployment resolution tests, likely due to the `Vec<Target>` transition.
- **Status**: **UNSTABLE** (Test floor is broken).

---

## 📋 Recommendations

1. **Re-centralize Models**: Collapse `src/models/` back into `src/models.rs` unless an ADR is drafted for modularization.
2. **Restore Currency (MTL -> Scrap)**: Mass-rename all "MTL" occurrences back to "Scrap" to honor ADR-034.
3. **Re-spec the Gauntlet**: Either rollback G.5 or draft a retroactive SDD that incorporates the Designer's feedback on mission-phasing.
4. **Fix the Test Floor**: Prioritize fixing `f1b_loop_tests` to restore the 219 passing baseline before any G.6 work begins.

---
*End of Report*
