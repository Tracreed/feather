//! Sends entity-related packets to clients.
//! Spawn packets, position updates, equipment, animations, etc.

use base::Position;
use common::{Game, events::EntityDamageEvent};
use ecs::{Entity, SysResult, SystemExecutor};
use quill_common::components::{Health, OnGround};

use crate::{ClientId, NetworkId, Server, entities::PreviousPosition};

mod spawn_packet;

pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
    spawn_packet::register(game, systems);
    systems.group::<Server>()
        .add_system(send_entity_movement)
        .add_system(send_entity_damage);
}

/// Sends entity movement packets.
fn send_entity_movement(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (&position, prev_position, &on_ground, &network_id)) in game
        .ecs
        .query::<(&Position, &mut PreviousPosition, &OnGround, &NetworkId)>()
        .iter()
    {
        if position != prev_position.0 {
            server.broadcast_nearby_with(position, |client| {
                client.update_entity_position(network_id, position, on_ground);
            });
            prev_position.0 = position;
        }
    }
    Ok(())
}

fn send_entity_damage(game: &mut Game, server: &mut Server) -> SysResult {
    for (entity, (damage, health, client_id, network_id)) in game.ecs.query::<(&EntityDamageEvent, &mut Health, &ClientId, &NetworkId)>().iter() {
        health.damage(damage.damage);
        let client = server.clients.get(*client_id).unwrap();
        if health.health > 0.0 {
            client.update_health( health.health, 15, 5.0);
            client.send_entity_status(network_id.0, 2);
        } else {
            client.update_health( health.health, 15, 5.0);
            client.send_entity_status(network_id.0, 3);
        }
    }
    Ok(())
}
