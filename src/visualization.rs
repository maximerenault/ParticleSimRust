use crate::Simulation;
use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawMode, Font, Mesh, Text};
use ggez::{Context, GameResult};
use std::time::{Duration, Instant};

pub struct SimulationVisualizer {
    simulation: Simulation,
    pub simulation_time: f64,
    pub steps_taken: usize,
    last_update_time: Instant,
    frame_count: usize,
    pub fps: f64,
    fps_window: Duration,
    sim_step_duration: Duration,
    steps_per_frame: u32,
    target_frame_duration: Duration,
}

impl SimulationVisualizer {
    pub fn new(simulation: Simulation) -> Self {
        SimulationVisualizer {
            simulation,
            simulation_time: 0.0,
            steps_taken: 0,
            last_update_time: Instant::now(),
            frame_count: 0,
            fps: 0.0,
            fps_window: Duration::from_secs_f64(0.5),
            sim_step_duration: Duration::from_secs_f64(0.0),
            steps_per_frame: 0,
            target_frame_duration: Duration::from_secs_f64(1.0 / 60.0), // 60 FPS
        }
    }
}

impl EventHandler for SimulationVisualizer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let frame_start = Instant::now();
        let mut steps_this_frame = 0;

        // Simulation loop
        let mut remaining_time = self.target_frame_duration;
        loop {
            let step_start = Instant::now();
            self.simulation.simulation_step();
            self.simulation_time += self.simulation.dt;
            self.steps_taken += 1;
            steps_this_frame += 1;
            let step_duration = step_start.elapsed();

            remaining_time = remaining_time.saturating_sub(step_duration);
            if remaining_time < self.sim_step_duration
                || (steps_this_frame > 1 && remaining_time.is_zero())
            {
                break;
            }
        }

        self.sim_step_duration = frame_start.elapsed() / steps_this_frame;
        self.steps_per_frame = steps_this_frame;

        // FPS calculation
        self.frame_count += 1;
        let now = Instant::now();
        if now.duration_since(self.last_update_time) >= self.fps_window {
            self.fps = self.frame_count as f64 / self.fps_window.as_secs_f64();
            self.frame_count = 0;
            self.last_update_time = now;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        // Draw particles
        let positions = self.simulation.get_particle_positions();
        for position in &positions {
            let circle =
                Mesh::new_circle(ctx, DrawMode::fill(), *position, 2.0, 0.1, Color::WHITE)?;
            graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        }

        // Draw simulation stats
        let display_text = format!(
            "Sim Time: {:.2} s\nSteps: {}\nFPS: {:.1}\nStep Time: {:.2} ms\nSteps per frame: {}",
            self.simulation_time,
            self.steps_taken,
            self.fps,
            self.sim_step_duration.as_secs_f64() * 1000.0,
            self.steps_per_frame
        );
        let text = Text::new((display_text, Font::default(), 20.0));
        graphics::draw(ctx, &text, ([10.0, 10.0],))?;

        graphics::present(ctx)?;
        Ok(())
    }
}
