use ggez::graphics::Image;
use std::any::Any;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::rc::Rc;

pub trait Entity {} // Things that can be placed into the world will all implement this functionality


#[derive(Debug)]
pub struct DecorationType {
    pub image: Rc<Image>
}

#[derive(Debug)]
pub struct Decoration {
    pub meta: Rc<DecorationType>,
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






pub struct EntityLibrary {
    /*
        Holds a hash of types of entities and the various entities corresponding
        to each type.
    */
    entities: HashMap<String, Box<Any>>,
}

impl EntityLibrary {
    pub fn new() -> Self {
        EntityLibrary {
            entities: HashMap::new()
        }
    }

    pub fn insert<T: Entity + 'static>(&mut self, entity_id: &str, entity: T) -> () {
        self.entities.insert(entity_id.into(), Box::new(Rc::new(entity)));
    }

    pub fn get<T: Entity + 'static>(&self, entity_id: &str) -> Rc<T> {
        self.entities[entity_id].downcast_ref::<Rc<T>>().unwrap().clone()
    }
}
