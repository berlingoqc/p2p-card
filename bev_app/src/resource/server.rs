use bevy::prelude::*;
use game::logic::deck::encryption::generate_random_nonce;
use rand::rngs::OsRng;

use super::MyPlayerResource;

#[derive(Resource)]
pub struct SelectedMatchboxServer {
    pub url: String,
}

#[derive(Resource)]
pub struct SelectedRoom {
    pub name: String,
}

impl SelectedRoom {
    pub fn create_my_room(my_player: &MyPlayerResource) -> Self {
        let mut rng = OsRng;
        let uniqute_name = generate_random_nonce();
        let unique_name = String::from_utf8_lossy(&uniqute_name);
        Self {
            name: format!("{}-{}", my_player.player.name, unique_name),
        }
    }
}

use base64::{engine::general_purpose::URL_SAFE, Engine as _};

pub fn generate_rool_url(
    server: &SelectedMatchboxServer,
    room: &SelectedRoom,
    my_player: &MyPlayerResource,
) -> String {
    format!(
        "{}/{}?name={}&hash={}&pub_key={}",
        server.url,
        room.name,
        my_player.player.name,
        my_player.player.hash,
        URL_SAFE.encode(my_player.player.pub_key.as_bytes())
    )
}
