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
        }
    }
}

impl EventHandler for SimulationVisualizer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.simulation.simulation_step();

        self.simulation_time += self.simulation.dt;
        self.steps_taken += 1;
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

        let positions = self.simulation.get_particle_positions();

        for position in &positions {
            let circle =
                Mesh::new_circle(ctx, DrawMode::fill(), *position, 2.0, 0.1, Color::WHITE)?;
            graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        }

        let display_text = format!(
            "Sim Time: {:.2} s\nSteps: {}\nFPS: {:.1}",
            self.simulation_time, self.steps_taken, self.fps
        );
        let text = Text::new((display_text, Font::default(), 20.0));
        graphics::draw(ctx, &text, ([10.0, 10.0],))?;

        graphics::present(ctx)?;
        Ok(())
    }
}
