use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use bevy::prelude::*;

pub fn get_random_shape(
    scale: f32,
    rng: &mut StdRng,
    mut meshes: &mut ResMut<Assets<Mesh>>,
) -> bevy::prelude::Handle<bevy::prelude::Mesh> {
    let i = rng.gen_range(1..10);
    match i {
        1 => meshes.add(Circle::new(50.0)),
        2 => meshes.add(CircularSector::new(50.0, 1.0)),
        3 => meshes.add(CircularSegment::new(50.0, 1.25)),
        4 => meshes.add(Ellipse::new(25.0, 50.0)),
        5 => meshes.add(Annulus::new(25.0, 50.0)),
        6 => meshes.add(Capsule2d::new(25.0, 50.0)),
        7 => meshes.add(Rhombus::new(75.0, 100.0)),
        9 => meshes.add(Rectangle::new(50.0, 100.0)),
        10 => meshes.add(RegularPolygon::new(50.0, 6)),
        _ => meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        )),
    }
}

pub fn get_random_color(rng: &mut StdRng) -> Color {
    let mut r = StdRng::seed_from_u64(42);

    Color::hsl(r.gen_range(0.0f32..360.0f32), 0.95, 0.7)
}

pub fn vec3_to_vec_i32(v: &Vec3) -> Vec<i32> {
    vec![v.x as i32, v.y as i32, v.z as i32]
}

pub fn veci32_to_i32(v: &Vec<i32>) -> [f32; 3] {
    [
        *v.get(0).unwrap() as f32,
        *v.get(1).unwrap() as f32,
        *v.get(2).unwrap() as f32,
    ]
}

pub fn i32_to_veci32(v: &[f32; 3]) -> Vec<i32> {
    vec![v[0] as i32, v[1] as i32, v[2] as i32]
}
