#[allow(dead_code)]
use crate::game::PLAYER_SPEED;
use crate::utils::clamp;
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode};
use ggez::nalgebra as na;
use ggez::timer;
use ggez::Context;

pub const RACKET_HEIGHT: f32 = 100.0;
pub const RACKET_WIDTH: f32 = 20.0;
pub const RACKET_HEIGHT_HALF: f32 = RACKET_WIDTH * 0.5;
pub const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;

struct Racket;

pub fn move_racket(pos: &mut na::Point2<f32>, keycode: KeyCode, y_dir: f32, ctx: &mut Context) {
    let dt = timer::delta(ctx).as_secs_f32();
    let screen_h = graphics::drawable_size(ctx).1;
    if keyboard::is_key_pressed(ctx, keycode) {
        pos.y += y_dir * PLAYER_SPEED * dt;
    }
    clamp(
        &mut pos.y,
        RACKET_HEIGHT_HALF,
        screen_h - RACKET_HEIGHT_HALF,
    );
}
