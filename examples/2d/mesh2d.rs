//! Shows how to render a polygonal [`Mesh`], generated from a [`Rectangle`] primitive, in a 2D scene.

use bevy::{color::palettes::basic::PURPLE, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, input_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(RegularPolygon::new(1.0, 8))),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Transform::default().with_scale(Vec3::splat(128.)),
    ));
}

#[derive(Resource, Default)]
struct Polygon();

fn input_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<Entity, With<Mesh2d>>,
) {
    for (entity) in query {
        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            println!("down");
            commands
                .entity(entity)
                .insert(Mesh2d(meshes.add(RegularPolygon::new(1.0, 7))));
        }
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            println!("up");
        }
    }
}
