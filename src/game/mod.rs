use ggez::{
    event::EventHandler,
    graphics::{self, Color},
    Context, GameResult,
};
use rand::Rng;

use crate::world::World;

pub struct Game {
    world: World,
}

impl Game {
    pub fn new() -> Self {
        let seed: u64 = rand::thread_rng().gen();

        let world = World::new(seed);

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
