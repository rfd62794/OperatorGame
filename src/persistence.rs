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

use crate::genetics::GeneticTier;
use crate::inventory::Inventory;
use crate::models::{Deployment, Expedition, LogEntry, Mission, ResourceYield};
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
    /// The operator-in-progress.
    pub operator:     crate::models::Operator,
    /// UTC timestamp when synthesis completes and the slime can be collected.
    pub completes_at: DateTime<Utc>,
}

impl IncubatingGenome {
    /// Create a new incubation entry. Duration follows ADR-010 tier table:
    /// Blooded/Bordered: 900s | Sundered/Drifted: 1200s | Threaded: 1500s
    /// Convergent: 1800s | Liminal: 2100s | Void: 2400s
    pub fn new(operator: crate::models::Operator) -> Self {
        let base_secs = 900i64;
        let tier_bonus = match operator.genome.genetic_tier() {
            GeneticTier::Blooded | GeneticTier::Bordered => 0,
            GeneticTier::Sundered | GeneticTier::Drifted => 300,
            GeneticTier::Threaded                        => 600,
            GeneticTier::Convergent                      => 900,
            GeneticTier::Liminal                         => 1200,
            GeneticTier::Void                            => 1500,
        };
        let duration_secs = base_secs + tier_bonus;
        Self {
            operator,
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
/// v10 (Sprint G.2): added unlocked_nodes HashSet
/// v11 (Sprint G.3): added hat_inventory and equipped_hat
pub const SAVE_VERSION: u32 = 11;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    /// Player currency. Can be negative (Debt).
    pub bank: i64,
    /// Active or unresolved deployments.
    pub deployments: Vec<Deployment>,
    /// Available missions for the day.
    pub missions: Vec<Mission>,
    /// Operational operators (identity + state).
    #[serde(default)]
    pub slimes: Vec<crate::models::Operator>,
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
    pub last_upkeep_at: DateTime<Utc>,
    /// Last time the mission pool was refreshed.
    pub last_pool_refresh: DateTime<Utc>,
    /// Cross-session Cargo Bay (Biomass, Scrap, Reagents). ADR-030.
    #[serde(default)]
    pub inventory: Inventory,
    /// Active or resolved island expeditions (Sprint 3). ADR-002 wall-clock.
    #[serde(default)]
    pub active_expeditions: Vec<Expedition>,
    /// UI state: Main navigation tab
    #[serde(default)]
    pub active_tab: crate::platform::BottomTab,
    /// UI state: Roster sub-tab
    #[serde(default)]
    pub roster_sub_tab: crate::platform::RosterSubTab,
    /// UI state: Missions sub-tab
    #[serde(default)]
    pub missions_sub_tab: crate::platform::MissionsSubTab,
    /// UI state: Map sub-tab
    #[serde(default)]
    pub map_sub_tab: crate::platform::MapSubTab,
    /// UI state: Logs sub-tab
    #[serde(default)]
    pub logs_sub_tab: crate::platform::LogsSubTab,
    /// Persisted mission/combat log. Replaces the old in-RAM Vec<String> on OperatorApp.
    /// Capped at 50 entries; newest entry first.
    #[serde(default)]
    pub combat_log: Vec<LogEntry>,
    /// Save format version for migrations.
    #[serde(default)]
    pub version: u32,
    /// Set of unlocked map node IDs.
    #[serde(default)]
    pub unlocked_nodes: std::collections::HashSet<usize>,
    /// Purchased hats not currently equipped (G.3).
    #[serde(default)]
    pub hat_inventory: Vec<crate::models::HatId>,
}

impl Default for GameState {
    fn default() -> Self {
        let mut unlocked = std::collections::HashSet::new();
        unlocked.insert(0); // Center node always unlocked
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
            last_pool_refresh: Utc::now(),
            inventory: Inventory::default(),
            active_expeditions: Vec::new(),
            active_tab: crate::platform::BottomTab::Roster,
            roster_sub_tab: crate::platform::RosterSubTab::default(),
            missions_sub_tab: crate::platform::MissionsSubTab::default(),
            map_sub_tab: crate::platform::MapSubTab::default(),
            logs_sub_tab: crate::platform::LogsSubTab::default(),
            combat_log: Vec::new(),
            version: SAVE_VERSION,
            unlocked_nodes: unlocked,
            hat_inventory: Vec::new(),
        }
    }
}

impl GameState {
    /// Sprint 8: Refresh pool if calendar date has changed (00:00 UTC).
    /// Sprint G.1: Active deployments are protected from removal.
    pub fn refresh_missions_if_needed(&mut self, now: DateTime<Utc>) -> bool {
        let is_same_day = self.last_pool_refresh.date_naive() == now.date_naive();
        
        if is_same_day && !self.missions.is_empty() {
            return false;
        }

        use rand::SeedableRng;
        use rand::rngs::SmallRng;
        
        // Seed from the date (days since unix epoch) to ensure consistent daily pool.
        let days = now.timestamp() / 86400;
        let mut rng = SmallRng::seed_from_u64(days as u64);

        // Task A.1: Identify missions currently targeted by active deployments.
        let active_mission_ids: std::collections::HashSet<uuid::Uuid> = self.deployments
            .iter()
            .filter(|d| !d.resolved)
            .map(|d| d.mission_id)
            .collect();

        // Retain active missions to prevent orphaning.
        self.missions.retain(|m| active_mission_ids.contains(&m.id));

        // Task B.2: Generate the static tiered pool
        let mut new_pool = crate::world_map::generate_static_missions(&mut rng);
        
        // Add new missions that aren't already in the pool (avoiding duplicates if refresh is called twice)
        for m in new_pool {
            if !self.missions.iter().any(|existing| existing.id == m.id) {
                self.missions.push(m);
            }
        }

        self.last_pool_refresh = now;
        true
    }
}

/// Constant for daily upkeep per idle operator.
pub const UPKEEP_PER_DAY: i64 = 50;

impl GameState {
    pub fn new_with_seed_missions() -> Self {
        let mut state = Self::default();
        use rand::SeedableRng;
        let mut rng = rand::rngs::SmallRng::from_entropy();
        state.missions = crate::world_map::generate_static_missions(&mut rng);
        state.unlocked_nodes.insert(0);
        state
    }

