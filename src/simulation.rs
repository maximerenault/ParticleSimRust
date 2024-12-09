use crate::forces::compute_gravity;
use crate::integrator::time_integration;
use crate::particle::Particle;
use crate::quadtree::QuadTree;
use rayon::prelude::*;

pub const DIRECT_SUM: i32 = 0;
pub const DIRECT_SUM_PARALLEL: i32 = 1;
pub const BARNES_HUT: i32 = 2;

pub struct Simulation {
    pub particles: Vec<Particle>,
    pub total_forces: Vec<[f64; 2]>,
    pub dt: f64,
    simulation_type: i32,
    integrator_type: i32,
    theta: Option<f64>,
}

impl Simulation {
    pub fn new(
        particles: Vec<Particle>,
        dt: f64,
        simulation_type: i32,
        integrator_type: i32,
        theta: Option<f64>,
    ) -> Self {
        let total_forces = vec![[0.0, 0.0]; particles.len()];
        Simulation {
            particles,
            total_forces,
            dt,
            simulation_type,
            integrator_type,
            theta,
        }
    }

    pub fn simulation_step(&mut self) {
        if self.simulation_type == DIRECT_SUM {
            self.direct_sum_step()
        } else if self.simulation_type == DIRECT_SUM_PARALLEL {
            self.direct_sum_parallel_step()
        } else if self.simulation_type == BARNES_HUT {
            self.barnes_hut_step(self.theta.expect("Barnes-Hut expects a parameter theta!"))
        }
    }

    fn direct_sum_step(&mut self) {
        let particles = &mut self.particles;
        let total_forces = &mut self.total_forces;
        let dt = self.dt;

        for i in 0..particles.len() {
            for j in i + 1..particles.len() {
                let force = compute_gravity(&particles[i], &particles[j]);
                total_forces[i][0] += force[0];
                total_forces[i][1] += force[1];
                total_forces[j][0] -= force[0];
                total_forces[j][1] -= force[1];
            }
        }

        for (force, particle) in total_forces.iter_mut().zip(particles.iter_mut()) {
            time_integration(particle, force, dt, self.integrator_type);
            *force = [0.0, 0.0];
        }
    }

    fn direct_sum_parallel_step(&mut self) {
        let particles = &mut self.particles;
        let total_forces = &mut self.total_forces;
        let dt = self.dt;

        // Create thread-local buffers for forces
        let mut thread_local_forces: Vec<Vec<[f64; 2]>> =
            vec![vec![[0.0, 0.0]; particles.len()]; rayon::current_num_threads()];

        // Compute gravitational forces in parallel
        thread_local_forces
            .par_iter_mut()
            .enumerate()
            .for_each(|(thread_id, local_forces)| {
                for i in (thread_id..particles.len()).step_by(rayon::current_num_threads()) {
                    for j in i + 1..particles.len() {
                        let force = compute_gravity(&particles[i], &particles[j]);
                        local_forces[i][0] += force[0];
                        local_forces[i][1] += force[1];
                        local_forces[j][0] -= force[0];
                        local_forces[j][1] -= force[1];
                    }
                }
            });

        // Aggregate forces from thread-local buffers
        for local_forces in thread_local_forces {
            for (global_force, local_force) in total_forces.iter_mut().zip(local_forces) {
                global_force[0] += local_force[0];
                global_force[1] += local_force[1];
            }
        }

        // Integrate the motion of particles
        total_forces
            .par_iter_mut()
            .zip(particles.par_iter_mut())
            .for_each(|(force, particle)| {
                time_integration(particle, force, dt, self.integrator_type);
                *force = [0.0, 0.0]; // Reset the force
            });
    }

    fn barnes_hut_step(&mut self, theta: f64) {
        let mut root = QuadTree::new([0.0, 0.0, 800.0, 600.0]);
        for particle in self.particles.iter() {
            root.insert(*particle);
        }

        root.finalize();

        for particle in self.particles.iter_mut() {
            let force = root.compute_force(particle, theta);
            time_integration(particle, &force, self.dt, self.integrator_type);
        }
    }

    pub fn get_particle_positions(&self) -> Vec<[f32; 2]> {
        self.particles
            .iter()
            .map(|p| [p.position[0] as f32, p.position[1] as f32])
            .collect()
    }
}
