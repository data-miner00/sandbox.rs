use crate::render::{render_rectangle, render_square};
use crate::snake::{Block, Direction, Snake};

use piston_window::types::Color;
use piston_window::*;
use rand::{thread_rng, Rng};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 1.0];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_DELAY: f64 = 1.0;

pub struct Game {
    snake: Snake,

    is_food_exist: bool,
    food_block: Block,

    width: i32,
    height: i32,

    is_game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            is_food_exist: true,
            food_block: Block::new(6, 4),
            width,
            height,
            is_game_over: false,
        }
    }

    pub fn on_key_pressed(&mut self, key: Key) {
        if self.is_game_over {
            return;
        }

        let dir = match key {
            | Key::Up => Direction::Up,
            | Key::Down => Direction::Down,
            | Key::Left => Direction::Left,
            | Key::Right => Direction::Right,
            | _ => self.snake.head_dir(),
        };

        // Do nothing if the key is opposite
        if dir == self.snake.head_dir().opposite() {
            return;
        }

        self.update_snake(Some(dir));
    }

    pub fn render(&self, con: &Context, g: &mut G2d) {
        self.snake.render(con, g);

        if self.is_food_exist {
            render_square(
                FOOD_COLOR,
                self.food_block.get_x(),
                self.food_block.get_y(),
                con,
                g,
            );
        }

        render_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        render_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        render_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        render_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.is_game_over {
            render_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.is_game_over {
            if self.waiting_time > RESTART_DELAY {
                self.restart();
            }
            return;
        }

        if !self.is_food_exist {
            self.spawn_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.is_food_exist = true;
        self.food_block = Block::new(6, 4);
        self.is_game_over = false;
    }

    fn generate_random_coords(&self) -> (i32, i32) {
        let mut rng = thread_rng();
        (
            rng.gen_range(1..self.width - 1),
            rng.gen_range(1..self.height - 1),
        )
    }

    fn spawn_food(&mut self) {
        let (mut rand_x, mut rand_y) = self.generate_random_coords();

        while self.snake.check_tail_overlap(rand_x, rand_y) {
            (rand_x, rand_y) = self.generate_random_coords();
        }

        self.food_block = Block::new(rand_x, rand_y);

        self.is_food_exist = true;
    }

    fn check_is_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.check_tail_overlap(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.width - 1
    }

    fn check_is_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_pos();

        let is_head_pos_overlap_food =
            self.food_block.get_x() == head_x && self.food_block.get_y() == head_y;
        if self.is_food_exist && is_head_pos_overlap_food {
            self.is_food_exist = false;
            self.snake.restore_tail();
        }
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_is_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_is_eating();
        } else {
            self.is_game_over = true;
        }

        self.waiting_time = 0.0;
    }
}
