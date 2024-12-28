pub mod server;

use bevy::prelude::*;

use game::logic::players::MyPlayer;
use rand::rngs::StdRng;


#[derive(Resource)]
pub struct MyPlayerResource {
    pub player: MyPlayer,
    pub rng: StdRng,
}

