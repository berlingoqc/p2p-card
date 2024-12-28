mod resource;
mod components;
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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy-canvas".into()),
                ..default()
            }
            ),
            ..default()
        }).set(ImagePlugin::default_nearest()))

        .add_systems(Startup, (network::start_socket, setup_camera))
        .add_systems(Update, network::receive_messages)
        .add_systems(
            Update,
            network::send_message.run_if(on_timer(Duration::from_secs(5))),
        )
        .insert_resource(my_player_resource)


        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.6)))
        .run();
}


fn setup_camera(
    mut commands: Commands,
) {
    // configure the background color (if any), for a specific camera (2D)
    commands.spawn(Camera2d {
        ..Default::default()
    });
}