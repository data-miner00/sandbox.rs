use crate::render;

use piston_window::types::Color;
use piston_window::{Context, G2d};
use render::render_square;
use std::collections::LinkedList;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match *self {
            | Self::Up => Self::Down,
            | Self::Down => Self::Up,
            | Self::Left => Self::Right,
            | Self::Right => Self::Right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    x: i32,
    y: i32,
}

impl Block {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        let mut body: LinkedList<Block> = LinkedList::new();

        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Self {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn render(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            render_square(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_pos(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn head_dir(&self) -> Direction {
        self.direction
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        if let Some(direction) = dir {
            self.direction = direction;
        }

        let (prev_x, prev_y) = self.head_pos();

        /*  (0, 0) origin           (+x, 0)
         *   ----------------------->
         *   |                      |
         *   |      2D Plane        |
         *   |                      |
         *   v-----------------------
         *  (0, +y)                 (+x, +y)
         */
        let new_block = match self.direction {
            | Direction::Up => Block {
                x: prev_x,
                y: prev_y - 1,
            },
            | Direction::Down => Block {
                x: prev_x,
                y: prev_y + 1,
            },
            | Direction::Left => Block {
                x: prev_x - 1,
                y: prev_y,
            },
            | Direction::Right => Block {
                x: prev_x + 1,
                y: prev_y,
            },
        };

        self.body.push_front(new_block);

        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (prev_x, prev_y) = self.head_pos();

        let mut moving_dir = self.direction;

        if let Some(direction) = dir {
            moving_dir = direction;
        }

        match moving_dir {
            | Direction::Up => (prev_x, prev_y - 1),
            | Direction::Down => (prev_x, prev_y + 1),
            | Direction::Left => (prev_x - 1, prev_y),
            | Direction::Right => (prev_x + 1, prev_y),
        }
    }

    pub fn restore_tail(&mut self) {
        let block = self.tail.clone().unwrap();
        self.body.push_back(block);
    }

    pub fn check_tail_overlap(&self, x: i32, y: i32) -> bool {
        let mut counter = 0;

        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            counter += 1;

            if counter == self.body.len() - 1 {
                break;
            }
        }

        false
    }
}
