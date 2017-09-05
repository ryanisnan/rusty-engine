use ggez::graphics::Image;
use std::collections::HashMap;
use std::rc::Rc;

use entity::Entity;

#[derive(Debug)]
pub struct DecorationPrototype {
    pub image: Image
}

#[derive(Debug)]
pub struct DecorationLibrary {
    // Represents a library of different decoration types
    pub decorations: HashMap<String, Rc<DecorationPrototype>>
}

impl DecorationLibrary {
    pub fn new() -> Self {
        DecorationLibrary {
            decorations: HashMap::new(),
        }
    }

    pub fn load(&mut self, decoration_id: &str, decoration_type: DecorationPrototype) {
        self.decorations.insert(String::from(decoration_id), Rc::new(decoration_type));
    }
}

#[derive(Debug)]
pub struct Decoration {
    pub meta: Rc<DecorationPrototype>,
}

impl Entity for Decoration {
    fn is_moveable(&self) -> bool { false }
    fn bind_camera(&self) {}
    fn unbind_camera(&self) {}
}
