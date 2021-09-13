use std::convert::identity;

use quill::{
    components::CustomName,
    EntityInit, Game, Plugin, Setup, Velocity, Position
};

use rand::Rng;

quill::plugin!(PhysicsTesting);

struct PhysicsTesting {
    tick_counter : u64
}

impl Plugin for PhysicsTesting {
    fn enable(_game: &mut Game, setup: &mut Setup<Self>) -> Self {
        setup.add_system(physics_test_system);
        PhysicsTesting { tick_counter: 0 }
    }

    fn disable(self, _game: &mut Game) {}
}

fn physics_test_system(plugin: &mut PhysicsTesting, game: &mut Game) {

    for(entity, (pos, _vel)) in game.query::<(&Position, &Velocity)>() {
        match entity.get::<quill::components::Name>(){
            Ok(name)=>{
                if plugin.tick_counter % 100 == 0 {
                    // Every even tick spawns mob with random velocity
                    let x = rand::thread_rng().gen_range(-0.75..0.75);
                    let y = rand::thread_rng().gen_range(-0.75..0.75);
                    let z = rand::thread_rng().gen_range(-0.75..0.75);

                    entity.send_message(format!("Spawning mob on {} with velocity : {} {} {}", name, x, y , z));
                    game.create_entity_builder(pos, random_mob())
                        .with(CustomName::new("name"))
                        .with(Velocity{ x, y, z })
                        .finish();
                }
            },
            Err(_) =>{
                return ()
            }
        }
        
    }

    plugin.tick_counter += 1;

}

fn random_mob() -> EntityInit {
    let mut entities = vec![
        EntityInit::Zombie,
        EntityInit::Piglin,
        EntityInit::Zoglin,
        EntityInit::Skeleton,
        EntityInit::Enderman,
        EntityInit::Cow,
    ];
    let index = rand::thread_rng().gen_range(0..entities.len());
    entities.remove(index)
}
