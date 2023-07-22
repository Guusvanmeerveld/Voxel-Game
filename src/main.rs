#[cfg(debug_assertions)]
use env_logger;

use game::Game;
use ggez::{event, ContextBuilder};

mod constants;
mod error;
mod game;
mod location;
mod orientation;
mod world;

fn main() {
    if cfg!(debug_assertions) {
        env_logger::init();
    }

    let (ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    let game = Game::new();

    event::run(ctx, event_loop, game);
}
