mod mapper;

use bevy::{prelude::*, utils::HashMap};
use bevy_matchbox::prelude::*;
use game::logic::players::OtherPlayer;
use mapper::{my_player_to_presentation, presentation_to_other_player};
use prost::Message;
use protocol::{
    frame::{create_function_frame, parse_function_frame},
    generated::{
        client::ConnectionState,
        msg::{ClientHandlers, Presentation},
    },
};

use crate::resource::{
    server::{generate_rool_url, SelectedMatchboxServer, SelectedRoom},
    MyPlayerResource,
};

const CHANNEL_ID: usize = 0;

#[derive(Clone)]
pub struct NVec3([i32; 3]);

#[derive(Clone)]
pub struct PeerInfo {
    pub state: ConnectionState,
    pub position: Option<NVec3>,
    pub other_player: Option<OtherPlayer>,
}

#[derive(Event)]
pub struct PlayerConnectionEvent {
    pub peer_info: PeerInfo,
    pub is_reconnect: bool,
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
    let socket = MatchboxSocket::new_reliable(generate_rool_url(
        &selected_server,
        &selected_room,
        &my_player,
    ));
    commands.insert_resource(socket);
    commands.insert_resource(PeersInfo {
        peers: HashMap::new(),
    });
}

pub fn send_message<T: prost::Message>(
    socket: &mut ResMut<MatchboxSocket>,
    peer_ids: Vec<String>,
    msg_type: ClientHandlers,
    payload: &T,
) -> Result<(), ()> {
    let peers: Vec<_> = socket.connected_peers().collect();

    let payload = create_function_frame(msg_type.into(), 0, payload).unwrap();

    for peer in peers {
        if peer_ids.len() == 0 || peer_ids.contains(&peer.0.to_string()) {
            info!("sending payload to {}", peer);
            socket
                .channel_mut(CHANNEL_ID)
                .send(payload.clone().into(), peer);
        }
    }

    Ok(())
}

pub fn receive_messages(
    mut socket: ResMut<MatchboxSocket>,
    mut peers_info: ResMut<PeersInfo>,

    mut ev_player_connection: EventWriter<PlayerConnectionEvent>,

    my_player_resource: Res<MyPlayerResource>,
) {
    for (peer, state) in socket.update_peers() {
        match state {
            PeerState::Connected => {
                peers_info.peers.insert(
                    peer.0.to_string(),
                    PeerInfo {
                        position: None,
                        other_player: None,
                        state: ConnectionState::Connected,
                    },
                );

                let presentation = my_player_to_presentation(&my_player_resource);

                send_message(
                    &mut socket,
                    vec![peer.0.to_string()],
                    ClientHandlers::Presentation,
                    &presentation,
                )
                .unwrap();
            }
            PeerState::Disconnected => {
                let peer_info = peers_info.peers.get_mut(&peer.0.to_string()).unwrap();
                peer_info.state = ConnectionState::Disconnected;

                ev_player_connection.send(PlayerConnectionEvent {
                    peer_info: peer_info.clone(),
                    is_reconnect: false,
                });
            }
        }
        info!("{peer}: {state:?}");
    }

    for (peer_id, message) in socket.channel_mut(CHANNEL_ID).receive() {
        if let Ok((id, data)) = parse_function_frame(&message) {
            let id = ClientHandlers::from(id);

            match id {
                ClientHandlers::Presentation => {
                    let p = Presentation::decode(data).unwrap();

                    let other_player = presentation_to_other_player(&p);

                    let new_peer_info = {
                        let peer_info = peers_info.peers.get_mut(&peer_id.0.to_string()).unwrap();
                        peer_info.other_player = Some(other_player);

                        peer_info.clone()
                    };

                    let mut reconnect = false;
                    let mut to_remove = vec![];
                    for (k, peer) in peers_info.peers.iter() {
                        if k.eq(&peer_id.0.to_string()) {
                            continue;
                        }
                        if let Some(peer_player) = peer.other_player.as_ref() {
                            if peer_player.name == new_peer_info.other_player.as_ref().unwrap().name
                            {
                                reconnect = true;
                                to_remove.push(k.clone());
                                break;
                            }
                        }
                    }
                    for item in to_remove.iter() {
                        peers_info.peers.remove(item).unwrap();
                    }

                    ev_player_connection.send(PlayerConnectionEvent {
                        peer_info: new_peer_info,
                        is_reconnect: reconnect,
                    });
                }
                _ => {}
            }
        }
    }
}
