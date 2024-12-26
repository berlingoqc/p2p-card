use bevy::prelude::*;

use crate::logic::players::MyPlayer;




#[derive(Resource)]
pub struct MyPlayerResource {
    pub player: MyPlayer
}