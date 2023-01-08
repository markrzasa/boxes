use piston::{
    Position,
};
use piston_window::{
    Key
};

use crate::boxes::Boxes;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stopped,
}

pub struct Player {
    cur_position: Position,
    prev_position: Position,
    cur_direction: Direction,
    prev_direction: Direction,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            cur_position: Position { x: x, y: y },
            prev_position: Position { x: x, y: y },
            cur_direction: Direction::Stopped,
            prev_direction: Direction::Stopped,            
        }
    }

    pub fn list_state(&self) {
        println!("prev dir: {:?}", self.cur_direction);
        println!("cur dir: {:?}", self.cur_direction);
        println!("cur pos: {},{}", self.cur_position.x, self.cur_position.y);
    }

    pub fn get_cur_direction(&self) -> Direction {
        self.cur_direction
    }

    pub fn get_cur_position(&self) -> Position {
        self.cur_position
    }

    pub fn get_prev_position(&self) -> Position {
        self.prev_position
    }

    pub fn clear(&mut self) {
        self.prev_position = self.cur_position;
    }

    pub fn is_moving(&self) -> bool {
        self.cur_direction != Direction::Stopped
    }

    pub fn changed_axis(&self) -> bool {
        println!("changed axis {:?} {:?}", self.prev_direction, self.cur_direction);
        let changed_axis = match self.prev_direction {
            Direction::Down | Direction::Up => {
                match self.cur_direction {
                    Direction::Left | Direction::Right => {
                        true
                    },
                    _ => { false },
                }
            },
            Direction::Left | Direction::Right => {
                match self.cur_direction {
                    Direction::Down | Direction::Up => {
                        true
                    },
                    _ => { false },
                }
            },
            _ => { false }
        };

        changed_axis
    }

    pub fn update(&self, boxes: &Boxes) -> Player
    {
        let mut player = Player{
            prev_position: self.cur_position,
            ..*self
        };

        match self.cur_direction {
            Direction::Up => player.cur_position.y -= 1,
            Direction::Down => player.cur_position.y += 1,
            Direction::Left => player.cur_position.x -= 1,
            Direction::Right => player.cur_position.x += 1,
            Direction::Stopped => {},
        }

        player.cur_position.x = player.cur_position.x.min(boxes.window_size.width as i32 - 8).max(8);
        player.cur_position.y = player.cur_position.y.min(boxes.window_size.height as i32 - 8).max(8);
        if player.cur_direction != Direction::Stopped {
            player.prev_direction = player.cur_direction;
        }

        player
    }

    pub fn button_pressed(&mut self, key: &Key) {
        match key {
            Key::Up => {self.cur_direction = Direction::Up},
            Key::Down => {self.cur_direction = Direction::Down},
            Key::Left => {self.cur_direction = Direction::Left},
            Key::Right => {self.cur_direction = Direction::Right},
            _ => {},
        }
    }

    pub fn button_released(&mut self, key: &Key) {
        match key {
            Key::Up => {
                match self.cur_direction {
                    Direction::Up => { self.cur_direction = Direction::Stopped },
                    _ => {},
                }
            },
            Key::Down => {
                match self.cur_direction {
                    Direction::Down => { self.cur_direction = Direction::Stopped },
                    _ => {},
                }
            },
            Key::Left => {
                match self.cur_direction {
                    Direction::Left => { self.cur_direction = Direction::Stopped },
                    _ => {},
                }
            },
            Key::Right => {
                match self.cur_direction {
                    Direction::Right => { self.cur_direction = Direction::Stopped },
                    _ => {},
                }
            },
            _ => {},
        }
    }
}
