use crate::particle::Particle;

pub fn compute_gravity(p1: &Particle, p2: &Particle) -> [f64; 2] {
    let g = 6.67430e-11; // Gravitational constant
    let dx = p2.position[0] - p1.position[0];
    let dy = p2.position[1] - p1.position[1];
    let dist_sq = dx * dx + dy * dy;

    let dist_sq = dist_sq.max(1e-1);

    let force_mag = g * p1.mass * p2.mass / dist_sq;
    
    let dist = dist_sq.sqrt();
    let unit_dx = dx / dist;
    let unit_dy = dy / dist;
    [force_mag * unit_dx, force_mag * unit_dy]
}
