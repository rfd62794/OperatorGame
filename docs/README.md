# OPERATOR — Document Index

> **The two Bibles. If code and design conflict, this is the arbiter.**

---

## 🎮 Design Bible (GDD)

| Document | Purpose | Status |
|----------|---------|--------|
| [GDD.md](GDD.md) | Game feel, systems, tone, player experience | ✅ Current (v2.0) |

---

## ⚙️ Engineering Bible (SDD)

| Document | Purpose | Status |
|----------|---------|--------|
| [CONSTITUTION.md](CONSTITUTION.md) | Non-negotiable governing principles | ✅ Locked v1.0 |
| [SPEC.md](SPEC.md) | Domain entity contracts + formulas | ✅ Current (v2.0) |
| [sdd/PLAN.md](sdd/PLAN.md) | Module map, dependencies, test coverage | ✅ Current (v2.0) |

---

## 📋 Architectural Decision Records (ADRs)

| ADR | Decision | Status |
|-----|---------|--------|
| [ADR-001](adr/ADR-001-rust-stack.md) | Rust over Python/Go for core logic | ✅ Accepted |
| [ADR-002](adr/ADR-002-timestamp-over-countdown.md) | Timestamp-based offline timers | ✅ Accepted |
| [ADR-003](adr/ADR-003-atomic-save.md) | Atomic `.tmp` → rename save strategy | ✅ Accepted |
| [ADR-004](adr/ADR-004-success-formula.md) | Per-attribute scoring formula | ✅ Accepted |
| [ADR-005 *(pending)*](adr/) | Genetics hex-wheel culture system | 🔄 Sprint 1 |

---

## 🔬 Functional Blueprints

| Document | Purpose |
|----------|---------|
| [SLIME_RUST_BLUEPRINT.md](SLIME_RUST_BLUEPRINT.md) | rpgCore systemic audit → Rust transplant spec |

---

## Rule: Which Bible Wins?

```
Is it about how a player FEELS?  →  GDD.md
Is it about how a system WORKS?  →  SPEC.md
Is it about WHY we chose X?      →  ADR-00N
Is it about where code LIVES?    →  sdd/PLAN.md
```
