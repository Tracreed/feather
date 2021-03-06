use crate::Game;
use base::Position;
use ecs::{SysResult, SystemExecutor};

struct BB {
    height: u32,
    width: u32,
}

pub struct Velocity {
    x: f64,
    y: f64,
    z: f64,
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
            gravity: 9.81,
            drag: 0.02,
            bounding_box: BB {
                height: 1,
                width: 1,
            },
            dba: true,
        }
    }
}

pub fn physics_system(game: &mut Game) -> SysResult {
    for (_entity, (phys, pos, vel)) in game
        .ecs
        .query::<(&mut Physics, &mut Position, &mut Velocity)>()
        .iter()
    {
        // add velocity to position
        pos.x += vel.x;
        pos.y += vel.y;
        pos.z += vel.z;

        // apply gravity to vel
        vel.y -= phys.gravity;
    }

    Ok(())
}

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.add_system(physics_system);
}