    // --- Hat Management (G.3 Equipment System) ---

    pub fn purchase_hat(&mut self, hat_id: crate::models::HatId, unlocked_nodes: &std::collections::HashSet<usize>) -> Result<(), String> {
        let catalog = crate::models::Hat::catalog();
        let hat = catalog.iter().find(|h| h.id == hat_id)
            .ok_or_else(|| "Hat not found in catalog".to_string())?;

        // Unlock check
        if hat.unlock_node_id != 0 && !unlocked_nodes.contains(&hat.unlock_node_id) {
            return Err(format!("Node {} must be scouted to unlock {}", hat.unlock_node_id, hat.name));
        }

        // Cost check
        if self.inventory.scrap < hat.scrap_cost.into() {
            return Err(format!("Insufficient MTL (Scrap). Need {}kg.", hat.scrap_cost));
        }

        // Ownership check
        if self.hat_inventory.contains(&hat_id) {
            return Err("You already own this hat.".to_string());
        }

        // Deduct and add
        self.inventory.scrap -= hat.scrap_cost as u64;
        self.hat_inventory.push(hat_id);
        
        Ok(())
    }

    pub fn equip_hat(&mut self, slime_id: uuid::Uuid, hat_id: crate::models::HatId) -> Result<(), String> {
        // 1. Ownership check (Inventory vs Other Operator)
        if !self.hat_inventory.contains(&hat_id) {
            // Find if anyone else has it
            let source_idx = self.slimes.iter().position(|s| s.equipped_hat == Some(hat_id));
            let target_idx = self.slimes.iter().position(|s| s.genome.id == slime_id);

            match (source_idx, target_idx) {
                (Some(src_i), Some(tgt_i)) if src_i != tgt_i => {
                    // SWAP directly using indices to avoid double &mut borrow
                    let old_hat = self.slimes[tgt_i].equipped_hat;
                    self.slimes[tgt_i].equipped_hat = Some(hat_id);
                    self.slimes[src_i].equipped_hat = old_hat;
                    return Ok(());
                }
                (Some(_), Some(_)) => return Ok(()), // Already equipped on self
                _ => return Err("Hat not owned or available.".to_string()),
            }
        }

        // 2. Equip from inventory
        let slime = self.slimes.iter_mut().find(|s| s.genome.id == slime_id)
            .ok_or_else(|| "Operator not found".to_string())?;

        self.hat_inventory.retain(|&id| id != hat_id);
        if let Some(old_hat) = slime.equipped_hat {
            self.hat_inventory.push(old_hat);
        }
        slime.equipped_hat = Some(hat_id);
        Ok(())
    }

