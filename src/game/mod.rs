use ggez::{
    event::EventHandler,
    graphics::{self, Color},
    Context, GameResult,
};
use rand::Rng;

use crate::{
    constants::{CHUNK_SIZE, WORLD_SIZE},
    world::{GeneratorConfig, World, WorldGenerator},
};

pub struct Game {
    world: World,
}

impl Game {
    pub fn new() -> Self {
        let seed: u64 = rand::thread_rng().gen();

        let config = GeneratorConfig::new(seed, WORLD_SIZE, CHUNK_SIZE);

        let mut world_gen = WorldGenerator::new(config);

        let world = world_gen.generate();

        Self { world }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        canvas.finish(ctx)
    }
}
