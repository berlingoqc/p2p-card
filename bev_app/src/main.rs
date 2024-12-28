mod resource;
mod components;
mod network;
mod arg_parser;
mod utils;

use rand::{rngs::StdRng, SeedableRng};
use resource::{server::{SelectedMatchboxServer, SelectedRoom}, MyPlayerResource};
use bevy::{prelude::*, time::common_conditions::on_timer, utils::Duration};
use game::logic::players::MyPlayer;

fn main() {

    let (user_config) = arg_parser::load_config().unwrap();
    let my_player = MyPlayer::load(user_config);

    let my_player_resource = MyPlayerResource {
        rng: StdRng::seed_from_u64(my_player.hash),
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

        .insert_resource(SelectedMatchboxServer {
            url: "ws://localhost".to_string(),
        })
        .insert_resource(SelectedRoom::create_my_room(&my_player_resource))

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