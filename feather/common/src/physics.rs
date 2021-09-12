use crate::{Game, World};
use libcraft_core::{BlockPosition, Velocity,Position};
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


pub fn check_collision(world : &World, old_pos: &mut Position, new_pos: Position) -> Option<bool> {

    let old_block_position = BlockPosition{
        x: old_pos.x as i32,
        y: old_pos.y as i32, 
        z: old_pos.z as i32
    };

    let new_block_position = BlockPosition{
        x: new_pos.x as i32,
        y: new_pos.y as i32, 
        z: new_pos.z as i32
    };

    // x-axis only
    for x in old_block_position.x..new_block_position.x+1 {
        let check_block_position = BlockPosition{x, y:new_block_position.y, z:new_block_position.z};

        match world.block_at(check_block_position) {
            Some(block) =>{
                if !block.is_air(){
                    return Some(false)
                }
            },
            None=>{},
        }
    }


    // y-axis only
    for y in old_block_position.y..new_block_position.y+1 {
        let check_block_position = BlockPosition{x:new_block_position.x, y:y, z:new_block_position.z};

        match world.block_at(check_block_position) {
            Some(block) =>{
                if !block.is_air(){
                    return Some(false)
                }
            },
            None=>{},
        }
    }



    // z-axis only
    for z in old_block_position.z..new_block_position.z+1 {
        let check_block_position = BlockPosition{x:new_block_position.x, y:new_block_position.y, z};

        match world.block_at(check_block_position) {
            Some(block) =>{
                if !block.is_air(){
                    return Some(false)
                }
            },
            None=>{},
        }
    }

    // movement is valid
    Some(true)
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


        match check_collision(&game.world, pos, new_pos){
            Some(valid) =>{
                if valid {
                    // set future position
                    pos.x = new_pos.x;
                    pos.y = new_pos.y;
                    pos.z = new_pos.z;
                }else{
                    // don't update position
                    return Ok(())
                }
            },
            None => return Ok(())
        }
    }

    Ok(())
}


pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.add_system(physics_system);
}


