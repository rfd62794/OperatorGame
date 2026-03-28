# AUDIT_REPORT_SpecAlignment

**Date:** March 28, 2026  
**Status:** Audit Finalized  
**Directive:** Read-only health check of documentation vs. current implementation.

---

## §A — ADR Gap Check

| Decision Item | Status | Finding / ADR Reference |
|:--- |:--- |:--- |
| **ADR-001 through ADR-032** | **CURRENT** | All records present in `docs/adr/`. |
| **DC Formula Derivation** | **MISSING** | Formula `round(danger * 20).clamp(4, 20)` in SDD-035 is not formally ADR'd. |
| **Resource Economy Mapping** | **MISSING** | Mapping (Cash/Biomass/Scrap/Reagents) defined in SDD-036 lacks an ADR. |
| **Hat Inventory Model** | **MISSING** | Transferable model (§4.2 SDD-036) is not formally ADR'd. |
| **Static Node ID Assignments** | **MISSING** | Ring mapping (Ember=10, etc.) is implicit in code but lacks an ADR. |
| **3-Operator Squad Cap** | **MISSING** | Hard constraint used in balance math (§3 SDD-035) lacks an ADR. |
| **SidePanel Navigation** | **CURRENT** | Documented in `ADR-030-responsive-layout.md`. |

---

## §B — SDD Currency Check

| SDD File | Version | Status | Notes |
|:--- |:--- |:--- |:--- |
| `DIFFICULTY_BALANCE_SDD.md` | §1.0 | **STALE** | Drift: `radial.rs` uses `ring * 10 - offset` rather than the danger-scaled formula. |
| `SDD036_Equipment_Hat_System.md` | §1.0 | **CURRENT** | Acts as directive for G.3; `models.rs` currently contains placeholder `Gear`. |

---

## §C — SPEC.md Alignment

| Feature / Mechanic | Status | Notes |
|:--- |:--- |:--- |
| **§1.1 Operator Definition** | **STALE** | Describes "Human Mercenary" and "Jobs"; current code uses Genetics/Slimes. |
| **§1.6 GameState Schema** | **STALE** | References `roster` vs `slimes` distinction that has consolidated into `Operator` slimes. |
| **§2 Success Formula** | **STALE** | placeholder formula doesn't match the `D20` + `DC` + `StatMod` combat engine. |
| **§8 CLI Commands** | **STALE** | `operator hire` and `operator hatch` are inconsistent with current genetics logic. |

---

## §D — CONSTITUTION.md Check

| Logic / Rule | Status | Notes |
|:--- |:--- |:--- |
| **§2 Technology Choice** | **CURRENT** | `egui`, `serde`, `chrono` match current binary exactly. |
| **§3 Scope (IN)** | **STALE** | Lists "Operator roster (hire)" as MVP; current focus is "Genetic Hatching". |
| **Directory Structure** | **CURRENT** | `src/ui/`, `src/models.rs`, and `docs/adr/` follow the mandate. |

---

## §E — AGENT_CONTRACT.md Check

| Document | Status | Notes |
|:--- |:--- |:--- |
| **AGENT_CONTRACT.md** | **MISSING** | File not found in repository root or `docs/`. |

> [!NOTE]
> Module structure mentioned in directive (`contracts.rs`, `radar.rs`, `squad.rs`) **EXISTS** in `src/ui/`. `quartermaster.rs` is **PLANNED** for G.3.

---

## §F — docs/state/current.md

| Document | Status | Notes |
|:--- |:--- |:--- |
| **docs/state/current.md** | **MISSING** | File not found in repository. `docs/roadmap/` contains sprint history only. |

---

### Audit Summary
The documentation suite has significant **Drift** in the high-level specs (`SPEC.md`, `CONSTITUTION.md`) as the project transitioned from a Human-centric to a Slime-centric genetics model. While recent ADRs (030-032) and SDDs (035-036) are structurally sound, they have not yet been back-ported to the primary specification files.

*Audit performed by Antigravity | Sprint G.2 Final Closure*
