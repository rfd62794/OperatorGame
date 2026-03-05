/// persistence.rs — GameState serialisation via serde_json.
///
/// Contract:
/// - `load()` returns a fresh GameState if `save.json` is absent.
/// - `save()` writes atomically (temp file → rename) to prevent corruption
///   if the process is killed mid-write.
/// - All errors are surfaced as `PersistenceError` — no silent failures.
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::models::{Deployment, Mission, Operator};

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum PersistenceError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersistenceError::Io(e) => write!(f, "I/O error: {e}"),
            PersistenceError::Json(e) => write!(f, "JSON error: {e}"),
        }
    }
}

impl From<std::io::Error> for PersistenceError {
    fn from(e: std::io::Error) -> Self {
        PersistenceError::Io(e)
    }
}

impl From<serde_json::Error> for PersistenceError {
    fn from(e: serde_json::Error) -> Self {
        PersistenceError::Json(e)
    }
}

// ---------------------------------------------------------------------------
// GameState — the single source of truth serialised to disk
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GameState {
    /// Every operator the player has ever hired (alive ones only).
    pub roster: Vec<Operator>,
    /// Player currency.
    pub bank: u64,
    /// Active or unresolved deployments.
    pub deployments: Vec<Deployment>,
    /// Static mission pool. Populated from seed data if absent.
    pub missions: Vec<Mission>,
}

impl GameState {
    pub fn new_with_seed_missions() -> Self {
        Self {
            missions: crate::models::seed_missions(),
            ..Default::default()
        }
    }
}

// ---------------------------------------------------------------------------
// I/O helpers
// ---------------------------------------------------------------------------

const SAVE_FILE: &str = "save.json";

/// Returns the canonical path to the save file.
pub fn save_path() -> PathBuf {
    PathBuf::from(SAVE_FILE)
}

/// Load GameState from disk.
///
/// - If `save.json` does not exist → return a fresh `GameState` with seed missions.
/// - If the file exists but is corrupt → surface the error (do NOT silently overwrite).
pub fn load(path: &Path) -> Result<GameState, PersistenceError> {
    if !path.exists() {
        return Ok(GameState::new_with_seed_missions());
    }

    let raw = fs::read_to_string(path)?;
    let state: GameState = serde_json::from_str(&raw)?;
    Ok(state)
}

/// Save GameState to disk using an atomic write:
///   1. Write to `<path>.tmp`
///   2. Rename to `<path>`
///
/// This prevents a partially-written file from corrupting progress if the
/// process is killed between writes.
pub fn save(state: &GameState, path: &Path) -> Result<(), PersistenceError> {
    let tmp_path = path.with_extension("json.tmp");
    let serialised = serde_json::to_string_pretty(state)?;
    fs::write(&tmp_path, serialised)?;
    fs::rename(&tmp_path, path)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn temp_path() -> PathBuf {
        let mut p = env::temp_dir();
        p.push(format!("operator_test_{}.json", uuid::Uuid::new_v4()));
        p
    }

    #[test]
    fn test_load_returns_fresh_when_absent() {
        let p = temp_path();
        // File does not exist → should get default state
        let state = load(&p).expect("load should succeed for absent file");
        assert!(state.roster.is_empty());
        assert_eq!(state.bank, 0);
        assert!(!state.missions.is_empty(), "Seed missions should be populated");
    }

    #[test]
    fn test_save_then_load_roundtrip() {
        let p = temp_path();
        let mut state = GameState::new_with_seed_missions();
        state.bank = 9001;

        save(&state, &p).expect("save should succeed");
        let loaded = load(&p).expect("load should succeed after save");

        assert_eq!(loaded.bank, 9001);
        assert_eq!(loaded.missions.len(), state.missions.len());

        // Clean up
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn test_corrupt_file_surfaces_error() {
        let p = temp_path();
        std::fs::write(&p, b"{{not valid json{{").expect("write fixture");
        let result = load(&p);
        assert!(
            matches!(result, Err(PersistenceError::Json(_))),
            "Corrupt file should return Json error"
        );
        let _ = std::fs::remove_file(&p);
    }
}
