//! Demonstrates visual flip capabilities for 2D mesh hourglasses

use bevy::prelude::*;
use bevy_hourglass::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HourglassPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2d);

    // Create an hourglass with 5 second timer and 0.5 second flip animation
    spawn_mesh_hourglass_with_flip(
        &mut commands,
        &mut meshes,
        &mut materials,
        Duration::from_secs(5),
        Vec3::new(-150.0, 0.0, 0.0),
        0.25,  // flip duration
        false, // don't auto-flip
    );

    // Create an auto-flipping hourglass
    spawn_mesh_hourglass_with_flip(
        &mut commands,
        &mut meshes,
        &mut materials,
        Duration::from_secs(3),
        Vec3::new(150.0, 0.0, 0.0),
        0.5,  // flip duration
        true, // auto-flip when empty
    );

    // Instructions
    commands.spawn((
        Text::new("Press SPACE to flip the left hourglass\nRight hourglass auto-flips when empty"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Hourglass, With<HourglassMesh>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // Find the first (left) hourglass and flip it if possible
        if let Some(mut hourglass) = query.iter_mut().next() {
            if hourglass.can_flip() {
                hourglass.flip();
            }
        }
    }
}
