use core::ops::Add;
use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0;

pub fn to_actual_coord<T: Add<Output = T> + Copy + From<f64>>(coord: i32) -> T {
    T::from((coord as f64) * BLOCK_SIZE)
}

pub fn render_square(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let x = to_actual_coord::<f64>(x);
    let y = to_actual_coord::<f64>(y);

    rectangle(color, [x, y, BLOCK_SIZE, BLOCK_SIZE], con.transform, g);
}

pub fn render_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_actual_coord::<f64>(x);
    let y = to_actual_coord::<f64>(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}
