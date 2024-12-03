mod particle;
mod forces;
mod integrator;
mod simulation;
mod utils;
mod visualization;

use ggez::{ContextBuilder, event};
use visualization::SimulationVisualizer;
use crate::simulation::Simulation;

fn main() {
    let n = 1000;
    let particles = utils::generate_random_particles(n);
    let dt = 0.1;

    let simulation = Simulation::new(particles, dt);

    let (ctx, event_loop) = ContextBuilder::new("Particle simulation", "Maxime Renault")
        .build()
        .expect("Failed to create ggez context");

    let visualizer = SimulationVisualizer::new(simulation);

    event::run(ctx, event_loop, visualizer);
}
