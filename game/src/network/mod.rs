
use bevy::prelude::*;
use bevy_matchbox::prelude::*;

const CHANNEL_ID: usize = 0;


/*
* Procotole pour communication.
* 
*
*/

pub enum NetworkState {
    NotConnected = 0,
    InRoom = 1,
    InGame = 2,
    Disconnected = 3,
}

pub enum NetworkGameState {

}

pub struct RoomConfiguration {
    pub name: String,
    pub max_players: u32,
    pub agreement_type: String,
}



pub fn start_socket(mut commands: Commands) {
    let socket = MatchboxSocket::new_reliable("ws://localhost:3536/hello");
    commands.insert_resource(socket);
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

pub fn receive_messages(mut socket: ResMut<MatchboxSocket>) {
    for (peer, state) in socket.update_peers() {
        info!("{peer}: {state:?}");
    }

    for (_id, message) in socket.channel_mut(CHANNEL_ID).receive() {
        match std::str::from_utf8(&message) {
            Ok(message) => info!("Received message: {message:?}"),
            Err(e) => error!("Failed to convert message to string: {e}"),
        }
    }
}