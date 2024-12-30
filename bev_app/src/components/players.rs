use bevy::prelude::*;
use game::logic::players::OtherPlayer;
use protocol::generated::client::ConnectionState;
use rand::rngs::StdRng;

use crate::{resource::MyPlayerResource, utils::{get_random_color, get_random_shape}};

const X_EXTENT: f32 = 900.;


#[derive(Component)]
pub struct Connected;

#[derive(Component)]
pub struct Disconnected;

#[derive(Component)]
pub struct MyPlayer;


#[derive(Component)]
pub struct Player {
    pub name: String,
    pub rng: StdRng,
    pub network_state: ConnectionState,
}


pub fn setup_my_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut my_player: ResMut<MyPlayerResource>
) {


    let shape = get_random_shape(1.0, &mut my_player.rng, &mut meshes);
    let color = get_random_color(&mut my_player.rng);

    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(color)),
        Text2d::new(my_player.player.name.clone()),
        Transform::from_xyz(my_player.player.position[0], my_player.player.position[1], my_player.player.position[2]),
        MyPlayer{},
        Player{ rng: my_player.rng.clone(), name: my_player.player.name.clone(), network_state: ConnectionState::Connected },
        Connected{},
    ));
}


pub fn add_other_player(
    commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,

    position: &[f32; 3],
    mut other_player: Player,
) {
    let shape = get_random_shape(1.0, &mut other_player.rng, &mut meshes);
    let color = get_random_color(&mut other_player.rng);

    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(color)),
        Text2d::new(other_player.name.clone()),
        Transform::from_xyz(position[0], position[1], position[2]),
        other_player,
        Connected{},
    ));

}

pub fn remove_other_player(
    commands: &mut Commands,

    name: String,

    players: &Query<(Entity, &Player)>,
) {

    for player in players.iter() {

        if player.1.name.eq(&name) {

            commands.entity(player.0).despawn_recursive();
            return;
        }
    }
}



