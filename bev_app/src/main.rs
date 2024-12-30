mod resource;
mod components;
mod network;
mod arg_parser;
mod utils;
mod system;

use components::players::setup_my_player;
use network::PlayerConnectionEvent;
use rand::{rngs::StdRng, SeedableRng};
use resource::MyPlayerResource;
use bevy::{prelude::*, time::common_conditions::on_timer, utils::Duration};
use game::logic::players::MyPlayer;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {

    let (user_config, selected_server, selected_room) = arg_parser::load_config().unwrap();
    let my_player = MyPlayer::load(user_config);

    let my_player_resource = MyPlayerResource {
        rng: StdRng::seed_from_u64(my_player.hash),
        player: my_player,
    };


    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy-canvas".into()),
                title: "p2p-card".to_string(),
                ..default()
            }
            ),
            ..default()
        }).set(ImagePlugin::default_nearest()))

        .insert_resource(selected_server)
        .insert_resource(selected_room)

        .add_event::<PlayerConnectionEvent>()

        .add_systems(Startup, (
            network::start_socket, setup_camera, setup_my_player,
        ))
        .add_systems(Update, (
            network::receive_messages,
            system::player::system_player_connection_event,
        ))
        .insert_resource(my_player_resource)

        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.6)))

        .add_plugins(WorldInspectorPlugin::new())


        .run();
}


fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2d {
        ..Default::default()
    });
}