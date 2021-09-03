use crate::Game;
use libcraft_core::{Velocity,Position};
use ecs::{SysResult, SystemExecutor};

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


pub fn physics_system(game: &mut Game) -> SysResult {
    for (_entity, (pos, vel)) in game
        .ecs
        .query::<(&mut Position, &mut Velocity)>()
        .iter()
    {
        let new_pos: Position = Position{
            x: pos.x+vel.x, 
            y: pos.y+vel.y, 
            z: pos.z+vel.z, 
            pitch: pos.pitch, 
            yaw:pos.yaw
        };

        *pos = new_pos;

        // set future position
        // pos.x = new_pos.x;
        // pos.y = new_pos.y;
        // pos.z = new_pos.z;

    }

    Ok(())
}


pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.add_system(physics_system);
}


