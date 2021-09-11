//! Sends entity-related packets to clients.
//! Spawn packets, position updates, equipment, animations, etc.

use base::Position;
use common::{Game, events::EntityDamageEvent};
use ecs::{Entity, SysResult, SystemExecutor};
use quill_common::components::{AwaitingRespawn, Health, OnGround};

use crate::{ClientId, NetworkId, Server, entities::PreviousPosition};

mod spawn_packet;

pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
    spawn_packet::register(game, systems);
    systems.group::<Server>()
        .add_system(send_entity_movement)
        .add_system(send_damage);
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

fn send_entity_damage(game: &Game, server: &mut Server, damage: &EntityDamageEvent, entity: Entity) -> SysResult {
    let entity_ref = game.ecs.entity(entity)?;
    let mut health = entity_ref.get_mut::<Health>()?;
    let position  = *entity_ref.get::<Position>()?;
    health.damage(damage.damage);
    if health.health > 0.0 {
        server.broadcast_nearby_with(position, |client| {
            client.send_entity_status(damage.id, 2);
        });
    } else {
        server.broadcast_nearby_with(position, |client| {
            client.send_entity_status(damage.id, 3);
        });
    }
    Ok(())
}

fn send_player_damage(game: &Game, server: &mut Server, damage: &EntityDamageEvent, entity: Entity) -> SysResult {
    let entity_ref = game.ecs.entity(entity)?;
    let mut health = entity_ref.get_mut::<Health>()?;
    let position = *entity_ref.get::<Position>()?;
    let client_id = *entity_ref.get::<ClientId>()?;

    health.damage(damage.damage);
    let client = server.clients.get(client_id).unwrap();
    if health.health > 0.0 {
        client.update_health(health.health, 15, 5.0);
        server.broadcast_nearby_with(position, |client| {
            client.send_entity_status(damage.id, 2);
        });
    } else {
        client.update_health(health.health, 15, 5.0);
        println!("Is dead");
        server.broadcast_nearby_with(position, |client| {
            client.send_entity_status(damage.id, 3);
        });
        game.ecs.get_mut::<AwaitingRespawn>(entity)?.0 = true;
    }
    Ok(())
}

fn send_damage(game: &mut Game, server: &mut Server) -> SysResult {
    for (entity, damage) in game.ecs.query::<&EntityDamageEvent>().iter() {
        let entity_ref = game.ecs.entity(entity)?;
        let entity_network_id = entity_ref.get::<ClientId>();
        println!("Found ClientId");
        match entity_network_id {
            Ok(_) => return send_player_damage(game, server, damage, entity),
            Err(_) => return send_entity_damage(game, server, damage, entity),
        }
    }
    Ok(())
}