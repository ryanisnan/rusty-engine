use ggez::graphics::Image;
use std::any::Any;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug)]
pub struct DecorationPrototype {
    pub image: Rc<Image>
}

#[derive(Debug)]
pub struct Decoration {
    pub meta: Rc<DecorationPrototype>,
}
