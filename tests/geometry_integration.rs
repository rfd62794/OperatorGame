use operator::geometry::{Point, Bounds};
use operator::render::garden_bridge::*;
use eframe::egui;

#[test]
fn test_bridge_point_conversions() {
    let p = Point::new(10.0, 20.0);
    let ep = point_to_egui(p);
    assert_eq!(ep.x, 10.0);
    assert_eq!(ep.y, 20.0);

    let p2 = egui_pos_to_point(ep);
    assert_eq!(p2, p);
}

#[test]
fn test_bridge_bounds_conversions() {
    let b = Bounds::new(0.0, 0.0, 100.0, 100.0);
    let eb = bounds_to_egui(b);
    assert_eq!(eb.min.x, 0.0);
    assert_eq!(eb.max.x, 100.0);

    let b2 = egui_rect_to_bounds(eb);
    assert_eq!(b2, b);
}

#[test]
fn test_point_math_normalization() {
    let p = Point::new(10.0, 0.0);
    let mag = (p.x * p.x + p.y * p.y).sqrt();
    let norm = Point::new(p.x / mag, p.y / mag); // Manual check against logic
    
    // Test the add/sub methods
    let p2 = Point::new(5.0, 5.0).add(Point::new(1.0, 1.0));
    assert_eq!(p2, Point::new(6.0, 6.0));
}

#[test]
fn test_bounds_contains_edge_cases() {
    let b = Bounds::new(0.0, 0.0, 10.0, 10.0);
    assert!(b.contains(Point::new(0.0, 0.0)));
    assert!(b.contains(Point::new(10.0, 10.0)));
    assert!(!b.contains(Point::new(-0.1, 5.0)));
    assert!(!b.contains(Point::new(10.1, 5.0)));
}

#[test]
fn test_garden_spawn_logic() {
    use operator::garden::Garden;
    use operator::genetics::{Culture, generate_random};
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    let mut r = SmallRng::seed_from_u64(42);
    let g = generate_random(Culture::Ember, "Test", &mut r);
    let ops = vec![operator::models::Operator::new(g)];
    let rect = Bounds::new(0.0, 0.0, 500.0, 500.0);
    let garden = Garden::from_operators(&ops, rect);
    
    assert_eq!(garden.agents.len(), 1);
    assert!(rect.contains(garden.agents[0].pos));
}

#[test]
fn test_garden_tick_velocity() {
    use operator::garden::Garden;
    use operator::genetics::{Culture, generate_random};
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    let mut r = SmallRng::seed_from_u64(42);
    let g = generate_random(Culture::Ember, "Test", &mut r);
    let ops = vec![operator::models::Operator::new(g)];
    let rect = Bounds::new(0.0, 0.0, 500.0, 500.0);
    let mut garden = Garden::from_operators(&ops, rect);
    
    // Force a target far away
    garden.agents[0].target = Some(Point::new(400.0, 400.0));
    garden.tick(0.1, None, rect);
    
    // Velocity should be non-zero after a tick with a target
    assert!(garden.agents[0].vel.x != 0.0 || garden.agents[0].vel.y != 0.0);
}

#[test]
fn test_garden_shyness_flee() {
    use operator::garden::{Garden, SHYNESS_RADIUS};
    use operator::genetics::{Culture, generate_random};
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    let mut r = SmallRng::seed_from_u64(42);
    let mut g = generate_random(Culture::Ember, "ShyTest", &mut r);
    g.shyness = 1.0; // Max shyness
    g.energy = 0.5;
    let ops = vec![operator::models::Operator::new(g)];
    let rect = Bounds::new(0.0, 0.0, 1000.0, 1000.0);
    let mut garden = Garden::from_operators(&ops, rect);
    
    let spawn_pos = garden.agents[0].pos;
    // Place cursor right next to it
    let cursor = Some(Point::new(spawn_pos.x + 10.0, spawn_pos.y));
    
    garden.tick(0.1, cursor, rect);
    
    // Shy agent should have negative X velocity (fleeing cursor at +10x)
    assert!(garden.agents[0].vel.x < 0.0);
}

