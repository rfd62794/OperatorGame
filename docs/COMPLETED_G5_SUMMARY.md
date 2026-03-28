# Sprint G.5: Multi-Target Missions & Reliability [COMPLETE]

## Objectives
1.  **Transition to Gauntlet Model**: ✅ Successfully implemented sequential target resolution.
2.  **Fix "Stuck Deployed" Bug**: ✅ Moved resolution logic to `GameState::resolve_deployment` for atomic state management.
3.  **UI Feedback**: ✅ Improved Contracts and AAR panels with target lists and progress tracking.

## G.5 Mission Design (Gauntlet)
### Sequential Encounter Logic
- Missions will transition from flat requirements to a `Vec<Target>`.
- `Deployment::resolve` iterates and rolls for each target.
- Failure on Target 1 = mission failure.
- Success on Target 1 -> Move to Target 2.
- Rewards scale linearly with targets defeated.

### UI Implementation (400x800)
- **Target List**: Contracts view will list combatants (e.g., "3x Orbital Sentry").
- **AAR Progress**: Multi-phase narrative in the AAR summary.

## Proposed Bug Fix (Reliability)
- Move state reset code out of `apply_aar_outcome` (dismissal) and into `resolve_deployment` (resolution).
- Ensure the state is updated in memory and persisted immediately.

## Design Questions
- **One Roll vs Phase Rolls?** I suggest sequential phase rolls (dramatic tension).
- **Deterministic Stories?** For now, random but capped at 50, but sequential logic makes narrative-merging a priority.

## Verification
- `cargo test --lib models::mission`
- `.\run_mobile.ps1` (Verify loop speed <30s)
