use piston::Position;
use piston_window::Key;

use crate::boxes::Boxes;

use super::enemy::Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stopped,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerState {
    Alive,
    Dead
}

pub struct Player {
    cur_position: Position,
    prev_position: Position,
    cur_direction: Direction,
    prev_direction: Direction,
    state: PlayerState,
    height: i32,
    width: i32,
}

impl Player {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            cur_position: Position { x, y },
            prev_position: Position { x, y },
            cur_direction: Direction::Stopped,
            prev_direction: Direction::Stopped,  
            state: PlayerState::Alive,
            height: height,
            width: width,
        }
    }

    pub fn reset(&mut self, x: i32, y: i32) {
        self.cur_position = Position { x, y };
        self.prev_position = self.cur_position;
        self.state = PlayerState::Alive;
    }

    pub fn dead(&mut self) {
        self.state = PlayerState::Dead;
    }

    pub fn is_dead(&mut self) -> bool {
        self.state == PlayerState::Dead
    }

    pub fn collided(&mut self, enemy: &Enemy) -> bool {
        let e1 = enemy.get_position();
        let e2 = Position{
            x: e1.x + enemy.get_width(),
            y: e1.y - enemy.get_height(),
        };
        let p1 = Position{
            x: self.cur_position.x + (self.width / 4),
            y: self.cur_position.y - (self.height / 4),
        };
        let p2 = Position{
            x: (self.cur_position.x + self.width) - (self.width / 4),
            y: (self.cur_position.y - self.height) + (self.height / 4),
        };
        if (p1.x >= e1.x && p1.x <= e2.x) || (p2.x >= e1.x && p2.x <= e2.x) {
            if (p1.y <= e1.y && p1.y >= e2.y) || (p2.y <= e1.y && p2.y >= e2.y) {
                println!("====================");
                println!("player {} {} {} {}", p1.x, p1.y, p2.x, p2.y);
                println!("enemy  {} {} {} {}", e1.x, e1.y, e2.x, e2.y);
                println!("====================");
                return true;
            }
        }

        false
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
