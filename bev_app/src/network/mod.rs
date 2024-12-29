
use bevy::{prelude::*, utils::{HashMap}};
use bevy_matchbox::prelude::*;
use rand::{rngs::StdRng, SeedableRng};

use crate::{components::players::{add_other_player, remove_other_player, Player}, resource::{server::{generate_rool_url, SelectedMatchboxServer, SelectedRoom}, MyPlayerResource}};

const CHANNEL_ID: usize = 0;


/*
* Procotole pour communication.
* 
*
*/

const X_EXTENT: f32 = 900.;

pub enum NetworkState {
    Connected = 0,
    Disconnected = 1,
    Left = 2,
}

pub struct RoomConfiguration {
    pub name: String,
    pub max_players: u32,
    pub agreement_type: String,
}

pub struct PeerInfo {
    pub name: String,
    pub state: NetworkState,
}

#[derive(Resource)]
pub struct PeersInfo {
    pub peers: HashMap<String, PeerInfo>,
}


pub fn start_socket(
    mut commands: Commands,
    selected_server: Res<SelectedMatchboxServer>,
    selected_room: Res<SelectedRoom>,
    my_player: Res<MyPlayerResource>,
) {
    let socket = MatchboxSocket::new_reliable(
        generate_rool_url(&selected_server, &selected_room, &my_player)
    );
    commands.insert_resource(socket);
    commands.insert_resource(PeersInfo{peers: HashMap::new()});
}

pub fn send_message(mut socket: ResMut<MatchboxSocket>) {
    let peers: Vec<_> = socket.connected_peers().collect();

    for peer in peers {
        let message = "Hello";
        info!("Sending message: {message:?} to {peer}");
        socket
            .channel_mut(CHANNEL_ID)
            .send(message.as_bytes().into(), peer);
    }
}

pub fn receive_messages(
    mut socket: ResMut<MatchboxSocket>,
    mut peers_info: ResMut<PeersInfo>,


    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    players: Query<(Entity, &Player)>,

) {
    for (peer, state) in socket.update_peers() {
        match state {
            PeerState::Connected => {
                peers_info.peers.insert(peer.0.to_string(), PeerInfo { name: "alice".to_string(), state: NetworkState::Connected });
                let position = Transform::from_xyz(-1_f32 * X_EXTENT / 2.0 + 100.0, 0.0, 0.0);

                add_other_player(&mut commands, &mut meshes, &mut materials, position, Player {
                    name: "alice".to_string(),
                    network_state: NetworkState::Connected,
                    rng: StdRng::seed_from_u64(14325325)
                });
            },
            PeerState::Disconnected => {
                let peer_info = peers_info.peers.remove(&peer.0.to_string()).unwrap();

                remove_other_player(&mut commands, peer_info.name.clone(), &players);

            }
        }
        info!("{peer}: {state:?}");
    }

    for (_id, message) in socket.channel_mut(CHANNEL_ID).receive() {
        match std::str::from_utf8(&message) {
            Ok(message) => info!("Received message: {message:?}"),
            Err(e) => error!("Failed to convert message to string: {e}"),
        }
    }
}