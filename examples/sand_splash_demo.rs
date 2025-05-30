//! Demonstrates the sand splash animation feature.
//!
//! This example showcases how to configure and use the optional sand splash animation
//! that creates particles around where sand hits the bottom of the hourglass.

use bevy::prelude::*;
use bevy_hourglass::{
    components::SandSplashConfig, 
    spawn_mesh_hourglass_with_timer, 
    HourglassMeshBuilder, 
    HourglassMeshBodyConfig, 
    HourglassMeshPlatesConfig, 
    HourglassMeshSandConfig, 
    HourglassPlugin
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

    // Spawn text label for basic hourglass
    commands.spawn((
        Text::new("Basic\n(No Splash)"),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_translation(Vec3::new(-300.0, -150.0, 0.0)),
    ));

    // Hourglass with default sand splash
    let default_splash_config = SandSplashConfig {
        enabled: true,
        ..Default::default()
    };

    HourglassMeshBuilder::new(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)))
        .with_body(HourglassMeshBodyConfig::default())
        .with_plates(HourglassMeshPlatesConfig::default())
        .with_sand(HourglassMeshSandConfig::default())
        .with_sand_splash(default_splash_config)
        .with_timing(10.0)
        .build(&mut commands, &mut meshes, &mut materials);

    // Spawn text label for default splash
    commands.spawn((
        Text::new("Default Splash"),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_translation(Vec3::new(0.0, -150.0, 0.0)),
    ));

    // Hourglass with custom sand splash configuration
    let custom_splash_config = SandSplashConfig {
        enabled: true,
        splash_radius: 35.0,
        particle_count: 12,
        particle_duration: 0.8,
        spawn_interval: 0.05,
        particle_color: Color::srgb(1.0, 0.4, 0.1), // Bright orange
        particle_size: 3.0,
    };

    HourglassMeshBuilder::new(Transform::from_translation(Vec3::new(300.0, 0.0, 0.0)))
        .with_body(HourglassMeshBodyConfig::default())
        .with_plates(HourglassMeshPlatesConfig::default())
        .with_sand(HourglassMeshSandConfig::default())
        .with_sand_splash(custom_splash_config)
        .with_timing(10.0)
        .build(&mut commands, &mut meshes, &mut materials);

    // Spawn text label for custom splash
    commands.spawn((
        Text::new("Custom Splash\n(Orange, Larger)"),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_translation(Vec3::new(300.0, -150.0, 0.0)),
    ));

    // Instructions
    commands.spawn((
        Text::new("Sand Splash Demo\nWatch the particles appear where sand hits the bottom!"),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_translation(Vec3::new(0.0, 250.0, 0.0)),
    ));
}