use game::Game;
use ggez::{event, ContextBuilder};

mod game;
mod location;
mod world;

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    let game = Game::new();

    event::run(ctx, event_loop, game);
}
