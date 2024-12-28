use bevy::prelude::*;

use game::logic::players::MyPlayer;



#[derive(Resource)]
pub struct MyPlayerResource {
    pub player: MyPlayer
}
