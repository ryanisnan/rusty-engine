extern crate ggez;

use assets::AssetLoader;
use ggez::graphics::Image;
use entity::{DecorationLibrary, Decoration};

use std::collections::HashMap;
use std::rc::Rc;

pub const TILE_WIDTH: u32 = 128;
pub const TILE_HEIGHT: u32 = 128;

#[derive(Debug)]
pub struct TileType {
    // Defines data common across various types of tiles (Flyweight Pattern)
    pub image: Rc<Image>,
    pub is_walkable: bool
}

#[derive(Debug)]
pub struct Tile {
    // Represents a game tile in the world
    pub meta: Rc<TileType>,
    pub decorations: Option<Vec<Decoration>>,
}

#[derive(Debug)]
pub struct TileLibrary {
    // Represents a library of different tile types
    pub tiles: HashMap<String, Rc<TileType>>
}

impl TileLibrary {
    pub fn new() -> Self {
        TileLibrary {
            tiles: HashMap::new(),
        }
    }

    pub fn load(&mut self, tile_id: &str, tile_type: TileType) {
        self.tiles.insert(String::from(tile_id), Rc::new(tile_type));
    }
}

#[derive(Debug)]
pub struct World {
    pub name: String,

    pub data: Vec<Vec<Tile>>,
    pub rows: u32,
    pub columns: u32,

    pub asset_loader: AssetLoader,
    pub tile_library: TileLibrary,
    pub decorations_library: DecorationLibrary
}

impl World {
    pub fn new(name: String) -> Self {
        World {
            name: name,
            data: Vec::new(),
            rows: 10,
            columns: 10,
            asset_loader: AssetLoader::new(),
            tile_library: TileLibrary::new(),
            decorations_library: DecorationLibrary::new(),
        }
    }

    pub fn get_width(&self) -> u32 {
        // Get the width of the world in pixels
        self.columns * TILE_WIDTH
    }

    pub fn get_height(&self) -> u32 {
        // Get the height of the world in pixels
        self.rows * TILE_HEIGHT
    }

    pub fn get_tiles(&self, left: f32, right: f32, top: f32, bottom: f32) -> Vec<Vec<&Tile>> {
        let idx_left = (left / TILE_WIDTH as f32).floor() as usize;
        let mut idx_right = (right / TILE_WIDTH as f32).floor() as usize;
        let idx_top = (top / TILE_HEIGHT as f32).floor() as usize;
        let mut idx_bottom = (bottom / TILE_HEIGHT as f32).floor() as usize;

        if idx_right >= self.columns as usize {
            idx_right = (self.columns - 1) as usize;
        }

        if idx_bottom >= self.rows as usize {
            idx_bottom = (self.rows - 1) as usize;
        }

        // println!("Cam stuff: {} {} {} {}", left, right, top, bottom);
        // println!("Indexes in the world are: Left - {}, Right - {}", idx_left, idx_right);
        // println!("Indexes in the world are: Top - {}, Bottom - {}", idx_top, idx_bottom);

        let mut tmp_rows: Vec<Vec<&Tile>> = Vec::new();
        for i in idx_top..(idx_bottom + 1) {
            let mut tmp_cols: Vec<&Tile> = Vec::new();
            for j in idx_left..(idx_right + 1) {
                tmp_cols.push(&self.data[i][j]);
            }
            tmp_rows.push(tmp_cols);
        }

        tmp_rows
    }
}
