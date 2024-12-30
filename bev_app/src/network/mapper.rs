use game::{logic::players::{MyPlayer, OtherPlayer}, utils::get_pub_key_from_vec};
use protocol::generated::msg::Presentation;

use crate::{resource::MyPlayerResource, utils::{i32_to_veci32, veci32_to_i32}};



pub fn presentation_to_other_player(presentation: &Presentation) -> OtherPlayer {

    let pub_key = get_pub_key_from_vec(presentation.pub_key.clone()).unwrap();
    let profile_public_key = presentation.profile_pub_key.clone().map(|d| get_pub_key_from_vec(d).unwrap()).or_else(|| Some(pub_key.clone())).unwrap();

    OtherPlayer {
        hash: presentation.hash,
        name: presentation.name.clone(),
        profile_public_key: profile_public_key,
        pub_key: pub_key,
        position: veci32_to_i32(&presentation.positions),
    }
    
}

pub fn my_player_to_presentation(my_player: &MyPlayerResource) -> Presentation {
    Presentation {
        hash: my_player.player.hash,
        name: my_player.player.name.clone(),
        profile_pub_key: Some(my_player.player.profile_public_key.to_bytes().to_vec()),
        pub_key: my_player.player.pub_key.to_bytes().to_vec(),
        positions: i32_to_veci32(&my_player.player.position),
    }
}