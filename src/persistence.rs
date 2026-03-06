/// persistence.rs — GameState serialisation via serde_json.
///
/// Contract:
/// - `load()` returns a fresh GameState if `save.json` is absent.
/// - `save()` writes atomically (temp file → rename) to prevent corruption
///   if the process is killed mid-write.
/// - All errors are surfaced as `PersistenceError` — no silent failures.
use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::genetics::{GeneticTier, SlimeGenome};
use crate::inventory::Inventory;
use crate::models::{AarOutcome, Deployment, Gear, Mission, SlimeState};
use crate::world_map::WorldMap;

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
            PersistenceError::Io(e)   => write!(f, "I/O error: {e}"),
            PersistenceError::Json(e) => write!(f, "JSON error: {e}"),
        }
    }
}

impl From<std::io::Error> for PersistenceError {
    fn from(e: std::io::Error) -> Self { PersistenceError::Io(e) }
}
impl From<serde_json::Error> for PersistenceError {
    fn from(e: serde_json::Error) -> Self { PersistenceError::Json(e) }
}

/// A genome currently being synthesised in the Bio-Incubator (ADR-010).
/// Collected by `operator incubate` once `completes_at` has passed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncubatingGenome {
    /// The genome-in-progress.
    pub genome:       SlimeGenome,
    /// UTC timestamp when synthesis completes and the slime can be collected.
    pub completes_at: DateTime<Utc>,
}

impl IncubatingGenome {
    /// Create a new incubation entry. Duration follows ADR-010 tier table:
    /// Blooded/Bordered: 900s | Sundered/Drifted: 1200s | Threaded: 1500s
    /// Convergent: 1800s | Liminal: 2100s | Void: 2400s
    pub fn new(genome: SlimeGenome) -> Self {
        let base_secs = 900i64;
        let tier_bonus = match genome.genetic_tier() {
            GeneticTier::Blooded | GeneticTier::Bordered => 0,
            GeneticTier::Sundered | GeneticTier::Drifted => 300,
            GeneticTier::Threaded                        => 600,
            GeneticTier::Convergent                      => 900,
            GeneticTier::Liminal                         => 1200,
            GeneticTier::Void                            => 1500,
        };
        let duration_secs = base_secs + tier_bonus;
        Self {
            genome,
            completes_at: Utc::now() + chrono::Duration::seconds(duration_secs),
        }
    }

    /// True if incubation has finished and the slime is ready to collect.
    pub fn is_ready(&self) -> bool {
        Utc::now() >= self.completes_at
    }

    /// Remaining seconds of incubation, or 0 if complete.
    pub fn remaining_secs(&self) -> i64 {
        let rem = (self.completes_at - Utc::now()).num_seconds();
        rem.max(0)
    }
}

// ---------------------------------------------------------------------------
// GameState — the single source of truth serialised to disk
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GameState {
    /// Player currency.
    pub bank: u64,
    /// Active or unresolved deployments.
    pub deployments: Vec<Deployment>,
    /// Static mission pool. Populated from seed data if absent.
    pub missions: Vec<Mission>,
    /// Slime stable — persists across sessions (fixes the Python persistence gap).
    #[serde(default)]
    pub slimes: Vec<SlimeGenome>,
    /// Genomes currently incubating in the Bio-Incubator (ADR-010).
    #[serde(default)]
    pub incubating: Vec<IncubatingGenome>,
    /// Crashed Ship repair tier (0–8). Gates demo unlock (ADR-013).
    /// 0 = emergency power only; 8 = ship fully restored (endgame).
    #[serde(default)]
    pub tech_tier: u8,
    /// The living planet map — 15 nodes, faction influence, expedition sites.
    /// Populated with a fresh fixed-seed map when absent from save (ADR-014).
    #[serde(default)]
    pub world_map: WorldMap,
    /// Cross-session Cargo Bay (Biomass, Scrap, Reagents). ADR-030.
    #[serde(default)]
    pub inventory: Inventory,
}

impl GameState {
    pub fn new_with_seed_missions() -> Self {
        Self {
            missions: crate::models::seed_missions(),
            bank: 100,
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
    #[cfg(target_os = "android")]
    {
        default_save_path_on_android()
    }
    #[cfg(not(target_os = "android"))]
    {
        let mut path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        path.pop(); // Remove the executable name from the path
        path.push(SAVE_FILE);
        path
    }
}

pub fn default_save_path_on_android() -> PathBuf {
    // ADR-042: Hard-path the Android internal data directory.
    // In a production JNI environment, we would query Context.getFilesDir().
    // For the Local Forge, we target the standard internal storage root.
    PathBuf::from("/data/data/com.robertdugger.operator/files/save.json")
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
        // assert!(state.roster.is_empty()); // Removed as per instruction
        assert_eq!(state.bank, 100); // New default bank is 100
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
