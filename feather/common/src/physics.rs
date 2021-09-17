use std::cmp::Ordering;

use crate::{Game, World};
use base::EntityKind;
use ecs::{SysResult, SystemExecutor};
use libcraft_core::{BlockPosition, Position, Vec3f, Vec3i, Velocity};
use quill_common::components::OnGround;

pub struct Physics {
    gravity: f64,
    drag: f64,
    bounding_box: vek::Aabb::<f64>,
    dba: bool, // drag before acceleration
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Side {
    NORTH,
    SOUTH,
    WEST,
    EAST,
    TOP,
    BOTTOM,
    NONE
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::NORTH => f.write_str("North"),
            Side::SOUTH => f.write_str("South"),
            Side::WEST => f.write_str("West"),
            Side::EAST => f.write_str("East"),
            Side::TOP => f.write_str("Top"),
            Side::BOTTOM => f.write_str("Bottom"),
            Side::NONE => f.write_str("None"),
        }
    }
}

impl Default for Physics {
    fn default() -> Physics {
        Physics {
            gravity: 0.08,
            drag: 0.02,
            bounding_box: vek::Aabb {
                min: vek::Vec3::zero(),
                max: vek::Vec3 {x: 0.6 as f64, y: 1.8 as f64, z: 0.6 as f64},
            },
            dba: true,
        }
    }
}

struct Ray {
    origin: Vec3f,
    direction: Vec3f,
}

/// The position at which a ray impacts a block.
#[derive(Debug, Clone, PartialEq)]
pub struct RayImpact {
    /// The position of the block which was impacted.
    pub block: BlockPosition,
    /// The exact position, in world coordinates, at
    /// which the ray met the block.
    pub pos: Position,
    /// The face(s) of the block where the ray impacted.
    pub face: Side,
}

pub fn check_collision(world: &World, old_pos: &mut Position, new_pos: Position) -> Option<RayImpact> {
    struct Step {
        x: i32,
        y: i32,
        z: i32,
    }

    let mut step = Step { x: 0, y: 0, z: 0 };

    let mut ray = Ray {
        origin: Vec3f {
            x: old_pos.x as f32,
            y: old_pos.y as f32,
            z: old_pos.z as f32,
        },
        direction: Vec3f {
            x: new_pos.x as f32 - old_pos.x as f32,
            y: new_pos.y as f32 - old_pos.y as f32,
            z: new_pos.z as f32 - old_pos.z as f32,
        },
    };

    // Used to track where we currently are along the ray.
    let mut position = old_pos.block();

    ray.direction.normalize();

    let mut delta = Vec3f {
        x: f32::INFINITY,
        y: f32::INFINITY,
        z: f32::INFINITY,
    };

    let mut next = Vec3f {
        x: f32::INFINITY,
        y: f32::INFINITY,
        z: f32::INFINITY,
    };

    match ray.direction.x.partial_cmp(&0.0).unwrap() {
        Ordering::Greater => {
            step.x = 1;
            delta.x = 1.0 / ray.direction.x;
            next.x = ((ray.origin.x + 1.0).floor() - ray.origin.x) / ray.direction.x; // Brings X position to next integer
        }
        Ordering::Less => {
            step.x = -1;
            delta.x = (1.0 / ray.direction.x).abs();
            next.x = ((ray.origin.x - (ray.origin.x - 1.0).ceil()) / ray.direction.x).abs();
        }
        _ => (),
    }

    match ray.direction.y.partial_cmp(&0.0).unwrap() {
        Ordering::Greater => {
            step.y = 1;
            delta.y = 1.0 / ray.direction.y;
            next.y = ((ray.origin.y + 1.0).floor() - ray.origin.y) / ray.direction.y;
        }
        Ordering::Less => {
            step.y = -1;
            delta.y = (1.0 / ray.direction.y).abs();
            next.y = ((ray.origin.y - (ray.origin.y - 1.0).ceil()) / ray.direction.y).abs();
        }
        _ => (),
    }

    match ray.direction.z.partial_cmp(&0.0).unwrap() {
        Ordering::Greater => {
            step.z = 1;
            delta.z = 1.0 / ray.direction.z;
            next.z = ((ray.origin.z + 1.0).floor() - ray.origin.z) / ray.direction.z;
        }
        Ordering::Less => {
            step.z = -1;
            delta.z = (1.0 / ray.direction.z).abs();
            next.z = ((ray.origin.z - (ray.origin.z - 1.0).ceil()) / ray.direction.z).abs();
        }
        _ => (),
    }

    let mut face = Side::NONE;


    let mut distance =  Vec3f { x: 0.0, y: 0.0, z: 0.0 };
    let max_distance: f32 = 100.0;
    while distance.magnitude_squared() < max_distance {
        // impact!
        match world.block_at(BlockPosition { x: position.x, y: position.y, z: position.z }) {
            Some(block) => {
                if block.is_solid() {
                    println!("Was a block at: {}, {}, {}", position.x, position.y, position.z);
                    println!("Face: {}", face);
                    let pos = ray.origin + ray.direction * distance;
                    return Some(RayImpact {
                        block: position.into(),
                        pos: Position {
                            x: pos.x as f64,
                            y: pos.y as f64,
                            z: pos.z as f64,
                            pitch: new_pos.pitch,
                            yaw: new_pos.yaw,
                        },
                        face,
                    });
                }
            }
            None => {
                println!("Chunk not loaded");
                return None;
            }
        }

        if next.x < next.y {
            if next.x < next.z {
                position.x += step.x;
                next.x += delta.x;
                distance.x += 1.0;
                face = if step.x == 1 {
                    Side::WEST
                } else {
                    Side::EAST
                };
            } else {
                position.z += step.z;
                next.z += delta.z;
                distance.z += 1.0;
                face = if step.z == 1 {
                    Side::SOUTH
                } else {
                    Side::NORTH
                };
            }
        } else {
            if next.y < next.z {
                position.y += step.y;
                next.y += delta.y;
                distance.y += 1.0;
                face = if step.y == 1 {
                    Side::BOTTOM
                } else {
                    Side::TOP
                };
            } else {
                position.z += step.z;
                next.z += delta.z;
                distance.z += 1.0;
                face = if step.z == 1 {
                    Side::SOUTH
                } else {
                    Side::NORTH
                }
            }
        }
    }
    // If we didn't hit a block we just return None
    None
}

