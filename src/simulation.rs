use crate::particle::Particle;
use crate::forces::compute_gravity;
use crate::integrator::euler_step;

pub fn simulation_step(particles: &mut Vec<Particle>, total_forces: &mut Vec<[f64; 2]>, dt: f64) {
    for i in 0..particles.len() {
        for j in i+1..particles.len() {
            let force = compute_gravity(&particles[i], &particles[j]);
            total_forces[i][0] += force[0];
            total_forces[i][1] += force[1];
            total_forces[j][0] -= force[0];
            total_forces[j][1] -= force[1];
        }
    }
    for (force, particle) in total_forces.iter_mut().zip(particles.iter_mut()) {
        euler_step(particle, *force, dt);
        *force = [0.0, 0.0];
    }
}
