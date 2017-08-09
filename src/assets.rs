extern crate ggez;

use ggez::Context;
use ggez::graphics::Image;
use std::rc::Rc;
use std::any::Any;
use std::collections::HashMap;


pub trait Loadable {
    fn load(ctx: &mut Context, file_path: &str) -> Self where Self: Sized;
}

impl Loadable for ggez::graphics::Image {
    fn load(ctx: &mut Context, file_path: &str) -> Self {
        println!("Loading an image...");
        ggez::graphics::Image::new(ctx, file_path).unwrap()
    }
}

#[derive(Debug)]
pub struct AssetLoader {
    assets: HashMap<String, Box<Any>>,
}

impl AssetLoader {
    /*
        Loads assets and stores them in a hashmap with the key being the path for the file,
        and the value is the asset content.
    */
    pub fn new() -> Self {
        AssetLoader {
            assets: HashMap::new(),
        }
    }

    pub fn load<T: Loadable + 'static>(&mut self, ctx: &mut Context, file_path: &str) -> Rc<T> {
        /*
            Loads an asset and stores the asset into the internal assets hashmap where
            the file_path is the key, and the asset is the value.
        */
        println!("AssetLoader: Loading an asset...");

        let asset = self.assets.entry(file_path.into()).or_insert_with(|| Box::new(Rc::new(T::load(ctx, file_path))));

        if !asset.is::<T>() {
            println!("Warning: Loading file as different asset type");
            *asset = Box::new(Rc::new(T::load(ctx, file_path)));
        }

        asset.downcast_ref::<Rc<T>>().unwrap().clone()
    }

    pub fn load_image(&mut self, ctx: &mut Context, path: &str) -> Rc<Image> {
        // Shortcut for loading images
        self.load(ctx, path)
    }
}
