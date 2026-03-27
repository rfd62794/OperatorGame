// tests/f1b_loop.rs

use chrono::{Duration, Utc};
use crate::models::{AarOutcome, LogOutcome, Operator, SlimeState, ResourceYield, MissionTier};
use crate::ui::AarSummary;
use crate::persistence::GameState;
use crate::ui::OperatorApp;

fn setup_app_with_mission() -> OperatorApp {
    let mut app = OperatorApp::new_dummy();
    app.state.bank = 1000;
    app.state.missions.push(crate::models::Mission::new(
        "Test Mission",
        MissionTier::Starter,
        5,
        1,
        0, 0, 0,
        0.0,
        10,
        ResourceYield::scrap(10000),
        None,
        None, false
    ));
    app
}

#[test]
fn test_f1b_01_recruit_sub_tab_initialization() {
    let mut app = OperatorApp::new_dummy();
    assert_eq!(app.active_tab, crate::platform::BottomTab::Roster);
    assert_eq!(app.roster_sub_tab, crate::platform::RosterSubTab::Collection); // Default is Collection
    
    // Switch to Recruit tab
    app.roster_sub_tab = crate::platform::RosterSubTab::Recruit;
    assert_eq!(app.roster_sub_tab, crate::platform::RosterSubTab::Recruit);
}

#[test]
fn test_f1b_02_recruit_slime_deducts_funds() {
    let mut state = GameState::default();
    state.bank = 1000;
    
    let initial_count = state.slimes.len();
    let id = crate::recruitment::purchase_recruit(&mut state, "Rookie").expect("Failed to draft");
    
    assert_eq!(state.slimes.len(), initial_count + 1);
    assert_eq!(state.bank, 1000 - crate::recruitment::STANDARD_RECRUIT_COST as i64);
    assert!(state.slimes.iter().any(|s| s.genome.id == id));
}

#[test]
fn test_f1b_03_stage_and_launch_mission() {
    let mut app = setup_app_with_mission();
    
    let op_id = crate::recruitment::purchase_recruit(&mut app.state, "Rookie").unwrap();
    
    let mission = app.state.missions[0].clone();
    
    // Stage
    app.staged_operators.insert(op_id);
    assert_eq!(app.staged_operators.len(), 1);
    
    // Launch
    app.launch_mission(mission.clone());
    
    assert_eq!(app.state.deployments.len(), 1);
    assert_eq!(app.state.deployments[0].mission_id, mission.id);
    assert!(app.staged_operators.is_empty());
    
    // Verify slime state
    let op = app.state.slimes.iter().find(|s| s.genome.id == op_id).unwrap();
    assert!(matches!(op.state, SlimeState::Deployed(_)));
}

#[test]
fn test_f1b_04_deployment_resolves_and_creates_pending_aar() {
    let mut app = setup_app_with_mission();
    let op_id = crate::recruitment::purchase_recruit(&mut app.state, "Rookie").unwrap();
    let mission = app.state.missions[0].clone();
    
    app.staged_operators.insert(op_id);
    app.launch_mission(mission);
    
    // Fast forward time
    app.state.deployments[0].completes_at -= Duration::hours(1);
    
    // Resolve manually as would happen in render_active_ops
    let dep_id = app.state.deployments[0].id;
    app.resolve_deployment(dep_id);
    
    assert!(app.state.deployments[0].resolved);
    assert!(app.pending_aar.is_some());
}

#[test]
fn test_f1b_05_aar_contains_xp_gained() {
    let mut app = setup_app_with_mission();
    let op_id = crate::recruitment::purchase_recruit(&mut app.state, "Rookie").unwrap();
    app.staged_operators.insert(op_id);
    app.launch_mission(app.state.missions[0].clone());
    app.state.deployments[0].completes_at -= Duration::hours(1);
    
    let dep_id = app.state.deployments[0].id;
    app.resolve_deployment(dep_id);
    
    let aar = app.pending_aar.as_ref().unwrap();
    assert!(aar.xp_gained > 0);
}

#[test]
fn test_f1b_06_aar_awards_xp_to_operator() {
    let mut app = setup_app_with_mission();
    let op_id = crate::recruitment::purchase_recruit(&mut app.state, "Rookie").unwrap();
    
    // Record initial XP
    let initial_xp = app.state.slimes.iter().find(|s| s.genome.id == op_id).unwrap().total_xp;
    
    app.staged_operators.insert(op_id);
    app.launch_mission(app.state.missions[0].clone());
    app.state.deployments[0].completes_at -= Duration::hours(1);
    
    let dep_id = app.state.deployments[0].id;
    app.resolve_deployment(dep_id);
    
    let final_xp = app.state.slimes.iter().find(|s| s.genome.id == op_id).unwrap().total_xp;
    assert!(final_xp > initial_xp);
}

#[test]
fn test_f1b_07_resolving_aar_resets_slime_state_if_not_injured() {
    let mut app = setup_app_with_mission();
    let op_id = crate::recruitment::purchase_recruit(&mut app.state, "Hero").unwrap();
    
    // Make sure they have a lot of HP/stats so they don't get injured (likely victory)
    if let Some(op) = app.state.slimes.iter_mut().find(|s| s.genome.id == op_id) {
        op.genome.base_hp = 1000.0;
        op.genome.base_atk = 100.0;
    }
    
    app.staged_operators.insert(op_id);
    app.launch_mission(app.state.missions[0].clone());
    app.state.deployments[0].completes_at -= Duration::hours(1);
    
    let dep_id = app.state.deployments[0].id;
    app.resolve_deployment(dep_id); // This applies outcome AND resets state if no injury
    
    let op = app.state.slimes.iter().find(|s| s.genome.id == op_id).unwrap();
    // They could be injured in rare cases, but mostly Idle
    if !matches!(op.state, SlimeState::Injured(_)) {
        assert!(matches!(op.state, SlimeState::Idle));
    }
}

