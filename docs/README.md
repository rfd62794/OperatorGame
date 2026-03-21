# OperatorGame Documentation

Welcome to the OperatorGame design, architecture, and development documentation.

## Quick Links

- **[Deployment & Automation](../DEPLOYMENT.md)** — How to build and deploy to Android
- **[Constitution](../CONSTITUTION.md)** — Governance, ADR discipline, project structure
- **[Specification](../SPEC.md)** — Game mechanics, rules, balance

## Documentation Structure

### [Design](./design/)
High-level visual identity, blueprint, and design philosophy.
- `DESIGN_BLUEPRINT.md` — Aesthetic direction, color system, UI patterns
- `VISUAL_IDENTITY.md` — Branding, asset guidelines

### [Systems](./systems/)
Core game mechanics: genetics, stats, lifecycle, combat.
- `STAT_SYSTEM.md` — Character stat derivation and growth
- `LIFECYCLE_SDD.md` — Breeding, incubation, lifecycle phases
- Genetics engine, color mixing, stat calculations

### [Sprints & Phases](./sprints/)
Development history: phase directives, SDD specs, architectural decisions.
- `phase_e_core_loop.md` — Core loop implementation
- `phase_f_ui_polish.md` — Mobile UI refinement
- `android_tools_architecture.md` — PowerShell automation framework
- `crash_diagnosis_directive.md` — Debugging and telemetry
- (All sprint planning and architectural evolution)

### [Architecture Decision Records](./adr/)
Critical technical decisions and their rationale.

### [Roadmap](./roadmap/)
Future features, Sprint 2+, long-term vision.

## For New Contributors

1. Read `../CONSTITUTION.md` for project governance
2. Read `../SPECIFICATION.md` for game rules
3. Skim `./design/` to understand aesthetic direction
4. Skim `./systems/` to understand mechanics
5. Review relevant `./sprints/` phase for context on current work

## For Build/Deploy

See `../DEPLOYMENT.md` for Android automation framework.

---

*Last updated: March 2026*
