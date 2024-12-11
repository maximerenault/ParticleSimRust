use crate::particle::Particle;
use std::f64::consts::PI;

pub const GRAVIT_CONST: f64 = 4.0 * PI * PI;

pub fn compute_gravity(p1: &Particle, p2: &Particle) -> [f64; 2] {
    let dx = p2.position[0] - p1.position[0];
    let dy = p2.position[1] - p1.position[1];
    let dist_sq = dx * dx + dy * dy;

    let dist_sq = dist_sq.max(1.0);

    let force_mag = GRAVIT_CONST * p1.mass * p2.mass / dist_sq;

    let dist = dist_sq.sqrt();
    let unit_dx = dx / dist;
    let unit_dy = dy / dist;
    [force_mag * unit_dx, force_mag * unit_dy]
}