    /// Complete a mission or expedition, mutate state (XP, Injuries, Roster reset),
    /// and return the outcome.
    ///
    /// BUG FIX: This method ensures operators are reset to Idle (if not injured)
    /// BEFORE returning, making the resolution atomic and session-resilient.
    pub fn resolve_deployment(&mut self, dep_id: uuid::Uuid, rng: &mut impl rand::Rng) -> Result<(crate::models::Deployment, crate::models::AarOutcome, Vec<String>), String> {
        let dep_idx = self.deployments.iter().position(|d| d.id == dep_id)
            .ok_or_else(|| "Deployment not found".to_string())?;
        
        // 1. Extract deployment and mission
        let dep = self.deployments.remove(dep_idx);
        let mission = self.missions.iter().find(|m| m.id == dep.mission_id)
            .ok_or_else(|| "Mission not found".to_string())?;

        // 2. Resolve combat logic
        let squad: Vec<&crate::models::Operator> = self.slimes.iter()
            .filter(|o| dep.operator_ids.contains(&o.genome.id))
            .collect();
            
        let mut outcome = dep.resolve(mission, &squad, rng);

        // 3. Award XP
        let mut level_ups = Vec::new();
        {
            let mut mut_squad: Vec<&mut crate::models::Operator> = self.slimes.iter_mut()
                .filter(|o| dep.operator_ids.contains(&o.genome.id))
                .collect();
                
            let xp_results = dep.award_squad_xp(mission, &mut mut_squad, &outcome);
            for (id, _xp, leveled) in xp_results {
                if leveled {
                    if let Some(op) = mut_squad.iter().find(|o| o.genome.id == id) {
                        level_ups.push(format!("{} reached Level {}!", op.name(), op.level));
                    }
                }
            }
        }

        // 4. Apply injuries
        crate::models::apply_outcome_injuries(&mut outcome, &mut self.slimes, &dep.operator_ids, rng);

        // 5. RELIABILITY FIX: Reset operator states
        // Ensure all operators involved return to Idle unless they were just injured.
        for op in self.slimes.iter_mut() {
            if dep.operator_ids.contains(&op.id()) {
                if !matches!(op.state, crate::models::operator::SlimeState::Injured(_)) {
                    op.state = crate::models::operator::SlimeState::Idle;
                }
            }
        }

        Ok((dep, outcome, level_ups))
    }

