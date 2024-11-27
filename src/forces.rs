use crate::particle::Particle;

pub fn compute_gravity(p1: &Particle, p2: &Particle) -> [f64; 2] {
    let g = 6.67430e-11; // Gravitational constant
    let dx = p2.position[0] - p1.position[0];
    let dy = p2.position[1] - p1.position[1];
    let dist_sq = dx * dx + dy * dy;
    let dist = dist_sq.sqrt();

    if dist == 0.0 {
        return [0.0, 0.0];
    }

    let force_magnitude = g * p1.mass * p2.mass / dist_sq;
    [force_magnitude * dx / dist, force_magnitude * dy / dist]
}
