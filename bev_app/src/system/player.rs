
use bevy::prelude::*;
use protocol::generated::client::ConnectionState;
use rand::{rngs::StdRng, SeedableRng};

use crate::{components::players::{add_other_player, Connected, Disconnected, Player}, network::PlayerConnectionEvent};


pub fn system_player_connection_event(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut ev_player_connection: EventReader<PlayerConnectionEvent>,

    mut players: Query<(Entity, &mut Player)>,
) {

    for ev in ev_player_connection.read() {

        let other_player = ev.peer_info.other_player.as_ref().unwrap();

        match ev.peer_info.state {
            ConnectionState::Connected => {
                if ev.is_reconnect {
                    for (e, mut p) in players.iter_mut() {
                        if p.name.eq(&other_player.name) {
                            commands.entity(e)
                                .insert((Connected{}, Text2d::new(other_player.name.clone())))
                                .remove::<Disconnected>();
                        }
                    }
                    return;
                }
                add_other_player(&mut commands, &mut meshes, &mut materials, &other_player.position, Player {
                    name: other_player.name.clone(),
                    network_state: ConnectionState::Connected,
                    rng: StdRng::seed_from_u64(other_player.hash),
                });
            },
            ConnectionState::Disconnected => {
                for (e, mut p) in players.iter_mut() {
                    if p.name.eq(&other_player.name) {
                        p.network_state = ConnectionState::Connected;

                        commands.entity(e).insert((
                            Disconnected{},
                            Text2d::new(format!("Disconnected: {}", other_player.name)),
                        )).remove::<Connected>();
                    }

                }
            },
        }
    }

}