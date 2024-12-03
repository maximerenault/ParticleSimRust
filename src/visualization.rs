use ggez::{Context, GameResult};
use ggez::event::{EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh};

use crate::particle::Particle;
use crate::simulation::simulation_step;

pub struct SimulationVisualizer {
    pub particles: Vec<Particle>,
    pub total_forces: Vec<[f64; 2]>,
    pub dt: f64,
}

impl SimulationVisualizer {
    pub fn new(particles: Vec<Particle>, dt: f64) -> Self {
        let total_forces : Vec<[f64; 2]> = vec![[0.0, 0.0]; particles.len()];
        SimulationVisualizer { particles, total_forces, dt }
    }
}

impl EventHandler for SimulationVisualizer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        simulation_step(&mut self.particles, &mut self.total_forces, self.dt);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        for particle in &self.particles {
            let circle = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                [particle.position[0] as f32, particle.position[1] as f32],
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
