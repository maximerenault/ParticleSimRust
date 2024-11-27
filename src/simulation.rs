use crate::particle::Particle;
use crate::forces::compute_gravity;
use crate::integrator::euler_step;

pub fn run_simulation(particles: &mut Vec<Particle>, dt: f64, steps: usize) {
    for _ in 0..steps {
        for i in 0..particles.len() {
            let mut total_force = [0.0, 0.0];
            for j in 0..particles.len() {
                if i != j {
                    let force = compute_gravity(&particles[i], &particles[j]);
                    total_force[0] += force[0];
                    total_force[1] += force[1];
                }
            }
            euler_step(&mut particles[i], total_force, dt);
        }
    }
}
