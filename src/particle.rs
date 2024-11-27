#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub position: [f64; 2],
    pub velocity: [f64; 2],
    pub mass: f64,
}

impl Particle {
    pub fn new(position: [f64; 2], velocity: [f64; 2], mass: f64) -> Self {
        Particle { position, velocity, mass }
    }
}
