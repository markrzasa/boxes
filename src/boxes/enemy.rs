use std::time::{Instant, Duration};

use piston::{
    Position,
    Size
};

use super::player::Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EnemyState {
    Alive,
    Dead,
    Done
}

pub struct Enemy {
    aggressive: bool,
    position: Position,
    height: i32,
    width: i32,
    state: EnemyState,
    animation_index: i32,
    animation_start: Instant
}

impl Enemy {
    pub fn new(x: i32, y: i32, height: i32, width: i32, aggressive: bool) -> Self {
        Self {
            aggressive: aggressive,
            position: Position {
                x: x,
                y: y,
            },
            height: height,
            width: width,
            state: EnemyState::Alive,
            animation_index: 0,
            animation_start: Instant::now(),
        }
    }

    pub fn is_aggressive(&self) -> bool {
        self.aggressive
    }

    pub fn is_alive(&self) -> bool {
        return self.state == EnemyState::Alive;
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn dead(& mut self) {
        self.state = EnemyState::Dead;
        self.animation_index = 0;
        self.animation_start = Instant::now();
    }

    pub fn get_state(&self) -> EnemyState {
        self.state
    }

    pub fn update_dead_animation(& mut self) -> (EnemyState, i32) {
        if self.animation_start.elapsed() > Duration::from_secs(1) {
            if self.animation_index >= 3 {
                self.state = EnemyState::Done;
            } else {
                self.animation_start = Instant::now();
                self.animation_index += 1;
            }
        }
 
        (self.state, self.animation_index)
    }

    fn update_move(& mut self) -> bool {
        if self.state != EnemyState::Alive {
            return false;
        }

        if self.animation_start.elapsed() < Duration::from_millis(250) {
            return false;
        }

        self.animation_start = Instant::now();
        true
    }

    pub fn move_toward_player(& mut self, player: &Player, window_size: Size) {
        if self.update_move() {
            let mut move_rate = 1;
            if self.aggressive {
                move_rate = 2;
            }
                let player_pos = player.get_cur_position();
            if self.position.x != player_pos.x {
                if self.position.x < player_pos.x {
                    self.position.x = std::cmp::min(self.position.x + 1, window_size.width as i32 - self.width)
                } else if self.position.x > player_pos.x {
                    self.position.x = self.position.x - move_rate;
                }
            }
            if self.position.y != player_pos.y{
                if self.position.y < player_pos.y {
                    self.position.y = std::cmp::min(self.position.y + 1, window_size.height as i32 - self.height);
                } else if self.position.y > player_pos.y {
                    self.position.y = self.position.y - move_rate;
                }
            }
        }
    }
    
    pub fn move_away_from_player(& mut self, player: &Player, window_size: Size) {
        if self.update_move() {
            let player_pos = player.get_cur_position();
            if self.position.x != player_pos.x {
                if self.position.x < player_pos.x {
                    self.position.x = self.position.x - 1;
                } else if self.position.x > player_pos.x {
                    self.position.x = std::cmp::min(self.position.x + 1, window_size.width as i32 - self.width);
                }
            }
            if self.position.y != player_pos.y {
                if self.position.y < player_pos.y {
                    self.position.y = self.position.y - 1;
                } else if self.position.y > player_pos.y {
                    self.position.y = std::cmp::min(self.position.y + 1, window_size.height as i32 - self.height);
                }
            }
        }
    }
}