pub fn physics_system(game: &mut Game) -> SysResult {
    for (_entity, (pos, vel, on_ground)) in game.ecs.query::<(&mut Position, &mut Velocity, &mut OnGround)>().iter() {
        let mut new_pos: Position = Position {
            x: pos.x + vel.x,
            y: pos.y + vel.y,
            z: pos.z + vel.z,
            pitch: pos.pitch,
            yaw: pos.yaw,
        };

        //let pending: Vec3f = 

        //                                           acceleration/blocks    drag    term
        //  Players and other living entities [note 1] 	0.08 	         	0.02 	3.92

        let mut gravity_tick = 1.0;

        if !on_ground.0 {
            //let strength = 0.02 * gravity_tick;

            let new_y = vel.y - 0.08;
            vel.y = new_y;
            vel.x *= 1.0 - 0.02;
            vel.y *= 1.0 - 0.02;
            vel.z *= 1.0 - 0.02;
        }

        println!("Entity With velocity: {:?}", vel);

        match check_collision(&game.world, pos, new_pos) {
            Some(impact) => {
                /*pos.x = clamp_pos.x as f64;
                pos.y = clamp_pos.y as f64;
                pos.z = clamp_pos.z as f64;*/

                println!("Impact!: {:?}", impact);

                pos.pitch = new_pos.pitch;
                pos.yaw = new_pos.yaw;

                /*vel.x = 0.0;
                vel.y = 0.0;
                vel.z = 0.0;*/
                //on_ground.0 = true;
            }
            None => {
                // valid movement
                //*pos = new_pos;
                pos.x = new_pos.x;
                pos.y = new_pos.y;
                pos.z = new_pos.z;
                pos.pitch = new_pos.pitch;
                pos.yaw = new_pos.yaw;
                on_ground.0 = false;
            }
        }
    }

    Ok(())
}

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.add_system(physics_system);
}
