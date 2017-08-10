use ggez::graphics::Image;
use std::collections::HashMap;
use std::rc::Rc;

use decoration::Decoration;

#[derive(Debug)]
pub struct TilePrototype {
    // Defines data common across various types of tiles (Flyweight Pattern)
    pub image: Image,
    pub is_walkable: bool
}

#[derive(Debug)]
pub struct Tile {
    // Represents a game tile in the world
    pub meta: Rc<TilePrototype>,
    pub decorations: Option<Vec<Decoration>>,
}

#[derive(Debug)]
pub struct TileLibrary {
    // Represents a library of different tile types
    pub tiles: HashMap<String, Rc<TilePrototype>>
}

impl TileLibrary {
    pub fn new() -> Self {
        TileLibrary {
            tiles: HashMap::new(),
        }
    }

    pub fn load(&mut self, tile_id: &str, tile_type: TilePrototype) {
        self.tiles.insert(String::from(tile_id), Rc::new(tile_type));
    }
}
