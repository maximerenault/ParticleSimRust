use std::time::Instant;

pub struct SimState {
    pub positions: Vec<[f32; 2]>,
    pub start_time: Instant,
    pub sim_time: f64,
    pub sim_speed: f64,
    pub steps_taken: usize,
}

impl Clone for SimState {
    fn clone(&self) -> Self {
        SimState {
            positions: self.positions.clone(),
            start_time: self.start_time,
            sim_time: self.sim_time,
            sim_speed: self.sim_speed,
            steps_taken: self.steps_taken,
        }
    }
}

impl SimState {
    pub fn new(particle_count: usize) -> Self {
        SimState {
            positions: vec![[0.0, 0.0]; particle_count],
            start_time: Instant::now(),
            sim_time: 0.0,
            sim_speed: 0.0,
            steps_taken: 0,
        }
    }
}
