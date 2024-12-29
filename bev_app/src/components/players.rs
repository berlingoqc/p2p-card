use bevy::prelude::*;
use rand::rngs::StdRng;

use crate::{network::NetworkState, resource::MyPlayerResource, utils::{get_random_color, get_random_shape}};

const X_EXTENT: f32 = 900.;

#[derive(Component)]
pub struct MyPlayer;


#[derive(Component)]
pub struct Player {
    pub name: String,
    pub rng: StdRng,
    pub network_state: NetworkState,
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
        Transform::from_xyz(-1_f32 * X_EXTENT / 2.0, 0.0, 0.0),
        MyPlayer{},
        Player{ rng: my_player.rng.clone(), name: my_player.player.name.clone(), network_state: NetworkState::Connected }
    ));
}


pub fn add_other_player(
    commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,

    position: Transform,
    mut my_player: Player,
) {
    let shape = get_random_shape(1.0, &mut my_player.rng, &mut meshes);
    let color = get_random_color(&mut my_player.rng);

    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(color)),
        Text2d::new(my_player.name.clone()),
        position,
        Player{ rng: my_player.rng.clone(), name: my_player.name.clone(), network_state: NetworkState::Connected }
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



