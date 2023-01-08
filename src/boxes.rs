mod enemy;
mod player;

use std::rc::Rc;
use std::vec;

use graphics::Transformed;
use ::image::{ImageFormat};
use opengl_graphics::{
    OpenGL
};
use piston::{Position};
use piston::input::*;
use piston::window::{WindowSettings};
use piston_window::{
    clear,
    color,
    text,
    Glyphs,
    PistonWindow,
    Size,
    Texture,
    TextureContext,
    TextureSettings
};
use rand::distributions::{Distribution, Uniform};
use sprite::*;

use enemy::{
    Enemy,
    EnemyState
};
use player::{Player};

use self::player::Direction;

const SPRITE_WIDTH: i32 = 32;
const START_X: i32 = SPRITE_WIDTH / 2;
const START_Y: i32 = SPRITE_WIDTH / 2;
const MAX_LINE_LEN: f64 = 300.0;
const WARN_LINE_LEN: f64 = MAX_LINE_LEN * 0.75;

enum GameState {
    Playing,
    LevelComplete,
    NextLevel
}

pub struct Line {
    to: Position,
    from: Position
}

impl Line {
    pub fn len(&self) -> f64 {
        let x_delta = (self.from.x - self.to.x).abs();
        let y_delta = (self.from.y - self.to.y).abs();
        ((x_delta.pow(2) + y_delta.pow(2)) as f64).sqrt()
    }

    fn on_segment(&self, p: Position, q: Position, r: Position) -> bool {
        if (q.x <= std::cmp::max(p.x, r.x)) && (q.x >= std::cmp::min(p.x, r.x)) && 
        (q.y <= std::cmp::max(p.y, r.y)) && (q.y >= std::cmp::min(p.y, r.y)) {
            return true;
        }

        false
    }

    fn orientation(&self, p: Position, q: Position, r: Position) -> i32 {
        let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
        if val == 0 {
            0
        } else if val > 0 {
            1
        } else {
            2
        }
    }

    pub fn intersects(&self, line: &Line) -> bool {
        let o1 = self.orientation(self.from, self.to, line.from);
        let o2 = self.orientation(self.from, self.to, line.to);
        let o3 = self.orientation(line.from, line.to, self.from);
        let o4 = self.orientation(line.from, line.to, self.to);

        if o1 != o2 && o3 != o4 {
            return true;
        }

        if o1 == 0 && self.on_segment(self.from, line.to, self.to) {
            return true;
        }

        if o2 == 0 && self.on_segment(self.from, line.to, self.to) {
            return true;
        }

        if o3 == 0 && self.on_segment(line.from, self.from, line.to) {
            return true;
        }

        if o4 == 0 && self.on_segment(line.from, self.to, line.to) {
            return true;
        }
      
        false
    }
}

pub struct Boxes {
    window_size: Size,
    lines: Vec<Line>,
    player: Player,
    enemies: Vec<Enemy>,
    state: GameState
}

