//! Example of the mesh-based hourglass implementations with automatic animation.

use bevy::prelude::*;
use bevy_hourglass::{
    BulbStyle, HourglassMeshBodyConfig, HourglassMeshBuilder, HourglassMeshPlatesConfig,
    HourglassMeshSandConfig, HourglassPlugin, NeckStyle, SandSplashConfig,
};

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
            bulb_style: BulbStyle::Circular {
                curvature: 1.0,
                width_factor: 1.0,
                curve_resolution: 20,
            },
            neck_style: NeckStyle::Curved {
                curvature: 1.0,
                width: 14.0,
                height: 20.0,
                curve_resolution: 10,
            },
            color: Color::srgba(0.85, 0.95, 1.0, 0.2),
        })
        .with_plates(HourglassMeshPlatesConfig {
            width: 200.0,
            height: 10.0,
            color: Color::srgb(0.6, 0.4, 0.2),
        })
        .with_sand(HourglassMeshSandConfig {
            color: Color::srgb(0.9, 0.8, 0.6),
            fill_percent: 1.0,
            wall_offset: 4.0,
        })
        .with_sand_splash(SandSplashConfig::default())
        .with_timing(10.0) // 10-second timer for automatic animation
        .build(&mut commands, &mut meshes, &mut materials);
}
