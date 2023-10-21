extern crate image;
extern crate rand;

use image::{ImageBuffer, Luma};
use rand::Rng;

pub struct Point {
    x: u32,
    y: u32,
}

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

fn main() {
    let mut img = ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        if x == 0 && y == 0 {
            Luma([0u8])
        } else {
            Luma([255u8])
        }
    });

    let mut iterations: u32 = 1_000_000;

    let points: [Point; 3] = [
        Point { x: WIDTH / 2, y: 0 },
        Point { x: 0, y: HEIGHT },
        Point {
            x: WIDTH,
            y: HEIGHT,
        },
    ];

    let mut rand_point = Point {
        x: rand::thread_rng().gen_range(0..WIDTH),
        y: rand::thread_rng().gen_range(0..HEIGHT),
    };

    let pixel = img[(0, 0)];
    while iterations > 0 {
        iterations -= 1;
        let rand_index = rand::thread_rng().gen_range(0..3);
        rand_point.x = (rand_point.x + points[rand_index].x) / 2;
        rand_point.y = (rand_point.y + points[rand_index].y) / 2;
        img.put_pixel(rand_point.x, rand_point.y, pixel);
    }

    img.save("tri.png").unwrap();
}
