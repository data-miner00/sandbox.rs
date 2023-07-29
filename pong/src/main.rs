use ggez;
use ggez::event;
use ggez::graphics;
use ggez::{ContextBuilder, GameResult};

mod ball;
mod game;
mod racket;
mod utils;

use game::Game;

fn main() -> GameResult {
    let cb = ContextBuilder::new("pong", "TanTan");
    let (mut ctx, mut event_loop) = cb.build()?;
    graphics::set_window_title(&ctx, "Rusty Pong");
    let mut state = Game::new(&mut ctx);
    event::run(&mut ctx, &mut event_loop, &mut state)
}
