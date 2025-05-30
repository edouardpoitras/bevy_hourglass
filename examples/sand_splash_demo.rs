//! Demonstrates the sand splash animation feature.
//!
//! This example showcases how to configure and use the optional sand splash animation
//! that creates particles around where sand hits the bottom of the hourglass.

use bevy::prelude::*;
use bevy_hourglass::{
    spawn_mesh_hourglass_with_timer, HourglassMeshBodyConfig, HourglassMeshBuilder,
    HourglassMeshPlatesConfig, HourglassMeshSandConfig, HourglassPlugin, SandSplashConfig,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HourglassPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn camera
    commands.spawn(Camera2d);

    // Basic hourglass without sand splash (for comparison)
    spawn_mesh_hourglass_with_timer(
        &mut commands,
        &mut meshes,
        &mut materials,
        10.0, // 10 seconds
        Vec3::new(-300.0, 0.0, 0.0),
    );

    // Hourglass with default sand splash
    let default_splash_config = SandSplashConfig::default();

    HourglassMeshBuilder::new(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)))
        .with_body(HourglassMeshBodyConfig::default())
        .with_plates(HourglassMeshPlatesConfig::default())
        .with_sand(HourglassMeshSandConfig::default())
        .with_sand_splash(default_splash_config)
        .with_timing(10.0)
        .build(&mut commands, &mut meshes, &mut materials);

    // Hourglass with custom sand splash configuration
    let custom_splash_config = SandSplashConfig {
        splash_radius: 25.0,
        particle_count: 10,
        particle_duration: 0.8,
        spawn_interval: 0.05,
        particle_color: Color::srgb(0.0, 0.0, 0.0), // Large black particles
        particle_size: 3.0,
        vertical_offset: 5.0,
    };

    HourglassMeshBuilder::new(Transform::from_translation(Vec3::new(300.0, 0.0, 0.0)))
        .with_body(HourglassMeshBodyConfig::default())
        .with_plates(HourglassMeshPlatesConfig::default())
        .with_sand(HourglassMeshSandConfig::default())
        .with_sand_splash(custom_splash_config)
        .with_timing(10.0)
        .build(&mut commands, &mut meshes, &mut materials);
}