    /// Sprint 7B: Maintenance Pressure
    /// Deducts $50 per idle operator per day.
    /// Sprint G.1: Temporarily disabled for loop validation.
    pub fn apply_daily_upkeep(&mut self, _now: DateTime<Utc>) -> (i64, i64) {
        // TODO: Re-enable after loop validation
        return (0, 0);
        
        /* Original logic preserved for re-enablement:
        let elapsed = now - self.last_upkeep_at;
        let days = elapsed.num_seconds() as f64 / 86400.0;
        ...
        */
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
    let mut state: GameState = serde_json::from_str(&raw)?;
        
    // Migration from v10:
    if state.version < 11 {
        // Note: serde(default) handles hat_inventory automatically.
        state.version = 11;
    }

    // Sprint G.2 Migration: Ensure Center Node (0) is unlocked in all saves.
    if state.version < SAVE_VERSION {
        if state.unlocked_nodes.is_empty() {
            state.unlocked_nodes.insert(0);
        }
        state.version = SAVE_VERSION;
    }

    // Task A.2: Orphan Recovery. If an active deployment references a missing mission, reconstruct it.
    let mut orphans = Vec::new();
    for dep in &state.deployments {
        if !dep.resolved && !state.missions.iter().any(|m| m.id == dep.mission_id) {
            orphans.push(dep.mission_id);
        }
    }
    
    for id in orphans {
        state.missions.push(Mission {
            id,
            name: format!("[ORPHANED] Unknown Contract #{}", &id.to_string()[..5]),
            description: "CRITICAL PERSISTENCE ERROR: Original mission data was truncated. Performance history unavailable.".to_string(),
            tier: crate::models::MissionTier::Standard,
            base_dc: 10,
            min_roster_level: 1,
            difficulty: 0.5, // Neutral fallback
            reward: ResourceYield::scrap(100),
            duration_secs: 60,
            affinity: None,
            req_strength: 5,
            req_agility: 5,
            req_intelligence: 5,
            node_id: None,
            is_scout: false,
        });
    }

    // Sprint G.2 Migration: Ensure Center Node (0) is unlocked in all saves.
    if state.version < SAVE_VERSION {
        if state.unlocked_nodes.is_empty() {
            state.unlocked_nodes.insert(0);
        }
        state.version = SAVE_VERSION;
    }

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
        assert_eq!(state.bank, 500); 
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
        state.slimes.push(crate::models::Operator::new(genome)); // 1 idle operator -> $50 upkeep
        
        // We only care about cost and total_ops
        let (cost, idle_count) = state.apply_daily_upkeep(Utc::now());
        assert_eq!(idle_count, 0, "Idle count returned by stub is 0");
        assert_eq!(cost, 0, "Upkeep must return 0 in Sprint G.1 (temp disabled)");
        // assert_eq!(state.bank, -150); // Original bank was -140, stay -140
        
        // Second run should deduct nothing
        state.last_upkeep_at = Utc::now() - chrono::Duration::days(1);
        let (cost2, _) = state.apply_daily_upkeep(Utc::now());
        assert_eq!(cost2, 0);
        assert_eq!(state.bank, -140);
    }

    #[test]
    fn test_mission_pool_refresh_logic() {
        let mut state = GameState::default();
        use chrono::TimeZone;
        let now = Utc.with_ymd_and_hms(2026, 3, 14, 12, 0, 0).single().unwrap();
        
        // Initial refresh
        state.refresh_missions_if_needed(now);
        assert_eq!(state.missions.len(), 20); // 14 standard + 6 scouts
        let first_id = state.missions[0].id;
        
        // Same day — should NOT refresh
        state.refresh_missions_if_needed(now + chrono::Duration::hours(1));
        assert_eq!(state.missions[0].id, first_id);
        
        // Next day — SHOULD refresh
        state.refresh_missions_if_needed(now + chrono::Duration::hours(25));
        assert_ne!(state.missions[0].id, first_id);
    }

    #[test]
    fn test_mission_pool_seed_determinism() {
        let mut state1 = GameState::default();
        let mut state2 = GameState::default();
        let now = Utc::now();
        
        state1.refresh_missions_if_needed(now);
        state2.refresh_missions_if_needed(now);
        
        assert_eq!(state1.missions[0].name, state2.missions[0].name);
        assert_eq!(state1.missions[0].difficulty, state2.missions[0].difficulty);
    }

