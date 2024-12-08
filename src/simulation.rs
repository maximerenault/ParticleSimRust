use crate::forces::compute_gravity;
use crate::integrator::euler_step;
use crate::particle::Particle;
use crate::quadtree::QuadTree;

pub const DIRECT_SUM: i32 = 0;
pub const BARNES_HUT: i32 = 1;

pub struct Simulation {
    pub particles: Vec<Particle>,
    pub total_forces: Vec<[f64; 2]>,
    pub dt: f64,
    simulation_type: i32,
    theta: Option<f64>,
}

impl Simulation {
    pub fn new(
        particles: Vec<Particle>,
        dt: f64,
        simulation_type: i32,
        theta: Option<f64>,
    ) -> Self {
        let total_forces: Vec<[f64; 2]> = vec![[0.0, 0.0]; particles.len()];
        Simulation {
            particles,
            total_forces,
            dt,
            simulation_type,
            theta: theta,
        }
    }

    pub fn simulation_step(&mut self) {
        if self.simulation_type == DIRECT_SUM {
            self.direct_sum_step()
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
            euler_step(particle, *force, dt);
            *force = [0.0, 0.0];
        }
    }

    fn barnes_hut_step(&mut self, theta: f64) {
        let mut root = QuadTree::new([0.0, 0.0, 800.0, 600.0]);
        for particle in self.particles.iter() {
            root.insert(*particle);
        }

        root.finalize();

        for particle in self.particles.iter_mut() {
            let force = root.compute_force(particle, theta);
            euler_step(particle, force, self.dt);
        }
    }

    pub fn get_particle_positions(&self) -> Vec<[f32; 2]> {
        self.particles
            .iter()
            .map(|p| [p.position[0] as f32, p.position[1] as f32])
            .collect()
    }
}
