//! Demonstrates setting sand and splash color dynamically.
//!
//! This example showcases how to change the color of the sand and splash particles
//! over time, creating a rainbow effect as the hourglass runs.

use bevy::prelude::*;
use bevy_hourglass::{
    Hourglass, HourglassMeshBodyConfig, HourglassMeshBuilder, HourglassMeshPlatesConfig,
    HourglassMeshSandConfig, HourglassPlugin, SandSplash, SandSplashConfig,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HourglassPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_rainbow_color)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn camera
    commands.spawn(Camera2d);

    // Hourglass with custom sand splash configuration
    let custom_splash_config = SandSplashConfig {
        splash_radius: 25.0,
        particle_count: 10,
        particle_duration: 0.8,
        spawn_interval: 0.05,
        particle_color: Color::default(),
        particle_size: 3.0,
        vertical_offset: 5.0,
    };

    HourglassMeshBuilder::new(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)))
        .with_body(HourglassMeshBodyConfig::default())
        .with_plates(HourglassMeshPlatesConfig::default())
        .with_sand(HourglassMeshSandConfig::default())
        .with_sand_splash(custom_splash_config)
        .with_timing(10.0)
        .build(&mut commands, &mut meshes, &mut materials);
}

fn update_rainbow_color(
    time: Res<Time>,
    mut hourglass_query: Query<(&mut Hourglass, Option<&mut SandSplash>)>,
) {
    // Cycle through hue over time (0-360 degrees)
    let hue = (time.elapsed_secs() * 60.0) % 360.0; // Complete cycle every 6 seconds

    // Create rainbow color using HSL (saturation = 1.0, lightness = 0.5 for vibrant colors)
    let rainbow_color = Color::hsla(hue, 1.0, 0.5, 1.0);

    // Update sand color for all hourglasses
    for (mut hourglass, sand_splash) in hourglass_query.iter_mut() {
        // Update the hourglass sand color
        hourglass.sand_color = rainbow_color;

        // Update the sand splash particle color if the entity has a SandSplash component
        if let Some(mut splash) = sand_splash {
            splash.config.particle_color = rainbow_color;
        }
    }
}
