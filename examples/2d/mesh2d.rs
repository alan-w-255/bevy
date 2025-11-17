//! Shows how to render a polygonal [`Mesh`], generated from a [`Rectangle`] primitive, in a 2D scene.

use bevy::{color::palettes::basic::PURPLE, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, input_system)
        .run();
}

#[derive(Component)]
struct PolygonSideCount(u32);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        PolygonSideCount(3u32),
        Mesh2d(meshes.add(RegularPolygon::new(1.0, 3u32))),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Transform::default().with_scale(Vec3::splat(128.)),
    ));
}

fn input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Single<(&mut Mesh2d, &mut PolygonSideCount)>,
) {
    let (mut mesh, mut sides) = query.into_inner();
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        sides.0 -= if sides.0 > 3 { 1 } else { 0 };
        meshes.remove(mesh.0.id());
        mesh.0 = meshes.add(RegularPolygon::new(1.0, sides.0));
        println!("down");
    } else if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        sides.0 += 1;
        mesh.0 = meshes.add(RegularPolygon::new(1.0, sides.0));
        println!("up");
    }
}
