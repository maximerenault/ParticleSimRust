mod forces;
mod integrator;
mod particle;
mod quadtree;
mod simstate;
mod simulation;
mod simulationloop;
mod utils;
mod visualization;

use crate::integrator::{EULER, LEAPFROG, MIDPOINT, RK4};
use crate::simstate::SimState;
use crate::simulation::{Simulation, BARNES_HUT, DIRECT_SUM, DIRECT_SUM_PARALLEL};
use crate::simulationloop::simulationloop;
use crate::visualization::SimulationVisualizer;
use ggez::{event, graphics, ContextBuilder};
use std::sync::{Arc, RwLock};
use std::time::Duration;

fn main() {
    let n = 500;
    let particles = utils::generate_random_particles_around_attractor(n);
    let dt = 0.001;
    let speed = 0.1;
    let step_duration = Duration::from_secs_f64(dt / speed);
    let simulation_type = DIRECT_SUM_PARALLEL;
    let integrator_type = LEAPFROG;
    let theta = 0.5;

    let shared_state = Arc::new(RwLock::new(SimState::new(n)));
    let simulation = Simulation::new(particles, dt, simulation_type, integrator_type, Some(theta));
    let visualizer = SimulationVisualizer::new(shared_state.clone());

    let (ctx, event_loop) = ContextBuilder::new("GravitSim", "Maxime Renault")
        .build()
        .expect("Failed to create ggez context");
    graphics::set_window_title(&ctx, "Gravitation simulation");

    simulationloop(shared_state.clone(), simulation, step_duration, 30.0);
    event::run(ctx, event_loop, visualizer);
}
