# ADR-003 — Persistence: Atomic Write via Temp-File Rename
> **Status:** Accepted | 2026-03-04

## Context
`save.json` is the single source of truth for the entire game state. A partial write — caused by a crash or forced kill mid-write — would produce a corrupt file and destroy the player's roster permanently.

## Decision
**Write to a `.tmp` file first, then rename atomically.**

```rust
// ✅ ACCEPTED
pub fn save(state: &GameState, path: &Path) -> Result<(), PersistenceError> {
    let tmp_path = path.with_extension("json.tmp");
    let serialised = serde_json::to_string_pretty(state)?;
    fs::write(&tmp_path, serialised)?;   // partial write risk on .tmp only
    fs::rename(&tmp_path, path)?;        // atomic on POSIX; near-atomic on NTFS
    Ok(())
}

// ❌ REJECTED — direct overwrite, corruption risk
fs::write("save.json", serialised)?;
```

## Rationale
On both POSIX (Linux/macOS) and NTFS (Windows), `rename()` over an existing file is atomic at the filesystem level — the old file is never visible in a half-written state. The worst case scenario is that `save.json.tmp` exists alongside a valid `save.json` on next launch, which is harmless.

## Consequences
- **Positive:** A crash between write and rename leaves the previous valid `save.json` intact.
- **Positive:** Players can recover from a corrupt `.tmp` manually without losing their roster.
- **Negative:** Adds one additional file operation per save cycle. Negligible at this scale.
- **Note:** `fs::rename` across filesystem boundaries (e.g., `/tmp` → `/home`) is fallible and will panic. The `.tmp` path is derived from the save path explicitly to prevent cross-device issues.
