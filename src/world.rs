extern crate ggez;

use ggez::graphics::Rect;

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

    pub fn get_visible_subset(&self, rect: Rect) -> Vec<Vec<&Tile>> {
        // Returns a subset of tiles (matrix form) that would be encapsulated by the supplied rectangle.
        // Note: This method is greedy in that it will include whole tiles, so if tiles are partially
        // out of the bounds, they will also be returned.
        let idx_left = (rect.left() / TILE_WIDTH as f32).floor() as usize;
        let mut idx_right = (rect.right() / TILE_WIDTH as f32).floor() as usize;
        let idx_top = (rect.top() / TILE_HEIGHT as f32).floor() as usize;
        let mut idx_bottom = (rect.bottom() / TILE_HEIGHT as f32).floor() as usize;

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
