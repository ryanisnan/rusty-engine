use std::fmt::Debug;

use ggez::graphics::Point;

use entity::Entity;
use world::World;

pub struct Physics {}

impl Physics {
    pub fn is_satisfied<T: Entity>(world: &World, obj: &T, action: Option<&PhysicsAction>) -> bool {
        match action {
            Some(Move) => {
                if cfg!(physics_debug) {
                    println!("Physics engine satisified, executing {:?}", action);
                }
                true
            },
            None => true
        }
    }

    pub fn execute<T: Entity>(obj: &mut T, action: Option<&PhysicsAction>) {
        if cfg!(physics_debug) {
            println!("Physics engine executing {:?}", action);
        }
    }

    pub fn move_up<T: Entity>(mut obj: T, distance: f32) {
        obj.get_point().y += distance;
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

#[derive(Debug)]
pub struct MovePhysicsAction {
    pub direction: Direction,
    pub distance: f32
}

#[derive(Debug)]
pub enum PhysicsAction {
    Move(MovePhysicsAction),
}
