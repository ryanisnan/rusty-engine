use std::fmt::Debug;

use entity::Entity;
use world::World;

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


pub struct Physics {}

impl Physics {
    pub fn is_satisfied<T: Entity + Debug>(world: &World, obj: &T, action: Option<&PhysicsAction>) -> bool {
        match action {
            Some(Move) => {
                if cfg!(feature = "physics_debug") {
                    println!("Physics engine satisified, executing {:?}", action);
                }
                true
            },
            None => true
        }
    }

    pub fn execute<T: Entity + Debug>(obj: &mut T, action: Option<&PhysicsAction>) {
        if cfg!(feature = "physics_debug") {
            println!("Physics engine executing {:?}", action);
        }

        match action {
            Some(variant) => {
                match variant {
                    &PhysicsAction::Move(ref u) => {
                        match &u.direction {
                            &Direction::Up => {
                                obj.get_point().y += u.distance;
                            },
                            &Direction::UpRight => {
                                obj.get_point().x += u.distance;
                                obj.get_point().y += u.distance;
                            },
                            &Direction::Right => {
                                obj.get_point().x += u.distance;
                            },
                            &Direction::DownRight => {
                                obj.get_point().x += u.distance;
                                obj.get_point().y -= u.distance;
                            },
                            &Direction::Down => {
                                obj.get_point().y -= u.distance;
                            },
                            &Direction::DownLeft => {
                                obj.get_point().x -= u.distance;
                                obj.get_point().y -= u.distance;
                            },
                            &Direction::Left => {
                                obj.get_point().x -= u.distance;
                            },
                            &Direction::UpLeft => {
                                obj.get_point().x -= u.distance;
                                obj.get_point().y += u.distance;
                            },
                        }

                    }
                }
            },
            None => (),
        }

        if cfg!(feature = "physics_debug") {
            println!("Physics engine post-execution: {:?}", obj);
        }
    }

    pub fn move_up<T: Entity>(mut obj: T, distance: f32) {
        obj.get_point().y += distance;
    }
}
