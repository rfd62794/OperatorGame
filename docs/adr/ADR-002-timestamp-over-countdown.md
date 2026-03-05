# ADR-002 — Mission Timers: Timestamp-over-Countdown
> **Status:** Accepted | 2026-03-04

## Context
The MVP requires missions that take 30–300 seconds. The naive implementation is a decrementing counter in memory (`time_remaining -= delta`). This requires the application to stay running.

The product's stated design goal — a "Mafia Wars refresh" feel — explicitly assumes the user closes the app between sessions.

## Decision
**Store the absolute UTC completion timestamp** (`completes_at: DateTime<Utc>`) instead of a remaining-seconds counter.

```rust
// ✅ ACCEPTED — wall-clock comparison
pub fn is_complete(&self) -> bool {
    Utc::now() >= self.completes_at
}

// ❌ REJECTED — requires active process
let mut time_remaining: u64 = mission.duration_secs;
// (decremented on every tick)
```

## Rationale
- A timestamp survives serialisation to `save.json` and is restored exactly on reload.
- Progress percentage for the GUI is trivially derived: `elapsed / total_duration`.
- No background thread, no tick loop, and no IPC needed at Tier 1.
- The "offline" window (app closed) is accounted for automatically — richer gameplay without richer code.

## Consequences
- **Positive:** Missions progress while the app is closed. Reopening the app and running `aar` immediately resolves any completed missions.
- **Positive:** GUI progress bars derive from `(Utc::now() - started_at) / duration_secs`. Pure arithmetic.
- **Negative:** System clock rollback can make missions appear incomplete longer than expected.
- **Accepted risk:** For single-player offline play, clock manipulation is an accepted edge case.
- **Future:** If multiplayer is added, `completes_at` must be set by the server and treated as immutable by the client.
