use crate::simstate::SimState;
use ggez::graphics::{self, Color, DrawMode, Font, Text};
use ggez::{Context, GameResult};
use std::sync::{Arc, RwLock};

pub struct SimulationVisualizer {
    shared_state: Arc<RwLock<SimState>>,
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
            my_state,
        }
    }
}

impl ggez::event::EventHandler for SimulationVisualizer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if let Ok(state) = self.shared_state.read() {
            self.my_state = state.clone()
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        // Draw particles as a batch of circles
        let mut mesh_builder = graphics::MeshBuilder::new();
        for position in &self.my_state.positions {
            let _ = mesh_builder.circle(DrawMode::fill(), *position, 1.0, 0.1, Color::WHITE);
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
            ggez::timer::fps(ctx),
        );
        let text = Text::new((display_text, Font::default(), 20.0));
        graphics::draw(ctx, &text, ([10.0, 10.0],))?;

        graphics::present(ctx)?;
        Ok(())
    }
}
