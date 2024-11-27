use crate::particle::Particle;

pub fn euler_step(p: &mut Particle, force: [f64; 2], dt: f64) {
    p.velocity[0] += force[0] / p.mass * dt;
    p.velocity[1] += force[1] / p.mass * dt;

    p.position[0] += p.velocity[0] * dt;
    p.position[1] += p.velocity[1] * dt;
}