impl Boxes {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            window_size: Size::from([width, height]),
            lines: vec![
                Line{
                    to: Position { x: START_X, y: START_Y },
                    from: Position { x: START_X, y: START_Y },
                }
            ],
            enemies: vec![
                Enemy::new(100, height as i32 - SPRITE_WIDTH, SPRITE_WIDTH, SPRITE_WIDTH),
                Enemy::new(300, height as i32 - SPRITE_WIDTH, SPRITE_WIDTH, SPRITE_WIDTH)
            ],
            player: Player::new(START_X, START_Y),
            state: GameState::Playing
        }
    }

    fn clear_lines(&mut self) {
        self.lines.clear();
        self.player.clear();
        self.lines.push(Line { to: self.player.get_cur_position(), from: self.player.get_prev_position() });
    }

    fn list_state(&mut self) {
        println!("====================");
        self.player.list_state();
        println!("lines:");
        for l in self.lines.iter() {
            println!("  {},{} {},{}", l.from.x, l.from.y, l.to.x, l.to.y);
        }
        println!("====================");
    }

    fn update(&mut self) {
        if self.player.is_moving() {
            if let Some(l) = self.lines.last_mut() {
                if l.len() > MAX_LINE_LEN {
                    self.clear_lines();
                } else {
                    if self.player.changed_axis() {
                        self.lines.push(Line{
                            to: self.player.get_cur_position(),
                            from: self.player.get_prev_position()
                        })
                    } else {
                        l.to = self.player.get_cur_position();
                    }
                }
            }
        }

        if self.lines.len() >= 4 {
            if let Some(last_line) = self.lines.last() {
                if let Some(first_line) = self.lines.first() {
                    if last_line.intersects(first_line) {
                        let x_coords = vec!(
                            first_line.from.x,
                            first_line.to.x,
                            last_line.from.x,
                            last_line.to.x
                        );
                        let y_coords = vec!(
                            first_line.from.y,
                            first_line.to.y,
                            last_line.from.y,
                            last_line.to.y
                        );
                        if let Some(start_x) = x_coords.iter().min() {
                            if let Some(start_y) = y_coords.iter().min() {
                                if let Some(end_x) = x_coords.iter().max() {
                                    if let Some(end_y) = y_coords.iter().max() {
                                        for e in self.enemies.iter_mut() {
                                            let e_pos = e.get_position();
                                            if (*start_x <= e_pos.x) && (e_pos.x <= *end_x) && (*start_y <= e_pos.y) && (e_pos.y <= *end_y) {
                                                e.dead();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self.clear_lines();
                    }
                }
            }
        }

        if self.lines.len() > 4 {
            self.lines.remove(0);
        }

        let mut range = rand::thread_rng();
        let attack = Uniform::from(0..3);
        for e in self.enemies.iter_mut() {
            match attack.sample(&mut range) {
                0 => {e.move_away_from_player(&self.player, self.window_size)},
                1 => {},
                _ => {e.move_toward_player(&self.player, self.window_size)},
            }
        }

        self.list_state();
    }

    pub fn run(&mut self) {
        let opengl = OpenGL::V3_2;

        let mut window: PistonWindow = WindowSettings::new("Boxes", self.window_size)
            .exit_on_esc(true)
            .graphics_api(opengl)
            .build()
            .unwrap();
    
        let factory = window.factory.clone();
        let mut texture_context = TextureContext {
            factory: factory,
            encoder: window.factory.create_command_buffer().into()
        };
    
        let mut glyphs = Glyphs::from_bytes(
            include_bytes!("../fonts/PressStart2PRegular.ttf"),
            window.create_texture_context(),
            TextureSettings::new(),
        ).unwrap();

        let image_buffer = match ::image::load_from_memory_with_format(include_bytes!("../images/hero.png"), ImageFormat::Png) {
            Ok(img) => img,
            Err(_) => panic!("failed to load hero sprite"),
        };
    
        let player_texture = Rc::new(Texture::from_image(
            &mut texture_context,
            &image_buffer.to_rgba8(),
            &TextureSettings::new()
        ).unwrap());
    
        let mut player_sprite = Sprite::from_texture(player_texture);

        let image_buffer = match ::image::load_from_memory_with_format(include_bytes!("../images/enemy.png"), ImageFormat::Png) {
            Ok(img) => img,
            Err(_) => panic!("failed to load enemy sprite"),
        };
        let enemy_texture = Rc::new(Texture::from_image(
            &mut texture_context,
            &image_buffer.to_rgba8(),
            &TextureSettings::new()
        ).unwrap());

        let mut enemy_sprites = self.enemies.iter().map(|_| Sprite::from_texture(enemy_texture.to_owned())).collect::<Vec<_>>();

        while let Some(e) = window.next() {
            match self.state {
                GameState::LevelComplete => {
                    window.draw_2d(&e, |c, g, _| {
                        clear(color::BLACK, g);
                        text::Text::new_color(color::YELLOW, 14).draw(
                            "Level Complete!",
                            & mut glyphs,
                            &c.draw_state,
                            c.transform.trans(50.0, 50.0),
                            g
                        ).unwrap();
                    });
                },
                GameState::NextLevel => {},
                GameState::Playing => {
                    self.player = self.player.update(self);
                    if let Some(Button::Keyboard(key)) = e.press_args() {
                        self.player.button_pressed(&key);
                    };
                    if let Some(Button::Keyboard(key)) = e.release_args() {
                        self.player.button_released(&key);
                    };
                    self.update();
        
                    window.draw_2d(&e, |c, g, _| {
                        clear(color::GRAY, g);
                        for (i, l) in self.lines.iter().enumerate() {
                            let mut color = color::RED;
                            if (i == (self.lines.len() - 1)) && (l.len() >= WARN_LINE_LEN) {
                                color = color::YELLOW;
                            }
                            piston_window::line_from_to(
                                color,
                                3.0,
                                [l.from.x as f64, l.from.y as f64],
                                [l.to.x as f64, l.to.y as f64],
                                c.transform,
                                g
                            );
                        }
        
                        let mut enemies_to_remove: Vec<usize> = vec![];
                        for (i, e) in enemy_sprites.iter_mut().enumerate() {
                            if let Some(enemy) = self.enemies.get_mut(i) {
                                e.set_position(enemy.get_position().x as f64, enemy.get_position().y as f64);
                                match enemy.get_state() {
                                    EnemyState::Alive => {e.set_src_rect([0.0, 0.0, SPRITE_WIDTH as f64, SPRITE_WIDTH as f64])},
                                    EnemyState::Dead => {
                                        let (state, ai) = enemy.update_dead_animation();
                                        match state {
                                            EnemyState::Dead => {
                                                e.set_src_rect([SPRITE_WIDTH as f64 * (ai as f64 + 1.0), 0.0, SPRITE_WIDTH as f64, SPRITE_WIDTH as f64]);
                                            },
                                            _ => {enemies_to_remove.push(i);}
                                        }
                                    },
                                    EnemyState::Done => {enemies_to_remove.push(i)},
                                }
                                if enemy.get_state() != EnemyState::Done {
                                    e.draw(c.transform, g);
                                }
                            }
                        }
        
                        if enemies_to_remove.len() > 0 {
                            for i in enemies_to_remove.iter().rev() {
                                enemy_sprites.remove(*i);
                            }
                        }
        
                        player_sprite.set_position(self.player.get_cur_position().x as f64, self.player.get_cur_position().y as f64);
                        match self.player.get_cur_direction() {
                            Direction::Down => {player_sprite.set_rotation(90.0)},
                            Direction::Left => {player_sprite.set_rotation(180.0)},
                            Direction::Right => {player_sprite.set_rotation(0.0)},
                            Direction::Up => {player_sprite.set_rotation(270.0)},
                            _ => {}
                        }
                        player_sprite.draw(c.transform, g);
                    });

                    if enemy_sprites.len() <= 0 {
                        self.enemies.clear();
                        self.state = GameState::LevelComplete;
                    }
                },
                _ => {},
            }
        }
    }
}
