#![no_std]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::format;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    red: u8,
    green: u8,
    blue: u8,
}

impl Cell {
    pub fn red(&self) -> u8 {
        self.red
    }
    pub fn green(&self) -> u8 {
        self.green
    }
    pub fn blue(&self) -> u8 {
        self.blue
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from(value: u8) -> Self {
        match value {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

pub trait SnakeProvider {
    // should return a random number between 0 and size
    fn provide_random_number(&self, size: u32) -> u32;
    // used for debugging
    fn debug_log(&self, message: &str);
}

pub struct SnakeGame {
    size: u32,
    cells: Vec<Cell>,
    snake: Vec<(u32, u32)>,
    current_food: (u32, u32),
    current_direction: Direction,
    provider: Box<dyn SnakeProvider>,
}

impl SnakeGame {
    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.current_direction = direction;
    }

    pub fn get_pixel_buffer_index(&self, row: u32, column: u32) -> u32 {
        // times 3 sinces a cell contains 3 bytes
        (row * self.size + column) * 3
    }

    fn get_cell_index(&self, row: u32, column: u32) -> usize {
        (row * self.size + column) as usize
    }

    pub fn tick(&mut self) {

        let msg = format!("Random: {}", self.provider.provide_random_number(32));
        self.provider.debug_log(msg.as_str());

        let mut next = self.cells.clone();

        // draw the food
        let index = self.get_cell_index(self.current_food.0, self.current_food.1);
        next[index] = Cell {
            red: 0,
            green: 255,
            blue: 0,
        };

        // clear last snake segment
        let (row, column) = self.snake.last().unwrap();
        let index = self.get_cell_index(*row, *column);
        next[index] = Cell {
            red: 0,
            green: 0,
            blue: 0,
        };

        // move the snake
        let current_direction = self.current_direction;

        let (row_offset, column_offset): (i32, i32) = match current_direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        // set new points and verify boundaries
        let mut new_snake: Vec<(u32, u32)> = vec![];
        let mut new_segment: Option<(u32, u32)> = None;

        for (index, &(row, column)) in &mut self.snake.iter().enumerate() {
            if index == 0 {
                if row == self.current_food.0 && column == self.current_food.1 {
                    new_segment = Some(*self.snake.last().unwrap());
                }
            }

            let row_offseted = row as i32 + row_offset;
            let column_offseted = column as i32 + column_offset;

            let new_row: u32;
            let new_column: u32;

            if row_offseted == self.size as i32 {
                new_row = 0;
            } else if row_offseted == -1 {
                new_row = self.size - 1;
            } else {
                new_row = row_offseted as u32;
            }

            if column_offseted == self.size as i32 {
                new_column = 0;
            } else if column_offseted == -1 {
                new_column = self.size - 1;
            } else {
                new_column = column_offseted as u32;
            }

            new_snake.push((new_row, new_column));
        }

        if let Some(segment) = new_segment {
            let msg = format!("Ate some food! new length {}", self.snake.len());
            self.provider.debug_log(msg.as_str());
            self.snake.push(segment);
        }

        // draw the snake
        for (row, column) in &self.snake {
            let index = self.get_cell_index(*row, *column);
            next[index] = Cell {
                red: 0,
                green: 125,
                blue: 255,
            };
        }

        self.snake = new_snake;
        self.cells = next;
    }

    pub fn new(size: u32, provider: Box<dyn SnakeProvider>) -> SnakeGame {
        let mut cells = vec![];

        for _ in 0..size * size {
            cells.push(Cell {
                red: 0,
                green: 0,
                blue: 0,
            });
        }

        SnakeGame {
            size,
            cells,
            snake: vec![(size / 2, size / 2)],
            current_food: (0, 0),
            current_direction: Direction::Up,
            provider
        }
    }
}
