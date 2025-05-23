//! Example of the mesh-based hourglass implementations with animation.

use bevy::prelude::*;
use bevy_hourglass::{
    update_sand_fill_percent, HourglassMesh, HourglassMeshBodyConfig, HourglassMeshBuilder,
    HourglassMeshPlatesConfig, HourglassMeshSandConfig, HourglassMeshSandState, HourglassPlugin,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HourglassPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, animate_hourglass)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());

    // Create an hourglass with body and plates using the builder pattern
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
            color: Color::srgb(0.8, 0.5, 0.3),
        })
        .with_plates(HourglassMeshPlatesConfig {
            width: 165.0,
            height: 10.0,
            color: Color::srgb(0.0, 0.0, 0.0),
        })
        .with_sand(HourglassMeshSandConfig {
            color: Color::srgb(0.9, 0.8, 0.6),
            fill_percent: 0.75,      // 75% of sand in top bulb, 25% in bottom
            scale_factor: 0.95,      // Sand is 95% of glass size
            neck_scale_factor: 0.35, // Sand is 35% of neck size
        })
        .build(&mut commands, &mut meshes, &mut materials);
}

/// System to animate the hourglass sand over time
fn animate_hourglass(
    time: Res<Time>,
    mut sand_query: Query<&mut HourglassMeshSandState, With<HourglassMesh>>,
) {
    for mut sand_state in sand_query.iter_mut() {
        // Create a simple animation that cycles the fill percentage over 5 seconds
        let cycle_time = 5.0; // seconds
        let elapsed = time.elapsed_secs() % cycle_time;
        let t = elapsed / cycle_time;
        
        // Create a smooth back-and-forth animation
        let fill_percent = if t < 0.5 {
            // First half: 1.0 to 0.0
            1.0 - (t * 2.0)
        } else {
            // Second half: 0.0 to 1.0
            (t - 0.5) * 2.0
        };
        
        update_sand_fill_percent(&mut sand_state, fill_percent);
    }
}
