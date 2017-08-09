use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

// Trait that all world entities must implement
pub trait Entity {}

// Data structure that holds prototypes for the various entities used in the game
pub struct EntityLibrary {
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
