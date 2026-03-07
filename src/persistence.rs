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
use crate::models::{Deployment, Expedition, Mission};
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

/// Current save format version. Increment with every breaking schema change.
/// v6 (Sprint 7): Deployment.is_emergency
pub const SAVE_VERSION: u32 = 6;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    /// Player currency. Can be negative (Debt).
    pub bank: i64,
    /// Active or unresolved deployments.
    pub deployments: Vec<Deployment>,
    /// Static mission pool. Populated from seed data if absent.
    pub missions: Vec<Mission>,
    /// Slime stable — persists across sessions.
    #[serde(default)]
    pub slimes: Vec<SlimeGenome>,
    /// Genomes currently incubating.
    #[serde(default)]
    pub incubating: Vec<IncubatingGenome>,
    /// Crashed Ship repair tier (0–8).
    #[serde(default)]
    pub tech_tier: u32,
    /// Lens unlocked flag for Bio-Manifest recessive alleles.
    #[serde(default)]
    pub lens_unlocked: bool,
    /// World map node wars (ADR-014).
    #[serde(default)]
    pub world_map: WorldMap,
    /// Last time daily upkeep was deducted.
    #[serde(default = "Utc::now")]
    pub last_upkeep_at: DateTime<Utc>,
    /// Cross-session Cargo Bay (Biomass, Scrap, Reagents). ADR-030.
    #[serde(default)]
    pub inventory: Inventory,
    /// Active or resolved island expeditions (Sprint 3). ADR-002 wall-clock.
    #[serde(default)]
    pub active_expeditions: Vec<Expedition>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            bank: 500,
            deployments: Vec::new(),
            missions: Vec::new(),
            slimes: Vec::new(),
            incubating: Vec::new(),
            tech_tier: 0,
            lens_unlocked: false,
            world_map: WorldMap::default(),
            last_upkeep_at: Utc::now(),
            inventory: Inventory::default(),
            active_expeditions: Vec::new(),
        }
    }
}

/// Constant for daily upkeep per idle operator.
pub const UPKEEP_PER_DAY: i64 = 50;

impl GameState {
    pub fn new_with_seed_missions() -> Self {
        Self {
            missions: crate::models::seed_missions(),
            bank: 100,
            ..Default::default()
        }
    }

    /// Sprint 7B: Maintenance Pressure
    /// Deducts $50 per idle operator per day.
    /// Returns (cost_deducted, idle_count).
    pub fn apply_daily_upkeep(&mut self, now: DateTime<Utc>) -> (i64, i64) {
        let elapsed = now - self.last_upkeep_at;
        let days = elapsed.num_seconds() as f64 / 86400.0;
        
        if days < 0.001 { // Tick every ~1.5 minutes or so
            return (0, 0);
        }

        // Only IDLE operators cost maintenance.
        let idle_count = self.slimes.iter()
            .filter(|s| matches!(s.state, crate::models::SlimeState::Idle))
            .count() as i64;

        let cost = (days * idle_count as f64 * UPKEEP_PER_DAY as f64) as i64;
        let floor = -(UPKEEP_PER_DAY * 3);
        
        if cost > 0 && self.bank > floor {
            self.bank = (self.bank - cost).max(floor);
            self.last_upkeep_at = now;
        }

        (cost, idle_count)
    }
}

// ---------------------------------------------------------------------------
// I/O helpers
// ---------------------------------------------------------------------------

#[allow(dead_code)]
const SAVE_FILE: &str = "save.json";

/// Returns the canonical path to the save file.
pub fn save_path() -> PathBuf {
    #[cfg(target_os = "android")]
    {
        // On Android, the path must be provided by android_activity at runtime.
        // This is now handled in android_main and passed to OperatorApp.
        PathBuf::from("save.json") 
    }
    #[cfg(not(target_os = "android"))]
    {
        let mut path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        path.pop(); // Remove the executable name from the path
        path.push(SAVE_FILE);
        path
    }
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
    // v3 → v4 migration: rename culture_expr → culture_alleles on each slime
    let raw = migrate_v3_to_v4(&raw);
    let state: GameState = serde_json::from_str(&raw)?;
    Ok(state)
}

/// Migrate a raw save JSON string from v3 to v4 schema.
/// Renames `culture_expr` to `culture_alleles.dominant` on every slime,
/// zero-padding the `recessive` field. No-op on already-migrated saves.
fn migrate_v3_to_v4(raw: &str) -> String {
    // Fast path: if the save already uses culture_alleles, skip.
    if !raw.contains("\"culture_expr\"") {
        return raw.to_string();
    }
    // Parse as generic JSON and patch each slime entry
    let Ok(mut root) = serde_json::from_str::<serde_json::Value>(raw) else {
        return raw.to_string(); // Let load() surface the error
    };
    let zero_recessive = serde_json::json!([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    // Patch slimes array
    for arr_key in &["slimes", "incubating"] {
        if let Some(arr) = root.get_mut(*arr_key).and_then(|v| v.as_array_mut()) {
            for entry in arr.iter_mut() {
                let genome = if *arr_key == "incubating" {
                    entry.get_mut("genome")
                } else {
                    Some(entry)
                };
                if let Some(g) = genome {
                    if let Some(expr) = g.as_object_mut().and_then(|m| m.remove("culture_expr")) {
                        g["culture_alleles"] = serde_json::json!({
                            "dominant":  expr,
                            "recessive": zero_recessive.clone(),
                        });
                    }
                }
            }
        }
    }
    serde_json::to_string(&root).unwrap_or_else(|_| raw.to_string())
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
        assert_eq!(state.bank, 100); // new_with_seed_missions sets this to 100
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

    #[test]
    fn test_apply_daily_upkeep_debt_floor() {
        let mut state = GameState::default();
        state.bank = -140;
        state.last_upkeep_at = Utc::now() - chrono::Duration::days(1);
        
        let mut rng = rand::thread_rng();
        let genome = crate::genetics::generate_random(crate::genetics::Culture::Ember, "T", &mut rng);
        state.slimes.push(genome); // 1 idle operator -> $50 upkeep
        
        // We only care about cost and total_ops
        let (cost, idle_count) = state.apply_daily_upkeep(Utc::now());
        assert_eq!(idle_count, 1);
        assert_eq!(cost, 10); // Only deducts $10 to hit floor of -150
        assert_eq!(state.bank, -150);
        
        // Second run should deduct nothing
        state.last_upkeep_at = Utc::now() - chrono::Duration::days(1);
        let (cost2, _) = state.apply_daily_upkeep(Utc::now());
        assert_eq!(cost2, 0);
        assert_eq!(state.bank, -150);
    }
}
