use quill::{
    components::{CustomName},
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

    let max_new_entites: u32 = 10;
    let mut spawned_entities: u32 = 0; 

    for(_entity, (pos, _vel)) in game.query::<(&Position, &Velocity)>() {
        //entity.send_message(format!("[Physiscs Testing] Your position is {:?} and your velocity is {:?}", pos, vel));
        if plugin.tick_counter % 100 == 0 && spawned_entities != max_new_entites  {

            //entity.send_message(format!("[Physiscs Testing] Spawning a mob on you"));
            if plugin.tick_counter % 2 == 0{
                game.create_entity_builder(pos, random_mob())
                    .with(CustomName::new("name"))
                    .with(Velocity{x:0.5, y:0.0, z:0.0})
                    .finish();
            }else{
                game.create_entity_builder(pos, random_mob())
                    .with(CustomName::new("name"))
                    .with(Velocity{x:0.0, y:0.0, z:0.5})
                    .finish();
            }
            
            spawned_entities += 1;
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
