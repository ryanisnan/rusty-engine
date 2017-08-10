extern crate ggez;

use tile::Tile;

pub const TILE_WIDTH: u32 = 128;
pub const TILE_HEIGHT: u32 = 128;

#[derive(Debug)]
pub struct World {
    pub name: String,

    pub data: Vec<Vec<Tile>>,
    pub rows: u32,
    pub columns: u32
}

impl World {
    pub fn new(name: String) -> Self {
        World {
            name: name,
            data: Vec::new(),
            rows: 10,
            columns: 10,
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

    pub fn get_subset(&self, left: f32, right: f32, top: f32, bottom: f32) -> Vec<Vec<&Tile>> {
        // Return a subset of tiles in a matrix form that are within the supplied constraints
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
