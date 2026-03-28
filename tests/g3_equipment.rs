use operator::models::{Operator, Hat, HatId};
use operator::persistence::GameState;
use uuid::Uuid;

#[test]
fn test_hat_stat_bonuses_applied() {
    let mut op = Operator::new(crate::operator::genetics::generate_random_standard());
    let base_stats = op.total_stats();
    
    // Equip Mage Hood (+5 INT)
    op.equipped_hat = Some(1); // Mage Hood
    let new_stats = op.total_stats();
    
    assert_eq!(new_stats.2, base_stats.2 + 5, "INT should increase by 5");
}

#[test]
fn test_purchase_hat_deducts_scrap() {
    let mut state = GameState::default();
    state.inventory.scrap = 1000;
    
    let mut unlocked = std::collections::HashSet::new();
    unlocked.insert(0);
    // Purchase Mage Hood (150 Scrap)
    state.purchase_hat(1, &unlocked).expect("Purchase should succeed");
    
    assert_eq!(state.inventory.scrap, 850);
    assert!(state.hat_inventory.contains(&1));
}

#[test]
fn test_purchase_unlocked_check() {
    let mut state = GameState::default();
    state.inventory.scrap = 1000;
    
    let mut unlocked = std::collections::HashSet::new();
    unlocked.insert(1);
    unlocked.insert(2);
    // Plate Helm (Unlock Node 5)
    let res = state.purchase_hat(2, &unlocked); // Not 5
    assert!(res.is_err(), "Should be locked");
    
    unlocked.insert(5);
    state.purchase_hat(2, &unlocked).expect("Should unlock with node 5");
}

#[test]
fn test_equip_hat_swaps_source() {
    let mut state = GameState::default();
    let id1 = Uuid::new_v4();
    let mut op1 = Operator::new(crate::operator::genetics::generate_random_standard());
    op1.genome.id = id1;
    state.slimes.push(op1);
    
    let id2 = Uuid::new_v4();
    let mut op2 = Operator::new(crate::operator::genetics::generate_random_standard());
    op2.genome.id = id2;
    state.slimes.push(op2);
    
    state.hat_inventory.push(1); // One Mage Hood owned
    
    // Equip on Slime 1
    state.equip_hat(id1, 1).expect("Equip 1");
    assert_eq!(state.slimes[0].equipped_hat, Some(1));
    assert!(state.hat_inventory.is_empty());
    
    // Equip same hat on Slime 2 (should swap)
    state.equip_hat(id2, 1).expect("Equip 2 / Swap");
    assert_eq!(state.slimes[1].equipped_hat, Some(1));
    assert_eq!(state.slimes[0].equipped_hat, None);
}

#[test]
fn test_hat_persistence_roundtrip() {
    let mut state = GameState::default();
    state.hat_inventory.push(1);
    
    let p = std::env::temp_dir().join("hat_test.json");
    operator::persistence::save(&state, &p).unwrap();
    
    let loaded = operator::persistence::load(&p).unwrap();
    assert!(loaded.hat_inventory.contains(&1));
    let _ = std::fs::remove_file(p);
}
