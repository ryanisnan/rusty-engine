use ggez::graphics::Image;
use std::collections::HashMap;
use std::rc::Rc;

pub trait Entity {} // Things that can be placed into the world will all implement this functionality


#[derive(Debug)]
pub struct DecorationType {
    pub image: Rc<Image>
}

#[derive(Debug)]
pub struct DecorationLibrary {
    pub decorations: HashMap<String, Rc<DecorationType>>
}

impl DecorationLibrary {
    pub fn new() -> Self {
        DecorationLibrary {
            decorations: HashMap::new(),
        }
    }

    pub fn load(&mut self, decoration_id: &str, decoration_type: DecorationType) {
        self.decorations.insert(String::from(decoration_id), Rc::new(decoration_type));
    }
}

#[derive(Debug)]
pub struct Decoration {
    pub meta: Rc<DecorationType>,
}
