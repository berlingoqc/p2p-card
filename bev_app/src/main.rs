mod resource;
mod network;
mod arg_parser;

use resource::MyPlayerResource;
use bevy::{prelude::*, time::common_conditions::on_timer, utils::Duration};
use game::logic::players::MyPlayer;

fn main() {

    let user_config = arg_parser::load_my_player_config().unwrap();
    let my_player = MyPlayer::load(user_config);

    let my_player_resource = MyPlayerResource {
        player: my_player,
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, network::start_socket)
        .add_systems(Update, network::receive_messages)
        .add_systems(
            Update,
            network::send_message.run_if(on_timer(Duration::from_secs(5))),
        )
        .insert_resource(my_player_resource)
        .run();
}