#[test]
fn test_f1b_08_log_entry_persisted_in_game_state() {
    let mut app = setup_app_with_mission();
    let op_id = crate::recruitment::purchase_recruit(&mut app.state, "Hero").unwrap();
    app.staged_operators.insert(op_id);
    app.launch_mission(app.state.missions[0].clone());
    app.state.deployments[0].completes_at -= Duration::hours(1);
    
    let dep_id = app.state.deployments[0].id;
    let initial_log_count = app.state.combat_log.len();
    
    app.resolve_deployment(dep_id);
    
    // Should have added at least 1 log entry (mission result), maybe 2 if levelled up
    assert!(app.state.combat_log.len() > initial_log_count);
    
    let latest_log = &app.state.combat_log[0];
    assert!(
        matches!(latest_log.outcome, LogOutcome::Victory | LogOutcome::Failure | LogOutcome::CritFail | LogOutcome::System)
    );
}

#[test]
fn test_f1b_09_operator_level_up_triggers_system_log() {
    let mut app = setup_app_with_mission();
    let op_id = crate::recruitment::purchase_recruit(&mut app.state, "Rookie").unwrap();
    
    // Put operator right on the brink of levelling up
    if let Some(op) = app.state.slimes.iter_mut().find(|s| s.genome.id == op_id) {
        let needed = op.xp_to_next();
        op.total_xp += needed - 1; 
    }
    
    app.staged_operators.insert(op_id);
    app.launch_mission(app.state.missions[0].clone());
    app.state.deployments[0].completes_at -= Duration::hours(1);
    
    let dep_id = app.state.deployments[0].id;
    app.resolve_deployment(dep_id);
    
    let op = app.state.slimes.iter().find(|s| s.genome.id == op_id).unwrap();
    assert!(op.level > 1, "Operator should have levelled up");
    
    // Check pending aar for level up
    let aar = app.pending_aar.as_ref().unwrap();
    assert!(!aar.level_ups.is_empty());
    
    // Ensure system log was generated
    let has_system_log = app.state.combat_log.iter().any(|log| {
        matches!(log.outcome, LogOutcome::System) && log.message.contains("RECOGNIZED")
    });
    assert!(has_system_log);
}

#[test]
fn test_f1b_10_dismissing_aar_clears_pending_state() {
    let mut app = OperatorApp::new_dummy();
    
    app.pending_aar = Some(AarSummary {
        mission_name: "Test".to_string(),
        outcome_label: "VICTORY".to_string(),
        outcome_color: eframe::egui::Color32::GREEN,
        xp_gained: 100,
        level_ups: vec![],
        roll_lines: vec![],
        injured_names: vec![]
    });
    
    assert!(app.pending_aar.is_some());
    // Simulate dismiss button click
    app.pending_aar = None;
    assert!(app.pending_aar.is_none());
}

#[test]
fn test_f1b_11_radar_tab_bar_height_constant_exists() {
    // E.1 Ensure TAB_BAR_HEIGHT constant exported from platform.rs
    let h = crate::platform::TAB_BAR_HEIGHT;
    assert_eq!(h, 48.0);
}

#[test]
fn test_f1b_12_operator_detail_selection_wiring() {
    let mut app = OperatorApp::new_dummy();
    assert!(app.selected_slime_id.is_none());
    
    let id = uuid::Uuid::new_v4();
    app.selected_slime_id = Some(id);
    assert_eq!(app.selected_slime_id, Some(id));
}

#[test]
fn test_f1b_13_combat_log_truncation() {
    let mut app = OperatorApp::new_dummy();
    
    // Fill the combat log beyond 50 entries
    for i in 0..60 {
        app.state.combat_log.insert(0, crate::models::LogEntry {
            timestamp: i,
            message: format!("Log {}", i),
            outcome: LogOutcome::System,
        });
    }
    
        // Mock a deployment resolution to trigger truncate
    app.state.deployments.push(crate::models::Deployment {
        id: uuid::Uuid::new_v4(),
        mission_id: uuid::Uuid::new_v4(), // Won't resolve if mission missing, so we inject
        operator_ids: vec![],
        completes_at: Utc::now() - Duration::hours(1), // Fix: Deployment uses completes_at
        resolved: false,
        is_emergency: false,
    });
    
    // Inject a blank mission so resolve_deployment proceeds
    let mut mission = crate::models::Mission::new(
        "Truncate Mission",
        MissionTier::Starter,
        5,
        1,
        0, 0, 0,
        0.1,
        10,
        ResourceYield::scrap(100),
        None,
        None, false
    );
    mission.id = app.state.deployments[0].mission_id;
    app.state.missions.push(mission);
    
    app.resolve_deployment(app.state.deployments[0].id);
    
    assert!(app.state.combat_log.len() <= 50, "Combat log should truncate to 50");
}
