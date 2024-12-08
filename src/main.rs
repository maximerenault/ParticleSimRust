mod forces;
mod integrator;
mod particle;
mod quadtree;
mod simulation;
mod utils;
mod visualization;

use crate::simulation::{Simulation, BARNES_HUT, DIRECT_SUM};
use ggez::{event, ContextBuilder};
use visualization::SimulationVisualizer;

fn main() {
    let n = 100;
    let particles = utils::generate_random_particles(n);
    let dt = 0.01;
    let simulation_type = BARNES_HUT;
    let theta = 0.5;

    let simulation = Simulation::new(particles, dt, simulation_type, Some(theta));

    let (ctx, event_loop) = ContextBuilder::new("Particle simulation", "Maxime Renault")
        .build()
        .expect("Failed to create ggez context");

    let visualizer = SimulationVisualizer::new(simulation);

    event::run(ctx, event_loop, visualizer);
}
