use crate::{Game, World};
use libcraft_core::{BlockPosition, Position, Vec3f, Velocity};
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

struct Ray {
    origin: Vec3f,
    direction: Vec3f
}


pub fn check_collision(world : &World, old_pos: &mut Position, new_pos: Position) -> Option<Vec3f> {

    struct Step {
        x: i32,
        y: i32, 
        z: i32
    }

    let mut step  = Step{x: 0, y:0, z:0};

    let mut ray = Ray{
        origin: Vec3f{x:old_pos.x as f32, y:old_pos.y as f32, z:old_pos.z as f32},
        direction: Vec3f{
            x:  new_pos.x as f32 -  old_pos.x as f32, 
            y:  new_pos.y as f32 -  old_pos.y as f32, 
            z:  new_pos.z as f32 -  old_pos.z as f32
        }
    };


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

    if ray.direction.x < 0_f32 {
        step.x = -1;
    }else {
        step.x = 1;
    }

    if ray.direction.y < 0_f32 {
        step.y = -1;
    }else {
        step.y = 1;
    }

    if ray.direction.z < 0_f32 {
        step.z = -1;
    }else {
        step.z = 1;
    }

    let delta_x = 1_f32 / ray.direction.x;
    let delta_y = 1_f32 / ray.direction.y;
    let delta_z = 1_f32 / ray.direction.z;

    let mut tmax_x = ray.origin.x.ceil() / ray.direction.x;
    let mut tmax_y = ray.origin.y.ceil() / ray.direction.y;
    let mut tmax_z = ray.origin.z.ceil() / ray.direction.z;


    let mut x : i32 = ray.origin.x.floor() as i32;
    let mut y : i32 = ray.origin.y.floor() as i32;
    let mut z : i32 = ray.origin.z.floor() as i32;

    loop {
        if tmax_x < tmax_y  {
            if tmax_x < tmax_z {
                x = x + step.x;
                if  step.x > 0 && x > new_block_position.x  
                ||  step.x < 0 && x < new_block_position.x  
                { return None }
                //if (X == justOutX) return(NIL); /* outside grid */
                tmax_x = tmax_x + delta_x;
            } else {
                z = z + step.z;
                if step.z > 0 && z > new_block_position.z  
                || step.z < 0  && z < new_block_position.z
                { return None }
                //if (Z == justOutZ) return(NIL);
                tmax_z= tmax_z + delta_z;
            }
        } else {
            if tmax_y < tmax_z {
                y = y + step.y;
                if step.y > 0 && y > new_block_position.y  
                || step.y < 0 && y < new_block_position.y
                { return None }
                //if (Y == justOutY) return(NIL);
                tmax_y= tmax_y + delta_y;
            } else {
                z = z + step.z;
                if step.z > 0 && z > new_block_position.z  
                || step.z < 0 && z < new_block_position.z
                { return None }
                //if (Z == justOutZ) return(NIL);
                tmax_z= tmax_z + delta_z;
            }
        }

        // impact
        match world.block_at(BlockPosition{x, y, z}) {
            Some(block) => {
                if block.is_solid(){
                    return Some(Vec3f{x: x as f32, y: y as f32, z: z as f32})
                }        
            },
            None => {},
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


        match check_collision(&game.world, pos, new_pos){
            Some(_clamp_pos) =>{
                //pos.x = clamp_pos.x as f64;
                //pos.y = clamp_pos.y as f64;
                //pos.z = clamp_pos.z as f64;

                //pos.pitch = new_pos.pitch;
                //pos.yaw = new_pos.yaw;
            },
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


