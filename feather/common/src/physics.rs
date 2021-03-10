use crate::Game;
use base::{BlockPosition, Position};
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

impl Default for Velocity{
    fn default() -> Velocity{
        Velocity{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
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
    for (_entity, (phys, mut pos, mut vel)) in game
        .ecs
        .query::<(&mut Physics, &mut Position, &mut Velocity)>()
        .iter()
    {
        let mut new_pos: Position = Position{
            x: pos.x+vel.x, 
            y: pos.y+vel.y, 
            z: pos.z+vel.z, 
            pitch: pos.pitch, 
            yaw:pos.yaw
        };

        //apply gravity to vel
        if game.block(new_pos.block())
            .unwrap()                        
            .is_solid()
        {
            vel.y = 0.0; 
            new_pos.y = new_pos.block().y as f64 + 1.0;
        }else{
            vel.y -= 0.08;
            //new_pos.y += vel.y;
        }


        // set future position
        pos.x = new_pos.x;
        pos.y = new_pos.y;
        pos.z = new_pos.z;

    }

    Ok(())
}


pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.add_system(physics_system);
}