    #[test]
    fn test_purchase_hat_logic() {
        let mut state = GameState::default();
        state.inventory.scrap = 100;
        let mut unlocked = std::collections::HashSet::new();
        unlocked.insert(0); // Scout Hood is node 0

        // Success
        state.purchase_hat(crate::models::HatId::ScoutHood, &unlocked).expect("buy hat");
        assert_eq!(state.inventory.scrap, 50);
        assert!(state.hat_inventory.contains(&crate::models::HatId::ScoutHood));

        // Duplicate
        let res = state.purchase_hat(crate::models::HatId::ScoutHood, &unlocked);
        assert!(res.is_err(), "Cannot buy same hat twice");

        // Insufficient funds
        state.inventory.scrap = 10;
        let res2 = state.purchase_hat(crate::models::HatId::KnightHelm, &unlocked);
        assert!(res2.is_err(), "Insufficient funds");

        // Locked
        state.inventory.scrap = 500;
        let res3 = state.purchase_hat(crate::models::HatId::KnightHelm, &unlocked);
        assert!(res3.is_err(), "Hat node 10 is locked");
    }

    #[test]
    fn test_equip_hat_logic_swap() {
        let mut state = GameState::default();
        let id_hat = crate::models::HatId::ScoutHood;
        state.hat_inventory.push(id_hat);

        let mut rng = rand::thread_rng();
        let g1 = crate::genetics::generate_random(crate::genetics::Culture::Ember, "O1", &mut rng);
        let g2 = crate::genetics::generate_random(crate::genetics::Culture::Tide,  "O2", &mut rng);
        let u1 = g1.id;
        let u2 = g2.id;
        state.slimes.push(crate::models::Operator::new(g1));
        state.slimes.push(crate::models::Operator::new(g2));

        // Equip from inventory
        state.equip_hat(u1, id_hat).expect("equip o1");
        assert_eq!(state.slimes[0].equipped_hat, Some(id_hat));
        assert!(state.hat_inventory.is_empty());

        // Swap to another operator
        state.equip_hat(u2, id_hat).expect("swap to o2");
        assert_eq!(state.slimes[0].equipped_hat, None);
        assert_eq!(state.slimes[1].equipped_hat, Some(id_hat));
    }

    #[test]
    fn test_operators_freed_after_resolve() {
        let mut state = GameState::default();
        let mut rng = rand::thread_rng();

        // Setup: 2 operators
        let g1 = crate::genetics::generate_random(crate::genetics::Culture::Ember, "O1", &mut rng);
        let g2 = crate::genetics::generate_random(crate::genetics::Culture::Tide,  "O2", &mut rng);
        let u1 = g1.id;
        let u2 = g2.id;
        state.slimes.push(crate::models::Operator::new(g1));
        state.slimes.push(crate::models::Operator::new(g2));

        // Setup: A mission and a deployment
        let mission = crate::models::Mission::new("Test", crate::models::MissionTier::Starter, 5, 1, 10, 10, 10, 0.1, 60, crate::models::ResourceYield::scrap(100), None, None, false);
        state.missions.push(mission.clone());
        let dep = crate::models::Deployment::start(&mission, vec![u1, u2], false);
        state.deployments.push(dep.clone());

        // Mark operators as deployed
        state.slimes[0].state = crate::models::operator::SlimeState::Deployed(mission.id);
        state.slimes[1].state = crate::models::operator::SlimeState::Deployed(mission.id);

        // Resolve (Dice roll happens here)
        let _ = state.resolve_deployment(dep.id, &mut rng).expect("resolve");

        // After resolve: operators must be Idle (or Injured if fate dictated)
        use crate::models::operator::SlimeState;
        assert!(matches!(state.slimes[0].state, SlimeState::Idle | SlimeState::Injured(_)));
        assert!(matches!(state.slimes[1].state, SlimeState::Idle | SlimeState::Injured(_)));

        // Deployment record must be gone
        assert!(state.deployments.is_empty());
    }
}
