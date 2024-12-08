use crate::simstate::SimState;
use ggez::graphics::{self, Color, DrawMode, Font, Text};
use ggez::{Context, GameResult};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

pub struct SimulationVisualizer {
    shared_state: Arc<RwLock<SimState>>,
    last_update_time: Instant,
    frame_count: usize,
    fps: f64,
    fps_window: Duration,
    my_state: SimState,
}

impl SimulationVisualizer {
    pub fn new(shared_state: Arc<RwLock<SimState>>) -> Self {
        let my_state;
        {
            let shared_state = shared_state.read().unwrap();
            my_state = shared_state.clone()
        }
        SimulationVisualizer {
            shared_state,
            last_update_time: Instant::now(),
            frame_count: 0,
            fps: 0.0,
            fps_window: Duration::from_secs_f64(0.5),
            my_state,
        }
    }
}

impl ggez::event::EventHandler for SimulationVisualizer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if let Ok(state) = self.shared_state.read() {
            self.my_state = state.clone()
        }

        // FPS calculation
        self.frame_count += 1;
        if self.last_update_time.elapsed() >= self.fps_window {
            self.fps = self.frame_count as f64 / self.fps_window.as_secs_f64();
            self.frame_count = 0;
            self.last_update_time = Instant::now();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        // Draw particles as a batch of circles
        let mut mesh_builder = graphics::MeshBuilder::new();
        for position in &self.my_state.positions {
            let _ = mesh_builder.circle(DrawMode::fill(), *position, 2.0, 0.1, Color::WHITE);
        }
        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        // Display simulation stats
        let display_text = format!(
            "Real Time: {:.1} s\nSim Time: {:.1} s\nSim Speed: {:.2}\nSteps: {}\nFPS: {:.1}",
            self.my_state.start_time.elapsed().as_secs_f64(),
            self.my_state.sim_time,
            self.my_state.sim_speed,
            self.my_state.steps_taken,
            self.fps
        );
        let text = Text::new((display_text, Font::default(), 20.0));
        graphics::draw(ctx, &text, ([10.0, 10.0],))?;

        graphics::present(ctx)?;
        Ok(())
    }
}
