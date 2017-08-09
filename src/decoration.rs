use ggez::graphics::Image;
use std::rc::Rc;

#[derive(Debug)]
pub struct DecorationPrototype {
    pub image: Rc<Image>
}

#[derive(Debug)]
pub struct Decoration {
    pub meta: Rc<DecorationPrototype>,
}
