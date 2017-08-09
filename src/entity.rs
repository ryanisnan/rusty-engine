use std::collections::HashMap;
use std::rc::Rc;
use ggez::Context;
use ggez::graphics::Image;
use assets::AssetLoader;

pub trait Entity {} // Things that can be placed into the world will all implement this functionality

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum DecorationType {
    Bush1x1,
    Stone,
    Stones,
}

#[derive(Debug)]
pub struct DecorationFlyweight {
    pub image: Rc<Image>
}

#[derive(Debug)]
pub struct DecorationLibrary {
    pub decorations: HashMap<DecorationType, Rc<DecorationFlyweight>>
}

impl DecorationLibrary {
    pub fn new(ctx: &mut Context, asset_loader: &mut AssetLoader) -> Self {
        let mut dl = DecorationLibrary {
            decorations: HashMap::new(),
        };

        dl.load_assets(ctx, asset_loader);

        dl
    }

    fn load_assets(&mut self, ctx: &mut Context, asset_loader: &mut AssetLoader) {
        self.decorations.insert(DecorationType::Bush1x1, Rc::new(DecorationFlyweight { image: asset_loader.load_image(ctx, "/bush1.png")}));
        self.decorations.insert(DecorationType::Stone, Rc::new(DecorationFlyweight { image: asset_loader.load_image(ctx, "/stone.png")}));
        self.decorations.insert(DecorationType::Stones, Rc::new(DecorationFlyweight { image: asset_loader.load_image(ctx, "/stones.png")}));
    }
}

#[derive(Debug)]
pub struct Decoration {
    pub flyweight: Rc<DecorationFlyweight>,
}
