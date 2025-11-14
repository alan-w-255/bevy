//! Shows how to render a polygonal [`Mesh`], generated from a [`Rectangle`] primitive, in a 2D scene.

use bevy::{color::palettes::basic::PURPLE, mesh::VertexAttributeValues, prelude::*};

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

fn input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Single<&Mesh2d>,
) {
    let Some(mesh) = meshes.get_mut(*query) else {
        return;
    };
    let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    else {
        return;
    };
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        println!("down");
        for position in positions {
            position[0] /= 2.0;
            position[1] /= 2.0;
            position[2] /= 2.0;
        }
    } else if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        println!("up");
        for position in positions {
            position[0] *= 2.0;
            position[1] *= 2.0;
            position[2] *= 2.0;
        }
    }
}
