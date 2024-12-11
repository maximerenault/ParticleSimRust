use crate::simstate::SimState;
use crate::simulation::Simulation;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

pub fn simulationloop(
    shared_state: Arc<RwLock<SimState>>,
    simulation: Simulation,
    step_duration: Duration, // Target duration for each simulation step
    update_frequency: f64,
) {
    thread::spawn(move || {
        let mut sim = simulation;
        let mut last_step_time = Instant::now();

        let speed_update_window = Duration::from_secs_f64(0.5); // speed updated every .5 secs
        let mut speed_update_time = Instant::now();
        let mut sim_speed = 0.0;
        let mut step_count = 0;

        let frame_duration = Duration::from_secs_f64(1.0 / update_frequency);
        let mut frame_update_time = Instant::now();

        let mut sim_time = 0.0;
        let mut sim_steps = 0;

        if let Ok(mut state) = shared_state.write() {
            state.start_time = Instant::now();
        }

        loop {
            // Limit sim speed
            let now = Instant::now();
            thread::sleep(step_duration.saturating_sub(now.duration_since(last_step_time)));
            last_step_time = Instant::now();

            sim.simulation_step();
            step_count += 1;

            // Compute actual sim speed
            if speed_update_time.elapsed() >= speed_update_window {
                sim_speed = (step_count as f64 * sim.dt) / speed_update_window.as_secs_f64();
                step_count = 0;
                speed_update_time = Instant::now();
            }

            sim_time += sim.dt;
            sim_steps += 1;

            // Update shared state if needed
            if frame_update_time.elapsed() >= frame_duration {
                let positions = sim.get_particle_positions();

                if let Ok(mut state) = shared_state.write() {
                    state.sim_time = sim_time;
                    state.steps_taken = sim_steps;
                    state.sim_speed = sim_speed;
                    state.positions = positions;
                }

                frame_update_time = Instant::now();
            }
        }
    });
}
