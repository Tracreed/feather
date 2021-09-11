use ecs::{SysResult, SystemExecutor};

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(check_dead_players);
}
