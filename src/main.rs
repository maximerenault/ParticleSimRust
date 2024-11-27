mod particle;
mod forces;
mod integrator;
mod simulation;
mod utils;

use crate::simulation::run_simulation;
use crate::utils::generate_random_particles;

fn main() {
    let mut particles = generate_random_particles(10);
    let dt = 0.01;
    let steps = 100;

    println!("Initial state: {:?}", particles);
    run_simulation(&mut particles, dt, steps);
    println!("Final state: {:?}", particles);
}
