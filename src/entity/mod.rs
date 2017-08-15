use ggez::graphics::Point;

pub mod decoration;

pub trait Entity {
    /*
        Defines behaviour of an object that can live in the world.
    */
    fn is_moveable(&self) -> bool;
    fn get_point(&self) -> &Point;
    fn bind_camera(&self);
    fn unbind_camera(&self);
}