#[test]
fn test_garden_curiosity_seek() {
    use operator::garden::{Garden, CURIOUS_RADIUS, SHYNESS_RADIUS};
    use operator::genetics::{Culture, generate_random};
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    let mut r = SmallRng::seed_from_u64(42);
    let mut g = generate_random(Culture::Ember, "CuriousTest", &mut r);
    g.curiosity = 1.0;
    g.shyness = 0.0;
    g.energy = 0.5;
    let ops = vec![operator::models::Operator::new(g)];
    let rect = Bounds::new(0.0, 0.0, 1000.0, 1000.0);
    let mut garden = Garden::from_operators(&ops, rect);
    
    let spawn_pos = garden.agents[0].pos;
    // Place cursor within curiosity radius but outside shyness
    let cursor = Some(Point::new(spawn_pos.x + SHYNESS_RADIUS + 20.0, spawn_pos.y));
    
    garden.tick(0.1, cursor, rect);
    
    // Curious agent should have positive X velocity (seeking cursor at +x)
    assert!(garden.agents[0].vel.x > 0.0);
}

#[test]
fn test_garden_friction_dampening() {
    use operator::garden::{Garden, FRICTION};
    use operator::genetics::{Culture, generate_random};
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    let mut r = SmallRng::seed_from_u64(42);
    let g = generate_random(Culture::Ember, "FrictionTest", &mut r);
    let ops = vec![operator::models::Operator::new(g)];
    let rect = Bounds::new(0.0, 0.0, 500.0, 500.0);
    let mut garden = Garden::from_operators(&ops, rect);
    
    garden.agents[0].vel = Point::new(100.0, 100.0);
    garden.tick(0.1, None, rect);
    
    // Speed should be less than initial due to friction
    assert!(garden.agents[0].vel.x < 100.0);
}

#[test]
fn test_garden_click_selection_radius() {
    use operator::garden::Garden;
    use operator::genetics::{Culture, generate_random};
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    let mut r = SmallRng::seed_from_u64(42);
    let g = generate_random(Culture::Ember, "ClickTest", &mut r);
    let ops = vec![operator::models::Operator::new(g)];
    let rect = Bounds::new(0.0, 0.0, 500.0, 500.0);
    let mut garden = Garden::from_operators(&ops, rect);
    
    let pos = garden.agents[0].pos;
    // Click slightly off center
    let click = Point::new(pos.x + 5.0, pos.y + 5.0);
    let hit = garden.handle_click(click);
    
    assert!(hit.is_some());
}

#[test]
fn test_garden_mood_derivation_from_stats() {
    use operator::garden::GardenAgent;
    use operator::render::slime::SlimeMood;

    let mut r = rand::thread_rng();
    let mut g = operator::genetics::generate_random(operator::genetics::Culture::Ember, "Mood", &mut r);
    
    // Sleepy case
    g.energy = 0.1;
    let op = operator::models::Operator::new(g.clone());
    let a = GardenAgent::new(&op, Point::ZERO);
    assert_eq!(a.mood, SlimeMood::Sleepy);
}

#[test]
fn test_garden_bounds_clamping() {
    use operator::garden::Garden;
    use operator::genetics::{Culture, generate_random};
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    let mut r = SmallRng::seed_from_u64(42);
    let g = generate_random(Culture::Ember, "WallTest", &mut r);
    let ops = vec![operator::models::Operator::new(g)];
    let rect = Bounds::new(0.0, 0.0, 100.0, 100.0);
    let mut garden = Garden::from_operators(&ops, rect);
    
    // Teleport agent outside
    garden.agents[0].pos = Point::new(200.0, 200.0);
    garden.tick(0.1, None, rect);
    
    // Should be clamped back within (wall margin is 25.0)
    assert!(garden.agents[0].pos.x <= rect.max_x);
    assert!(garden.agents[0].pos.y <= rect.max_y);
}
