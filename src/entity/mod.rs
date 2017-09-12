pub mod decoration;

use std::cell::RefCell;

use ggez::graphics::Point;


pub trait Entity {
    /*
        Defines behaviour of an object that can live in the world.
    */
    fn is_moveable(&self) -> bool { false }
    fn is_visible(&self) -> bool { true }

    fn get_point(&mut self) -> &mut Point;
}
