use operator::models::{Operator, Hat, HatId};
use operator::persistence::GameState;
use uuid::Uuid;

#[test]
fn test_hat_stat_bonuses_applied() {
    let mut op = Operator::new(operator::genetics::generate_random_standard());
    let (s, a, i, _, _, _) = op.total_stats();
    
    // Equip Mage Hood (+2 INT)
    op.equipped_hat = Some(HatId::MageHood);
    let new_stats = op.total_stats();
    
    assert_eq!(new_stats.2, i + 2, "INT should increase by 2");
}

#[test]
fn test_purchase_hat_deducts_scrap() {
    let mut state = GameState::default();
    state.inventory.scrap = 1000;
    
    let mut unlocked = std::collections::HashSet::new();
    unlocked.insert(0); // ScoutHood is 0
    unlocked.insert(12); // MageHood is 12
    
    // Purchase Mage Hood (100 Scrap)
    state.purchase_hat(HatId::MageHood, &unlocked).expect("Purchase should succeed");
    
    assert_eq!(state.inventory.scrap, 900);
    assert!(state.hat_inventory.contains(&HatId::MageHood));
}

#[test]
fn test_purchase_unlocked_check() {
    let mut state = GameState::default();
    state.inventory.scrap = 1000;
    
    let mut unlocked = std::collections::HashSet::new();
    unlocked.insert(0);
    
    // Knight Helm (Unlock Node 10)
    let res = state.purchase_hat(HatId::KnightHelm, &unlocked);
    assert!(res.is_err(), "Should be locked (requires Node 10)");
    
    unlocked.insert(10);
    state.purchase_hat(HatId::KnightHelm, &unlocked).expect("Should unlock with node 10");
}

#[test]
fn test_equip_hat_swaps_source() {
    let mut state = GameState::default();
    let id1 = Uuid::new_v4();
    let mut op1 = Operator::new(operator::genetics::generate_random_standard());
    op1.genome.id = id1;
    state.slimes.push(op1);
    
    let id2 = Uuid::new_v4();
    let mut op2 = Operator::new(operator::genetics::generate_random_standard());
    op2.genome.id = id2;
    state.slimes.push(op2);
    
    state.hat_inventory.push(HatId::MageHood); // One Mage Hood owned
    
    // Equip on Slime 1
    state.equip_hat(id1, HatId::MageHood).expect("Equip 1");
    assert_eq!(state.slimes[0].equipped_hat, Some(HatId::MageHood));
    assert!(state.hat_inventory.is_empty());
    
    // Equip same hat on Slime 2 (should swap)
    state.equip_hat(id2, HatId::MageHood).expect("Equip 2 / Swap");
    assert_eq!(state.slimes[1].equipped_hat, Some(HatId::MageHood));
    assert_eq!(state.slimes[0].equipped_hat, None);
}

#[test]
fn test_hat_persistence_roundtrip() {
    let mut state = GameState::default();
    state.hat_inventory.push(HatId::MageHood);
    
    let p = std::env::temp_dir().join("hat_test.json");
    operator::persistence::save(&state, &p).unwrap();
    
    let loaded = operator::persistence::load(&p).unwrap();
    assert!(loaded.hat_inventory.contains(&HatId::MageHood));
    let _ = std::fs::remove_file(p);
}
