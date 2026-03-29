// tests/g1_stability.rs
use operator::persistence::{GameState, load, save};
use operator::models::{Mission, MissionTier, Deployment, ResourceYield, Target};
use operator::world_map::generate_static_missions;
use chrono::{Utc, Duration};
use uuid::Uuid;
use std::collections::HashSet;
use rand::SeedableRng;

#[test]
fn test_g1_anchor_1_orphan_protection() {
    let mut state = GameState::default();
    let mut rng = rand::rngs::SmallRng::seed_from_u64(42);
    state.missions = generate_static_missions(&mut rng);
    
    let active_id = state.missions[0].id;
    state.deployments.push(Deployment {
        id: Uuid::new_v4(),
        mission_id: active_id,
        operator_ids: vec![],
        completes_at: Utc::now() + Duration::hours(1),
        resolved: false,
        is_emergency: false,
    });

    // Refresh on a different day
    let next_day = Utc::now() + Duration::days(1);
    state.refresh_missions_if_needed(next_day);
    
    assert!(state.missions.iter().any(|m| m.id == active_id), "Active mission must be protected from deletion during refresh");
    assert!(state.missions.len() >= 21, "Pool should contain new 20 missions plus the protected one");
}

#[test]
fn test_g1_anchor_2_orphan_reconstruction() {
    let mut state = GameState::default();
    let missing_id = Uuid::new_v4();
    
    state.deployments.push(Deployment {
        id: Uuid::new_v4(),
        mission_id: missing_id,
        operator_ids: vec![],
        completes_at: Utc::now() + Duration::hours(1),
        resolved: false,
        is_emergency: false,
    });
    
    // Simulate save/load cycle to trigger reconstruction logic in persistence::load
    let temp_save = std::env::temp_dir().join(format!("orphan_test_{}.json", Uuid::new_v4()));
    save(&state, &temp_save).unwrap();
    let loaded = load(&temp_save).unwrap();
    
    let reconstructed = loaded.missions.iter().find(|m| m.id == missing_id).expect("Mission should be reconstructed on load if missing from pool");
    assert!(reconstructed.name.contains("[ORPHANED]"), "Reconstructed mission should have [ORPHANED] prefix");
    
    let _ = std::fs::remove_file(&temp_save);
}

#[test]
fn test_g1_anchor_3_static_pool_size() {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(2025);
    let missions = generate_static_missions(&mut rng);
    assert_eq!(missions.len(), 20, "Standard pool size should be 20 (6 scouts + 14 standard)");
}

#[test]
fn test_g1_anchor_4_tier_ranges() {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(2026);
    let missions = generate_static_missions(&mut rng);
    
    for m in missions {
        let dc = m.targets[0].base_dc;
        match m.tier {
            MissionTier::Starter  => assert!((4..=7).contains(&dc), "Starter DC out of range: {}", dc),
            MissionTier::Standard => assert!((6..=11).contains(&dc), "Standard DC out of range: {}", dc),
            MissionTier::Advanced => assert!((10..=15).contains(&dc), "Advanced DC out of range: {}", dc),
            MissionTier::Elite    => assert!((12..=25).contains(&dc), "Elite DC out of range: {}", dc),
        }
    }
}

#[test]
fn test_g1_anchor_5_success_chance_labels() {
    // This essentially tests the label mapping logic in calculate_success_chance
    // Using a dummy mission
    let targets = vec![Target::new("Target 1", 10, 5, 5, 5)];
    let mut m = Mission::new("Test", MissionTier::Standard, targets, 1, 0.5, 60, ResourceYield::scrap(100), None, None, false);
    
    let (label, chance) = m.calculate_success_chance(&[]);
    assert_eq!(label, "UNSTAFFED");
    assert_eq!(chance, 0.0);
}

#[test]
fn test_g1_anchor_6_upkeep_disabled() {
    let mut state = GameState::default();
    state.last_upkeep_at = Utc::now() - Duration::days(5);
    let (cost, _) = state.apply_daily_upkeep(Utc::now());
    assert_eq!(cost, 0, "Upkeep must be zeroed for Sprint G.1 validation cycle");
}

#[test]
fn test_g1_anchor_8_tier_coverage() {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(999);
    let missions = generate_static_missions(&mut rng);
    
    let mut tiers = HashSet::new();
    for m in missions {
        tiers.insert(format!("{:?}", m.tier));
    }
    
    assert!(tiers.contains("Starter"));
    assert!(tiers.contains("Standard"));
    assert!(tiers.contains("Advanced"));
    assert!(tiers.contains("Elite"));
}

#[test]
fn test_g1_anchor_10_level_scaling_preview() {
    let targets = vec![Target::new("Apex", 12, 10, 10, 10)];
    let m = Mission::new("T3 Test", MissionTier::Elite, targets, 6, 0.5, 3600, ResourceYield::scrap(5000), None, None, false);
    assert_eq!(m.targets[0].base_dc, 12);
}
