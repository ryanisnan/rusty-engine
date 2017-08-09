extern crate ggez;

use assets::AssetLoader;
use ggez::graphics::Image;
use ggez::Context;
use entity::{DecorationLibrary, Decoration, DecorationType};

use std::collections::HashMap;
use std::rc::Rc;

pub const TILE_WIDTH: u32 = 128;
pub const TILE_HEIGHT: u32 = 128;


#[derive(Debug, PartialEq, Eq, Hash)]
enum TileType {
    // Basic enum listing the various types of tiles within the world
    GrassLight,
    GrassLight2,
    GrassLight3,
    GrassDark,
    Sand,
    Sand2,
    Sand3,
    GrassSandNW,
    GrassSandN,
    GrassSandN2,
    GrassSandN3,
    GrassSandNE,
    GrassSandE,
    GrassSandE2,
    GrassSandE3,
    GrassSandSE,
    GrassSandS,
    GrassSandS2,
    GrassSandS3,
    GrassSandSW,
    GrassSandW,
    GrassSandW2,
    GrassSandW3,
    SandGrassNW,
    SandGrassNE,
    SandGrassSE,
    SandGrassSW,
}

#[derive(Debug)]
pub struct TileMeta {
    // Defines data common across various types of tiles (Flyweight Pattern)
    pub image: Rc<Image>,
    pub is_walkable: bool
}

#[derive(Debug)]
pub struct Tile {
    // Represents a game tile in the world
    pub meta: Rc<TileMeta>,
    pub decorations: Option<Vec<Decoration>>,
}

#[derive(Debug)]
pub struct TileLibrary {
    // Represents a library of different tile types
    tiles: HashMap<TileType, Rc<TileMeta>>
}

impl TileLibrary {
    pub fn new(ctx: &mut Context, asset_loader: &mut AssetLoader) -> Self {
        let mut lib = TileLibrary {
            tiles: HashMap::new(),
        };

        lib.populate(ctx, asset_loader);

        lib
    }

    fn populate(&mut self, ctx: &mut Context, asset_loader: &mut AssetLoader) {
        self.tiles.insert(TileType::GrassLight, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/lgrass.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassLight2, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/lgrass2.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassLight3, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/lgrass3.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassDark, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/dgrass.png"), is_walkable: true }));
        self.tiles.insert(TileType::Sand, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/sand.png"), is_walkable: true }));
        self.tiles.insert(TileType::Sand2, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/sand2.png"), is_walkable: true }));
        self.tiles.insert(TileType::Sand3, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/sand3.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassSandNW, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/grass-sand-nw.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassSandN, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/grass-sand-n.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassSandN2, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/grass-sand-n2.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassSandN3, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/grass-sand-n3.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassSandNE, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/grass-sand-ne.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassSandE, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/grass-sand-e.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassSandE2, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/grass-sand-e2.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassSandE3, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/grass-sand-e3.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassSandSE, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/grass-sand-se.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassSandS, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/grass-sand-s.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassSandS2, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/grass-sand-s2.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassSandS3, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/grass-sand-s3.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassSandSW, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/grass-sand-sw.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassSandW, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/grass-sand-w.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassSandW2, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/grass-sand-w2.png"), is_walkable: true }));
        self.tiles.insert(TileType::GrassSandW3, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/grass-sand-w3.png"), is_walkable: true }));
        self.tiles.insert(TileType::SandGrassNW, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/sand-grass-nw.png"), is_walkable: true }));
        self.tiles.insert(TileType::SandGrassNE, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/sand-grass-ne.png"), is_walkable: true }));
        self.tiles.insert(TileType::SandGrassSE, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/sand-grass-se.png"), is_walkable: true }));
        self.tiles.insert(TileType::SandGrassSW, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/sand-grass-sw.png"), is_walkable: true }));
    }
}

#[derive(Debug)]
pub struct World {
    pub name: String,

    pub data: Vec<Vec<Tile>>,
    pub rows: u32,
    pub columns: u32,

    asset_loader: AssetLoader,
    tile_library: TileLibrary,
    decorations_library: DecorationLibrary
}

impl World {
    pub fn new(name: String, ctx: &mut Context) -> Self {
        let mut asset_loader = AssetLoader::new();

        let tile_library = TileLibrary::new(ctx, &mut asset_loader);
        let decorations_library = DecorationLibrary::new(ctx, &mut asset_loader);

        World {
            name: name,
            data: Vec::new(),
            rows: 10,
            columns: 10,
            asset_loader: asset_loader,
            tile_library: tile_library,
            decorations_library: decorations_library,
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

    fn generate_grass_tile(&self, decorations: Option<Vec<Decoration>>) -> Tile {
        let g = self.tile_library.tiles[&TileType::GrassLight].clone();
        Tile { meta: g, decorations: decorations}
    }

    fn generate_water_tile(&self, decorations: Option<Vec<Decoration>>) -> Tile {
        let w = self.tile_library.tiles[&TileType::Sand].clone();
        Tile { meta: w, decorations: decorations}
    }

    pub fn load_world_1(&mut self) {
        // Row 1
        let r1 = vec![
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
        ];
        self.data.push(r1);

        // Row 2
        let r2 = vec![
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: Some(vec![Decoration{flyweight: self.decorations_library.decorations[&DecorationType::Bush1x1].clone()}])},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
        ];
        self.data.push(r2);

        // Row 3
        let r3 = vec![
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassSandNW].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassSandN].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassSandN3].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassSandNE].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
        ];
        self.data.push(r3);

        // Row 4
        let r4 = vec![
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassSandSW].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::SandGrassNE].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::Sand].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassSandE3].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
        ];
        self.data.push(r4);

        // Row 5
        let r5 = vec![
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassSandSW].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassSandS].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassSandSE].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
        ];
        self.data.push(r5);

        // Row 6
        let r6 = vec![
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: Some(vec![Decoration{flyweight: self.decorations_library.decorations[&DecorationType::Stones].clone()}])},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
        ];
        self.data.push(r6);

        // Row 7
        let r7 = vec![
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: Some(vec![Decoration{flyweight: self.decorations_library.decorations[&DecorationType::Stones].clone()}])},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: Some(vec![Decoration{flyweight: self.decorations_library.decorations[&DecorationType::Stones].clone()}])},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: Some(vec![Decoration{flyweight: self.decorations_library.decorations[&DecorationType::Stones].clone()}])},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
            Tile { meta: self.tile_library.tiles[&TileType::GrassLight].clone(), decorations: None},
        ];
        self.data.push(r7);

        // Row 8
        let r8 = vec![
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
        ];
        self.data.push(r8);

        // Row 9
        let r9 = vec![
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
        ];
        self.data.push(r9);

        // Row 10
        let r10 = vec![
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
        ];
        self.data.push(r10);
    }
}
