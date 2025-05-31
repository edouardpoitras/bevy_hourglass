//! Loading animation demo with hourglass and animated text.

use bevy::prelude::*;
use bevy_hourglass::{
    BulbStyle, HourglassMeshBodyConfig, HourglassMeshBuilder, HourglassMeshPlatesConfig,
    HourglassMeshSandConfig, HourglassPlugin, NeckStyle,
};
use std::time::Duration;

#[derive(Component)]
struct LoadingText;

#[derive(Component)]
struct LoadingDot {
    delay: f32,
    timer: Timer,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HourglassPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, animate_loading_text)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Black background
    commands.spawn(Camera2d::default());
    commands.insert_resource(ClearColor(Color::BLACK));

    // Create a white hourglass with quick timing for loading effect
    HourglassMeshBuilder::new(Transform::from_xyz(0.0, 0.0, 0.0))
        .with_body(HourglassMeshBodyConfig {
            total_height: 150.0,
            bulb_style: BulbStyle::Circular {
                curvature: 1.0,
                width_factor: 1.0,
                curve_resolution: 20,
            },
            neck_style: NeckStyle::Curved {
                curvature: 1.0,
                width: 10.0,
                height: 15.0,
                curve_resolution: 10,
            },
            color: Color::srgba(1.0, 1.0, 1.0, 0.3), // White glass
        })
        .with_plates(HourglassMeshPlatesConfig {
            width: 140.0,
            height: 8.0,
            color: Color::WHITE, // White plates
        })
        .with_sand(HourglassMeshSandConfig {
            color: Color::WHITE, // White sand
            fill_percent: 1.0,
            wall_offset: 3.0,
            bottom_mound_factor: 0.1, // Subtle mound effect
        })
        .with_timing(1.5) // Quick 2-second timer for loading effect
        .with_flip_duration(0.5)
        .with_auto_flip(true)
        .build(&mut commands, &mut meshes, &mut materials);

    // Loading text
    commands.spawn((
        Text2d::new("Loading"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(-18.0, -122.0, 1.0),
        LoadingText,
    ));

    // Animated dots
    for i in 0..3 {
        commands.spawn((
            Text2d::new("."),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(58.0 + (i as f32 * 15.0), -129.0, 1.0),
            LoadingDot {
                delay: i as f32 * 0.2,
                timer: Timer::from_seconds(0.8, TimerMode::Repeating),
            },
        ));
    }
}

fn animate_loading_text(time: Res<Time>, mut dots: Query<(&mut TextColor, &mut LoadingDot)>) {
    for (mut color, mut dot) in dots.iter_mut() {
        dot.timer.tick(time.delta());

        if dot.timer.just_finished() {
            // Reset timer with delay
            let delay = dot.delay;
            dot.timer.reset();
            dot.timer.tick(Duration::from_secs_f32(delay));
        }

        // Calculate alpha based on timer progress
        let progress = dot.timer.elapsed_secs() / dot.timer.duration().as_secs_f32();
        let alpha = if progress < 0.5 {
            progress * 2.0
        } else {
            2.0 - (progress * 2.0)
        };

        color.0 = Color::srgba(1.0, 1.0, 1.0, alpha.clamp(0.2, 1.0));
    }
}
