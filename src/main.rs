mod forces;
mod integrator;
mod particle;
mod quadtree;
mod simstate;
mod simulation;
mod simulationloop;
mod utils;
mod visualization;

use crate::simstate::SimState;
use crate::simulation::{Simulation, BARNES_HUT, DIRECT_SUM};
use crate::simulationloop::simulationloop;
use crate::visualization::SimulationVisualizer;
use ggez::{event, ContextBuilder};
use std::sync::{Arc, RwLock};
use std::time::Duration;

fn main() {
    let n = 1000;
    let particles = utils::generate_random_particles(n);
    let dt = 0.01;
    let speed = 1.0;
    let step_duration = Duration::from_secs_f64(dt / speed);
    let simulation_type = BARNES_HUT;
    let theta = 0.5;

    let shared_state = Arc::new(RwLock::new(SimState::new(n)));
    let simulation = Simulation::new(particles, dt, simulation_type, Some(theta));
    let visualizer = SimulationVisualizer::new(shared_state.clone());

    let (ctx, event_loop) = ContextBuilder::new("Particle simulation", "Maxime Renault")
        .build()
        .expect("Failed to create ggez context");

    simulationloop(shared_state.clone(), simulation, step_duration, 30.0);
    event::run(ctx, event_loop, visualizer);
}
