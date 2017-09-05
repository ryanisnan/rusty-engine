pub mod decoration;

pub trait Entity {
    /*
        Defines behaviour of an object that can live in the world.
    */
    fn is_moveable(&self) -> bool { false }
    fn is_visible(&self) -> bool { true }

    fn bind_camera(&self);
    fn unbind_camera(&self);
}
