use ggez::{Context, graphics};
use ggez::graphics::Point;
use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

use world;
use world::World;
use camera::Camera;

pub struct Scene {
    active_zone: Rc<World>,
    pub drawables: HashMap<i8, Vec<(Box<Any>, Point, f32)>>
}

impl Scene {
    fn add_drawables_from_zone(&mut self, camera: &Camera) {
        // let zone = self.active_zone.clone();
        // let visible_tiles =  zone.get_visible_subset(camera.to_rect());
        // let base_depth = self.drawables.entry(0).or_insert_with(|| Vec::new());
        //
        // let x_offset = camera.left() % world::TILE_WIDTH as f32;
        // let y_offset = camera.top() % world::TILE_HEIGHT as f32;
        //
        // for (i, row) in visible_tiles.iter().enumerate() {
        //     let i = i as f32;
        //     for (j, tile) in row.iter().enumerate() {
        //         let j = j as f32;
        //
        //         let t_x = j * world::TILE_WIDTH as f32 + world::TILE_WIDTH as f32 / 2.0 - x_offset;
        //         let t_y = i * world::TILE_HEIGHT as f32 + world::TILE_HEIGHT as f32 / 2.0 - y_offset;
        //
        //         let p = Point::new(t_x, t_y);
        //
        //         base_depth.push((Box::new(&tile.meta.image), p, 0.0));
        //
        //         for (z, decoration) in tile.decorations.iter().enumerate() {
        //             // self.drawables.entry(1 + z as i8).or_insert_with(|| Vec::new()).push((Box::new(&decoration[0].meta.image), p, 0.0));
        //         }
        //     }
    }

    fn add_drawables_from_dynamics() {
        // Silly mock method to show what other types of things could be here
    }
}


pub struct Renderer {}
impl Renderer {
    pub fn draw_world(scene: &Scene, ctx: &mut Context) {
        graphics::clear(ctx);

        for depth in scene.drawables.keys() {
            for tup in scene.drawables[depth].iter() {
                println!("{:?}", tup);
                // let (drawable, point, rotation): (Box<Any>, Point, f32) = tup;
                // graphics::draw(ctx, &drawable, point, rotation);
            }
        }

        graphics::present(ctx);
        // let tiles = self.world.get_tiles(self.camera.get_left(), self.camera.get_right(), self.camera.get_top(), self.camera.get_bottom());
        // let x_offset = self.camera.get_left() % world::TILE_WIDTH as f32;
        // let y_offset = self.camera.get_top() % world::TILE_HEIGHT as f32;
        //
        // for (i, row) in tiles.iter().enumerate() {
        //     let i = i as f32;
        //     for (j, tile) in row.iter().enumerate() {
        //         let j = j as f32;
        //
        //         let p = graphics::Point::new(j * world::TILE_WIDTH as f32 + world::TILE_WIDTH as f32 / 2.0 - x_offset, i * world::TILE_HEIGHT as f32 + world::TILE_HEIGHT as f32 / 2.0 - y_offset);
        //         graphics::draw(ctx, &tile.meta.image, p, 0.0)?;
        //
        //         for decoration in tile.decorations.iter() {
        //             graphics::draw(ctx, &decoration[0].meta.image, p, 0.0);
        //         }
        //
        //     }
        // }
        // graphics::present(ctx);
        // Ok(())
    }
}
