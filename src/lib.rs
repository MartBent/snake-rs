#![no_std]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::format;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

impl Pixel {
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
    cells: Vec<Pixel>,
    snake: Vec<(u32, u32)>,
    current_food: (u32, u32),
    current_direction: Direction,
    finished: bool,
    provider: Box<dyn SnakeProvider>,
}

impl SnakeGame {
    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn cells(&self) -> &Vec<Pixel> {
        &self.cells
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if direction == Direction::Up && self.current_direction != Direction::Down {
            self.current_direction = direction;
        }

        else if direction == Direction::Down && self.current_direction != Direction::Up {
            self.current_direction = direction;
        }

        else if direction == Direction::Left && self.current_direction != Direction::Right {
            self.current_direction = direction;
        }

        else if direction == Direction::Right && self.current_direction != Direction::Left {
            self.current_direction = direction;
        }
    }

    pub fn set_direction_unchecked(&mut self, direction: Direction) {
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

        let mut next = self.cells.clone();

        // draw the food
        let index = self.get_cell_index(self.current_food.0, self.current_food.1);
        next[index] = Pixel {
            red: 0,
            green: 255,
            blue: 0,
        };

        // clear last snake segment
        let (row, column) = self.snake.last().unwrap();
        let index = self.get_cell_index(*row, *column);
        next[index] = Pixel {
            red: 0,
            green: 0,
            blue: 0,
        };

        // set new points and verify boundaries
        let mut new_snake: Vec<(u32, u32)> = vec![];

        // move the snake
        let (row_offset, column_offset): (i32, i32) = match self.current_direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        // get the new head location
        let snake_head = self.snake.first().unwrap();
        let row_offseted = snake_head.0 as i32 + row_offset;
        let column_offseted = snake_head.1 as i32 + column_offset;

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

        for (index, &segment) in &mut self.snake.iter().enumerate() {

            // check if any segments collide with the new head
            if segment == *new_snake.first().unwrap() {
                self.finished = true;
                break;
            }

            // only add the last segment if some food was eaten
            if index == self.snake.len() - 1 {
                if snake_head == &self.current_food {
                    let msg = format!("Ate some food! new length {}", new_snake.len());
                    self.provider.debug_log(msg.as_str());

                    // keep the tail on the new snake when food was eaten
                    new_snake.push(segment);

                    // create new food
                    let mut new_food = (self.provider.provide_random_number(self.size), self.provider.provide_random_number(self.size));

                    // if the new food location is invalid, retry. This will not work properly when the snake is really long
                    while new_snake.iter().all(|segment| *segment == new_food) {
                        new_food =  (self.provider.provide_random_number(self.size), self.provider.provide_random_number(self.size));
                    }

                    self.current_food = new_food;
                    
                }
            } else {
                // concatinate the old snake with the new head
                new_snake.push(segment);
            }
        }

        // draw the new snake
        for (row, column) in &new_snake {
            let index = self.get_cell_index(*row, *column);
            next[index] = Pixel {
                red: 0,
                green: 125,
                blue: 255,
            };
        }

        if !self.finished {
            self.snake = new_snake;
            self.cells = next;
        } else {
            self.provider.debug_log("Game over! restarting...");

            // reset game
            self.current_direction = Direction::Up;

            self.snake.clear();
            self.snake.push((self.size/2,self.size/2));

            self.cells.clear();
            for _ in 0..self.size * self.size {
                self.cells.push(Pixel {
                    red: 0,
                    green: 0,
                    blue: 0,
                });
            }

            let food = (self.provider.provide_random_number(self.size), self.provider.provide_random_number(self.size));
            self.current_food = food;
            self.finished = false;
        }
    }

    pub fn new(size: u32, provider: Box<dyn SnakeProvider>) -> SnakeGame {
        let mut cells = vec![];

        for _ in 0..size * size {
            cells.push(Pixel {
                red: 0,
                green: 0,
                blue: 0,
            });
        }

        let food = (provider.provide_random_number(size), provider.provide_random_number(size));

        SnakeGame {
            size,
            cells,
            snake: vec![(size / 2, size / 2)],
            current_food: food,
            current_direction: Direction::Up,
            finished: false,
            provider
        }
    }
}