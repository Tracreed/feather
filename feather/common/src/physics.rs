use crate::{Game, World};
use base::EntityKind;
use ecs::{SysResult, SystemExecutor};
use libcraft_core::{BlockPosition, Position, Vec3f, Vec3i, Velocity};
use quill_common::components::Name;

struct BB {
    height: u32,
    width: u32,
}

pub struct Physics {
    gravity: f64,
    drag: f64,
    bounding_box: BB,
    dba: bool, // drag before acceleration
}

impl Default for Physics {
    fn default() -> Physics {
        Physics {
            gravity: 0.08,
            drag: 0.02,
            bounding_box: BB {
                height: 1,
                width: 1,
            },
            dba: true,
        }
    }
}

struct Ray {
    origin: Vec3f,
    direction: Vec3f,
}

pub fn check_collision(world: &World, old_pos: &mut Position, new_pos: Position) -> Option<Vec3f> {
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
    let mut position = Vec3i {
        x: (old_pos.x as f32 / 1_f32).floor() as i32,
        y: (old_pos.y as f32 / 1_f32).floor() as i32,
        z: (old_pos.z as f32 / 1_f32).floor() as i32,
    };

    let new_block_position = BlockPosition {
        x: new_pos.x as i32,
        y: new_pos.y as i32,
        z: new_pos.z as i32,
    };

    let last_voxel: Vec3f = Vec3f {
        x: (new_pos.x as f32 / 1_f32).floor(),
        y: (new_pos.y as f32 / 1_f32).floor(),
        z: (new_pos.z as f32 / 1_f32).floor(),
    };

    if ray.direction.x < 0_f32 {
        step.x = -1;
    } else {
        step.x = 1;
    }

    if ray.direction.y < 0_f32 {
        step.y = -1;
    } else {
        step.y = 1;
    }

    if ray.direction.z < 0_f32 {
        step.z = -1;
    } else {
        step.z = 1;
    }

    let delta_x = 1_f32 / ray.direction.x * step.x as f32;
    let delta_y = 1_f32 / ray.direction.y * step.y as f32;
    let delta_z = 1_f32 / ray.direction.z * step.z as f32;

    let next_voxel_boundary_x = (position.x as f32 + step.x as f32) / ray.direction.x;
    let next_voxel_boundary_y = (position.y as f32 + step.y as f32) / ray.direction.y;
    let next_voxel_boundary_z = (position.z as f32 + step.z as f32) / ray.direction.z;

    //let step_size = sqrtf64()

    let mut tmax_x = (next_voxel_boundary_x / old_pos.x as f32) / ray.direction.x;
    let mut tmax_y = (next_voxel_boundary_y / old_pos.x as f32) / ray.direction.y;
    let mut tmax_z = (next_voxel_boundary_z / old_pos.x as f32) / ray.direction.z;

    let mut diff: Vec3i = Vec3i {
        x: 0,
        y: 0,
        z: 0,
    };

    let mut neg_ray: bool = false;

    if position.x as f32 != last_voxel.x && ray.direction.x < 0_f32 {
        diff.x -= 1;
        neg_ray = true;
    };
    if position.y as f32 != last_voxel.y && ray.direction.y < 0_f32 {
        diff.y -= 1;
        neg_ray = true;
    };
    if position.z as f32 != last_voxel.z && ray.direction.z < 0_f32 {
        diff.z -= 1;
        neg_ray = true;
    };

    //println!("Position before if: {}", position);

    if is_solid(world, position) {
        return Some(Vec3f {
            x: position.x as f32,
            y: position.y as f32,
            z: position.z as f32,
        });
    };

    if neg_ray {
        position += diff;
        if is_solid(world, position) {
            return Some(Vec3f {
                x: position.x as f32,
                y: position.y as f32,
                z: position.z as f32,
            });
        } else {
            return None;
        }
    }

    println!("x: {}, y: {}, z: {}, tmax_x: {}, tmax_y: {}, tmax_z: {}", position.x, position.y, position.z, tmax_x, tmax_y, tmax_z);

    loop {
        if tmax_x < tmax_y {
            if tmax_x < tmax_z {
                position.x += step.x;
                /*if step.x > 0 && position.x > new_block_position.x || step.x < 0 && position.x < new_block_position.x
                {
                    return None;
                }*/
                //if (X == justOutX) return(NIL); /* outside grid */
                tmax_x += delta_x;
            } else {
                position.z += step.z;
                /*if step.z > 0 && position.z > new_block_position.z || step.z < 0 && position.z < new_block_position.z
                {
                    return None;
                }*/
                //if (Z == justOutZ) return(NIL);
                tmax_z += delta_z;
            }
        } else {
            if tmax_y < tmax_z {
                position.y += step.y;
                /*if step.y > 0 && position.y > new_block_position.y || step.y < 0 && position.y < new_block_position.y
                {
                    return None;
                }*/
                //if (Y == justOutY) return(NIL);
                tmax_y += delta_y;
            } else {
                position.z += step.z;
                /*if step.z > 0 && position.z > new_block_position.z || step.z < 0 && position.z < new_block_position.z
                {
                    return None;
                }*/
                //if (Z == justOutZ) return(NIL);
                tmax_z += delta_z;
            }
        }

        // impact
        match is_solid(world, position) {
            true => {
                return Some(Vec3f {
                    x: position.x as f32,
                    y: position.y as f32,
                    z: position.z as f32,
                });
            },
            false => return None,
        }
    }
}

fn is_solid(world: &World, position: Vec3i) -> bool {
    match world.block_at(BlockPosition { x: position.x, y: position.y, z: position.z }) {
        Some(block) => {
            if block.is_solid() {
                println!("Was a block at: {}, {}, {}", position.x, position.y, position.z);
                return true;
            } else {
                return false
            }
        }
        None => {
            println!("Chunk not loaded");
            return false;
        }
    }
}

pub fn physics_system(game: &mut Game) -> SysResult {
    for (_entity, (pos, vel)) in game.ecs.query::<(&mut Position, &mut Velocity)>().iter() {
        let new_pos: Position = Position {
            x: pos.x + vel.x,
            y: pos.y + vel.y,
            z: pos.z + vel.z,
            pitch: pos.pitch,
            yaw: pos.yaw,
        };

        match check_collision(&game.world, pos, new_pos) {
            Some(clamp_pos) => {
                pos.x = clamp_pos.x as f64;
                pos.y = clamp_pos.y as f64;
                pos.z = clamp_pos.z as f64;

                pos.pitch = new_pos.pitch;
                pos.yaw = new_pos.yaw;
            }
            None => {
                // valid movement
                //*pos = new_pos;
                pos.x = new_pos.x;
                pos.y = new_pos.y;
                pos.z = new_pos.z;
                pos.pitch = new_pos.pitch;
                pos.yaw = new_pos.yaw;
            }
        }
    }

    Ok(())
}

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.add_system(physics_system);
}
