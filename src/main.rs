pub mod boxes;

extern crate image;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;
extern crate sprite;

use boxes::Boxes;


fn main() {
    let mut boxes = Boxes::new(800, 800);
    boxes.run();
}
