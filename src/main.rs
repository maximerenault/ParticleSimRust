mod particle;
mod forces;
mod integrator;
mod simulation;
mod utils;
mod visualization;

use ggez::{ContextBuilder, event};
use visualization::SimulationVisualizer;

fn main() {
    let n = 1000;
    let particles = utils::generate_random_particles(n);
    let dt = 0.1; // Define a small time step for the simulation

    let (ctx, event_loop) = ContextBuilder::new("particle_simulation", "Maxime Renault")
        .build()
        .expect("Failed to create ggez context");

    let visualizer = SimulationVisualizer::new(particles, dt);

    event::run(ctx, event_loop, visualizer);
}
