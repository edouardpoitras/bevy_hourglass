//! Demonstration of the new composable curve system for creating different hourglass styles.

use bevy::prelude::*;
use bevy_hourglass::*;

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
    commands.spawn(Camera2d::default());

    // Create different hourglass styles to demonstrate the composable curve system

    // 1. Classic curved hourglass (default style)
    spawn_styled_mesh_hourglass(
        &mut commands,
        &mut meshes,
        &mut materials,
        10.0,
        Vec3::new(-400.0, 0.0, 0.0),
        BulbStyle::Circular {
            curvature: 1.0,
            width_factor: 0.75,
            curve_resolution: 20,
        },
        NeckStyle::Curved {
            curvature: 0.2,
            width: 12.0,
            height: 8.0,
            curve_resolution: 5,
        },
    );

    // 2. Straight-sided hourglass (triangular shape)
    spawn_styled_mesh_hourglass(
        &mut commands,
        &mut meshes,
        &mut materials,
        10.0,
        Vec3::new(-200.0, 0.0, 0.0),
        BulbStyle::Straight { width_factor: 0.75 },
        NeckStyle::Straight {
            width: 12.0,
            height: 8.0,
        },
    );

    // 3. Slightly curved hourglass with straight neck
    spawn_styled_mesh_hourglass(
        &mut commands,
        &mut meshes,
        &mut materials,
        10.0,
        Vec3::new(0.0, 0.0, 0.0),
        BulbStyle::Circular {
            curvature: 0.5,
            width_factor: 0.75,
            curve_resolution: 20,
        },
        NeckStyle::Straight {
            width: 12.0,
            height: 8.0,
        },
    );

    // 4. Highly curved hourglass with curved neck
    spawn_styled_mesh_hourglass(
        &mut commands,
        &mut meshes,
        &mut materials,
        10.0,
        Vec3::new(200.0, 0.0, 0.0),
        BulbStyle::Circular {
            curvature: 3.0,
            width_factor: 0.75,
            curve_resolution: 20,
        },
        NeckStyle::Curved {
            curvature: 1.0,
            width: 12.0,
            height: 8.0,
            curve_resolution: 5,
        },
    );

    // 5. Custom hourglass with different configurations
    HourglassMeshBuilder::new(Transform::from_translation(Vec3::new(400.0, 0.0, 0.0)))
        .with_body(HourglassMeshBodyConfig {
            total_height: 150.0,
            bulb_style: BulbStyle::Circular {
                curvature: 1.0,
                width_factor: 0.8,
                curve_resolution: 20,
            },
            neck_style: NeckStyle::Curved {
                curvature: 0.1,
                width: 8.0,
                height: 16.0,
                curve_resolution: 10,
            },
            color: Color::srgba(1.0, 0.7, 0.8, 0.3), // Pink glass
        })
        .with_plates(HourglassMeshPlatesConfig {
            width: 150.0,
            height: 8.0,
            color: Color::srgb(0.4, 0.2, 0.6), // Purple plates
        })
        .with_sand(HourglassMeshSandConfig {
            color: Color::srgb(1.0, 0.9, 0.5), // Light yellow sand
            fill_percent: 0.5,                 // Start with half-filled top bulb
            wall_offset: 5.0,                  // Sand is 5 pixels offset from glass wall
            bottom_mound_factor: 0.4,          // Nice mound effect for demonstration
        })
        .with_timing(10.0)
        .with_auto_flip(true)
        .with_flip_duration(0.5)
        .build(&mut commands, &mut meshes, &mut materials);

    // Add labels to show the different styles
    add_style_labels(&mut commands);
}

fn add_style_labels(commands: &mut Commands) {
    let labels = [
        ("Classic Curved", Vec3::new(-400.0, -150.0, 0.0)),
        ("Straight Sided", Vec3::new(-200.0, -150.0, 0.0)),
        ("Subtle Curves", Vec3::new(0.0, -150.0, 0.0)),
        ("High Curvature", Vec3::new(200.0, -150.0, 0.0)),
        ("Custom Style", Vec3::new(400.0, -150.0, 0.0)),
    ];

    for (label, position) in labels {
        commands.spawn((
            Text2d::new(label),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor::from(Color::WHITE),
            Transform::from_translation(position),
        ));
    }

    // Add title
    commands.spawn((
        Text2d::new("Composable Curve System - Different Hourglass Styles"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor::from(Color::srgb(1.0, 1.0, 0.0)), // Yellow
        Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
    ));
}
