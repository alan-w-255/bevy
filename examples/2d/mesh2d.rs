//! Shows how to render a polygonal [`Mesh`], generated from a [`Rectangle`] primitive, in a 2D scene.

use std::f32::consts::PI;

use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::{
    color::palettes::basic::PURPLE, input::common_conditions::input_just_pressed,
    platform::collections::HashMap, prelude::*,
};
use bevy_asset::RenderAssetUsages;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            input_system.run_if(
                input_just_pressed(KeyCode::ArrowUp).or(input_just_pressed(KeyCode::ArrowDown)),
            ),
        )
        .run();
}

/// Store the image handle that we will draw to, here.
#[derive(Resource)]
struct PolyMeshMap(HashMap<u32, Handle<Mesh>>);

#[derive(Component)]
struct PolygonSideCount(u32);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.insert_resource(PolyMeshMap(HashMap::new()));

    commands.spawn((
        PolygonSideCount(3u32),
        Mesh2d(meshes.add(RegularPolygon::new(2.5, 3u32).to_ring(1.0))),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Transform::default().with_scale(Vec3::splat(128.)),
    ));
}

fn input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut poly_map: ResMut<PolyMeshMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Single<(&mut Mesh2d, &mut PolygonSideCount)>,
) {
    let (mut mesh, mut sides) = query.into_inner();
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        sides.0 -= if sides.0 > 3 { 1 } else { 0 };
    } else if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        sides.0 += 1;
    } else {
        return;
    }
    if let Some(mesh_handle) = poly_map.0.get(&sides.0) {
        mesh.0 = mesh_handle.clone();
    } else {
        let mesh_handle = meshes.add(gen_polygon_ring_mesh(sides.0));
        poly_map.0.insert(sides.0, mesh_handle.clone());
        mesh.0 = mesh_handle;
    }
}

fn gen_polygon_ring_mesh(side_count: u32) -> Mesh {
    let mut poly_mesh = Mesh::new(
        PrimitiveTopology::TriangleStrip,
        RenderAssetUsages::RENDER_WORLD,
    );
    let mut vertices = vec![];
    let outer_r = 2.0f32;
    let inner_r = 1.7f32;
    for i in 0..side_count {
        let a = i as f32 * PI * 2.0 / side_count as f32;
        vertices.push([inner_r * ops::cos(a), inner_r * ops::sin(a), 0.0]);
        vertices.push([outer_r * ops::cos(a), outer_r * ops::sin(a), 0.0]);
    }
    poly_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    let mut indices = vec![];
    for i in 0..side_count * 2 - 2 {
        indices.extend_from_slice(&[i, i + 1, i + 2]);
    }
    poly_mesh.insert_indices(Indices::U32(indices));
    return poly_mesh;
}
