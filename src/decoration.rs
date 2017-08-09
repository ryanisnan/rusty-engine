use ggez::graphics::Image;
use std::rc::Rc;

use entity::Entity;

#[derive(Debug)]
pub struct DecorationPrototype {
    pub image: Rc<Image>
}

impl Entity for DecorationPrototype {}

#[derive(Debug)]
pub struct Decoration {
    pub meta: Rc<DecorationPrototype>,
}
