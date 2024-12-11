// Project: ParticleSimRust
// A simple gravity simulation using the Barnes-Hut algorithm
// Time unit : 1 year
// Distance unit : 1 AU
// Mass unit : 1 solar mass
// Gravitational constant : 4 * pi^2
// -------------------------------------
// Author: Maxime Renault, 2024


mod forces;
mod integrator;
mod particle;
mod quadtree;
mod simstate;
mod simulation;
mod simulationloop;
mod utils;
mod visualization;

use crate::integrator::{EULER, LEAPFROG, MIDPOINT};
use crate::simstate::SimState;
use crate::simulation::{
    Simulation, BARNES_HUT, BARNES_HUT_PARALLEL, DIRECT_SUM, DIRECT_SUM_PARALLEL,
};
use crate::simulationloop::simulationloop;
use crate::visualization::SimulationVisualizer;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{event, ContextBuilder};
use std::sync::{Arc, RwLock};
use std::time::Duration;

fn main() {
    let n = 10_000;
    let particles = utils::generate_random_particles_around_attractor(n);
    let dt = 0.001;
    let speed = 1.0;
    let step_duration = Duration::from_secs_f64(dt / speed);
    let simulation_type = BARNES_HUT_PARALLEL;
    let integrator_type = LEAPFROG;
    let theta = 2.0;

    let shared_state = Arc::new(RwLock::new(SimState::new(n)));
    let simulation = Simulation::new(particles, dt, simulation_type, integrator_type, Some(theta));
    let visualizer = SimulationVisualizer::new(shared_state.clone());

    let (ctx, event_loop) = ContextBuilder::new("GravitSim", "Maxime Renault")
        .window_setup(WindowSetup::default().title("Gravitation simulation"))
        .window_mode(WindowMode::default().dimensions(1500.0, 900.0))
        .build()
        .expect("Failed to create ggez context");

    simulationloop(shared_state.clone(), simulation, step_duration, 20.0);
    event::run(ctx, event_loop, visualizer);
}
