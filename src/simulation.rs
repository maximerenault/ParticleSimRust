use crate::particle::Particle;
use crate::forces::compute_gravity;
use crate::integrator::euler_step;

pub struct Simulation {
    pub particles: Vec<Particle>,
    pub total_forces: Vec<[f64; 2]>,
    pub dt: f64,
}

impl Simulation {
    pub fn new(particles: Vec<Particle>, dt: f64) -> Self {
        let total_forces: Vec<[f64; 2]> = vec![[0.0, 0.0]; particles.len()];
        Simulation { particles, total_forces, dt }
    }

    pub fn simulation_step(&mut self) {
        let particles = &mut self.particles;
        let total_forces = &mut self.total_forces;
        let dt = self.dt;
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

    pub fn get_particle_positions(&self) -> Vec<[f32; 2]> {
        self.particles
            .iter()
            .map(|p| [p.position[0] as f32, p.position[1] as f32])
            .collect()
    }
}