use base::Gamemode;
use common::Game;
use ecs::{Entity, EntityRef, SysResult};
use protocol::packets::client::ClientStatus;

use crate::{ClientId, Server};

pub fn handle_client_status(game: &Game, player: EntityRef, server: &Server, packet: ClientStatus) -> SysResult {
    if let Some(client) = server.clients.get(*player.get::<ClientId>()?) {
        match packet {
            ClientStatus::PerformRespawn => {
                client.send_respawn(*player.get::<Gamemode>()?);
            },
            ClientStatus::RequestStats => todo!(),
        }
    }
    Ok(())
}