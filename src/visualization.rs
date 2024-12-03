use ggez::{Context, GameResult};
use ggez::event::{EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh};
use crate::Simulation;

pub struct SimulationVisualizer {
    simulation: Simulation,
}

impl SimulationVisualizer {
    pub fn new(simulation: Simulation) -> Self {
        SimulationVisualizer { simulation }
    }
}

impl EventHandler for SimulationVisualizer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.simulation.simulation_step();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        let positions = self.simulation.get_particle_positions();

        for position in &positions {
            let circle = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                *position,
                2.0,
                0.1,
                Color::WHITE,
            )?;
            graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}
