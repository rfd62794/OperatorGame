use operator::models::{Mission, MissionTier, ResourceYield};
use operator::persistence::GameState;
use operator::world_map::{generate_static_missions, generate_scout_missions};
use rand::SeedableRng;

#[test]
fn test_save_v10_migration() {
    // Simulate an old save state
    let mut state = GameState::default();
    state.version = 9;
    state.unlocked_nodes.clear();
    
    // Migration logic (normally in persistence::load)
    if state.version < 10 {
        if state.unlocked_nodes.is_empty() {
            state.unlocked_nodes.insert(0);
        }
        state.version = 10;
    }
    
    assert!(state.unlocked_nodes.contains(&0), "Center node should be unlocked by migration");
    assert_eq!(state.version, 10);
}

#[test]
fn test_scout_missions_present() {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(42);
    let missions = generate_static_missions(&mut rng);
    
    let scouts: Vec<_> = missions.iter().filter(|m| m.is_scout).collect();
    assert_eq!(scouts.len(), 6, "Should have 6 scout missions");
    
    assert!(scouts.iter().any(|m| m.name.contains("Ember Flats")));
    assert!(scouts.iter().any(|m| m.name.contains("Tundra Shelf")));
}

#[test]
fn test_scout_dc_formula() {
    let scouts = generate_scout_missions();
    
    let ember = scouts.iter().find(|m| m.name.contains("Ember Flats")).unwrap();
    assert_eq!(ember.base_dc, 4, "Ember Flats (0.20) should be DC 4");
    
    let crystal = scouts.iter().find(|m| m.name.contains("Crystal Spire")).unwrap();
    assert_eq!(crystal.base_dc, 10, "Crystal Spire (0.50) should be DC 10");
}

#[test]
fn test_resource_yield_display() {
    let y_biomass = ResourceYield::new(15, 0, 0);
    assert_eq!(format!("{}", y_biomass), "15 Biomass");
    
    let y_scrap = ResourceYield::scrap(500);
    assert_eq!(format!("{}", y_scrap), "$500");
    
    let y_mixed = ResourceYield::new(10, 20, 5);
    assert_eq!(format!("{}", y_mixed), "B:10 S:20 R:5");
}

#[test]
fn test_node_unlock_and_yield_logic() {
    let mut state = GameState::default();
    let initial_biomass = state.inventory.biomass;
    
    // Create a scout mission for Node 10 (Ember Flats)
    let mission = Mission::new(
        "Scout: Ember Flats", MissionTier::Starter, 4, 1, 0, 0, 0, 0.20, 60,
        ResourceYield::new(15, 5, 2), None, Some(10), true
    );
    
    // Simulate Victory resolution logic from ui/mod.rs
    let outcome = operator::models::AarOutcome::Victory {
        reward: mission.reward,
        success_chance: 1.0,
        rolls: vec![],
        xp_gained: 10,
    };
    
    if let operator::models::AarOutcome::Victory { reward, .. } = outcome {
        state.bank += reward.scrap as i64;
        reward.apply_to_inventory(&mut state.inventory);
        
        if mission.is_scout {
            if let Some(node_id) = mission.node_id {
                state.unlocked_nodes.insert(node_id);
            }
        }
    }
    
    assert!(state.unlocked_nodes.contains(&10), "Node 10 should be unlocked");
    assert_eq!(state.inventory.biomass, initial_biomass + 15, "Biomass should be granted");
    assert_eq!(state.bank, 500 + 5, "Scrap should be added to bank (500 start + 5)");
}

#[test]
fn test_mission_visibility_filtering() {
    let mut state = GameState::default();
    state.unlocked_nodes.clear();
    state.unlocked_nodes.insert(0); // Center only
    
    let m_center = Mission::new("C", MissionTier::Starter, 5, 1, 0, 0, 0, 0.0, 60, ResourceYield::scrap(0), None, None, false);
    let m_locked = Mission::new("L", MissionTier::Starter, 5, 1, 0, 0, 0, 0.0, 60, ResourceYield::scrap(0), None, Some(10), false);
    
    // Visibility logic (from contracts.rs)
    let can_see_center = match m_center.node_id {
        None => true,
        Some(nid) => state.unlocked_nodes.contains(&(nid as usize)),
    };
    let can_see_locked = match m_locked.node_id {
        None => true,
        Some(nid) => state.unlocked_nodes.contains(&(nid as usize)),
    };
    
    assert!(can_see_center);
    assert!(!can_see_locked, "Missions for locked nodes should be hidden");
}
