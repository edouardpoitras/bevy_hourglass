//! Example of the mesh-based hourglass implementations with automatic animation.

use bevy::prelude::*;
use bevy_hourglass::{
    HourglassMeshBodyConfig, HourglassMeshBuilder, HourglassMeshPlatesConfig,
    HourglassMeshSandConfig, HourglassPlugin,
};
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HourglassPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());

    // Create an hourglass with body, plates, and automatic timing using the builder pattern
    HourglassMeshBuilder::new(Transform::from_xyz(0.0, 0.0, 0.0))
        .with_body(HourglassMeshBodyConfig {
            total_height: 200.0,
            bulb_radius: 100.0,
            bulb_width_factor: 0.75,
            bulb_height_factor: 1.0,
            bulb_curve_resolution: 20,
            neck_width: 12.0,
            neck_height: 7.0,
            neck_curve_resolution: 5,
            color: Color::srgba(0.85, 0.95, 1.0, 0.2),
        })
        .with_plates(HourglassMeshPlatesConfig {
            width: 165.0,
            height: 10.0,
            color: Color::srgb(0.6, 0.4, 0.2),
        })
        .with_sand(HourglassMeshSandConfig {
            color: Color::srgb(0.9, 0.8, 0.6),
            fill_percent: 1.0,       // Start with full top bulb
            scale_factor: 0.95,      // Sand is 95% of glass size
            neck_scale_factor: 0.35, // Sand is 35% of neck size
        })
        .with_timing(Duration::from_secs(30)) // 30-second timer for automatic animation
        .build(&mut commands, &mut meshes, &mut materials);
}
