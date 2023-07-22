use piston_window::*;
use piston_window::{types::Color, PistonWindow, WindowSettings};
use std::error::Error;

mod game;
mod render;
mod snake;

use game::Game;
use render::to_actual_coord;

const TITLE: &str = "Snake";
const BG_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() -> Result<(), Box<dyn Error>> {
    let (width, height) = (20, 20);

    let mut window: PistonWindow = WindowSettings::new(
        TITLE,
        [
            to_actual_coord::<f64>(width) as u32,
            to_actual_coord::<f64>(height) as u32,
        ],
    )
    .exit_on_esc(true)
    .build()?;

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.on_key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BG_COLOR, g);
            game.render(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }

    Ok(())
}
