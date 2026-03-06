use crate::genetics::Culture;

pub struct RadialNode {
    pub id: u32,
    pub ring: u8,
    pub position: (f32, f32), // (x, y) for UI rendering
    pub culture: Culture,
    pub difficulty_dc: u32,
}

pub fn generate_ripple_map() -> Vec<RadialNode> {
    let mut nodes = Vec::new();
    let center = (0.0, 0.0);

    // Node 0: The Hidden Meadow
    nodes.push(RadialNode {
        id: 0,
        ring: 0,
        position: center,
        culture: Culture::Void,
        difficulty_dc: 0,
    });

    // Ring Generation Loop
    for ring in 1..=3 {
        let count = 6;
        let radius = ring as f32 * 100.0 + 20.0; // r: 120, 220, 320
        let offset = if ring == 2 { 30.0_f32.to_radians() } else { 0.0 };
        
        for i in 0..count {
            let angle = (i as f32 * (360.0 / count as f32)).to_radians() + offset;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            
            // Assign Cultures based on Trinary RPS mapped to existing cultures
            // Ring 1 gets Primaries (Ember, Gale, Tide) repeated
            // Ring 2 gets Secondaries (Marsh, Crystal, Tundra) repeated
            // Ring 3 gets Mixed (Ember, Gale, Tide...) repeated
            let culture = match ring {
                1 => match i % 3 { 0 => Culture::Ember, 1 => Culture::Gale, _ => Culture::Tide },
                2 => match i % 3 { 0 => Culture::Marsh, 1 => Culture::Crystal, _ => Culture::Tundra },
                _ => match i % 3 { 0 => Culture::Ember, 1 => Culture::Gale, _ => Culture::Tide },
            };

            // Calculate basic pseudo-random difficulty (deterministic for now to avoid threading rng through here if not strictly needed)
            let diff = ring as u32 * 10 - ((i * 7) % 5);

            nodes.push(RadialNode {
                id: (ring * 10 + i) as u32, // Unique ID based on ring/index
                ring: ring as u8,
                position: (x, y),
                culture,
                difficulty_dc: diff,
            });
        }
    }
    nodes
}
