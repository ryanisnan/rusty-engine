extern crate ggez;

use ggez::graphics::Rect;

use camera::Camera;
use tile::Tile;

pub const TILE_WIDTH: u32 = 128;
pub const TILE_HEIGHT: u32 = 128;

#[derive(Debug)]
pub struct World {
    pub name: String,

    pub data: Vec<Vec<Tile>>,
}

impl World {
    pub fn new(name: String) -> Self {
        World {
            name: name,
            data: Vec::new(),
        }
    }

    pub fn rows(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn cols(&self) -> u32 {
        self.data.get(0).unwrap().len() as u32
    }


    pub fn width(&self) -> u32 {
        // Get the width of the world in pixels
        self.cols() * TILE_WIDTH
    }

    pub fn height(&self) -> u32 {
        // Get the height of the world in pixels
        self.rows() * TILE_HEIGHT
    }

    pub fn get_visible_subset(&self, cam: &Camera) -> Vec<Vec<&Tile>> {
        // Returns a subset of tiles (matrix form) that would be encapsulated by the supplied rectangle.
        // Note: This method is greedy in that it will include whole tiles, so if tiles are partially
        // out of the bounds, they will also be returned.
        let idx_left = ((cam.boundaries.left() - cam.left()).abs() / TILE_WIDTH as f32).floor() as usize;
        let idx_right = ((cam.boundaries.left() - cam.right()).abs() / TILE_WIDTH as f32 - 1.0).ceil() as usize;
        let idx_top = ((cam.boundaries.top() - cam.top()).abs() / TILE_HEIGHT as f32).floor() as usize;
        let idx_bottom = ((cam.boundaries.top() - cam.bottom()).abs() / TILE_HEIGHT as f32 - 1.0).ceil() as usize;

        let mut tmp_rows: Vec<Vec<&Tile>> = Vec::new();
        for i in idx_top..(idx_bottom + 1) {
            let mut tmp_row: Vec<&Tile> = Vec::new();
            for j in idx_left..(idx_right + 1) {
                tmp_row.push(&self.data[i][j]);
            }
            tmp_rows.push(tmp_row);
        }

        tmp_rows
    }

    pub fn show_indexes(&self, cam: &Camera) -> () {
        // Debug method!!
        let idx_left = ((cam.boundaries.left() - cam.left()).abs() / TILE_WIDTH as f32).floor() as usize;
        let idx_right = ((cam.boundaries.left() - cam.right()).abs() / TILE_WIDTH as f32 - 1.0).ceil() as usize;
        let idx_top = ((cam.boundaries.top() - cam.top()).abs() / TILE_HEIGHT as f32).floor() as usize;
        let idx_bottom = ((cam.boundaries.top() - cam.bottom()).abs() / TILE_HEIGHT as f32 - 1.0).ceil() as usize;

        println!("idx_left {}", idx_left);
        println!("idx_right {}", idx_right);
        println!("idx_top {}", idx_top);
        println!("idx_bottom {}", idx_bottom);
    }

}
