//! Auto-flipping hourglass mayhem - lots of hourglasses all over the screen!
//! Each hourglass has random colors, sizes, shapes, and flip durations.

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

    // Create lots of hourglasses with random configurations
    let grid_cols = 6;
    let grid_rows = 5;
    let num_hourglasses = grid_cols * grid_rows;

    for i in 0..num_hourglasses {
        let seed = i as f32;

        // Use grid-based positioning with some randomness to avoid perfect alignment
        let col = i % grid_cols;
        let row = i / grid_cols;

        let base_x = (col as f32 - (grid_cols as f32 - 1.0) / 2.0) * 180.0;
        let base_y = (row as f32 - (grid_rows as f32 - 1.0) / 2.0) * 120.0 - 50.0; // Offset down from text

        // Add random offset to break perfect grid alignment
        let random_x_offset = ((seed * 73.0) % 60.0) - 30.0; // -30 to 30
        let random_y_offset = ((seed * 127.0) % 40.0) - 20.0; // -20 to 20

        let x = base_x + random_x_offset;
        let y = base_y + random_y_offset;
        let position = Vec3::new(x, y, 0.0);

        // Random size scaling
        let scale = 0.3 + ((seed * 43.0) % 100.0) / 100.0 * 0.7; // 0.3 to 1.0

        // Random timer duration between 1 to 10 seconds
        let flip_duration = 1.0 + ((seed * 89.0) % 100.0) / 100.0 * 9.0;

        // Random colors
        let glass_hue = (seed * 37.0) % 360.0;
        let sand_hue = (seed * 113.0) % 360.0;
        let plate_hue = (seed * 197.0) % 360.0;

        let glass_color = Color::hsla(glass_hue, 0.7, 0.8, 0.3);
        let sand_color = Color::hsla(sand_hue, 0.8, 0.6, 1.0);
        let plate_color = Color::hsla(plate_hue, 0.6, 0.4, 1.0);

        // Random hourglass height
        let height = 80.0 + ((seed * 157.0) % 100.0) / 100.0 * 120.0; // 80 to 200

        // Random bulb style
        let bulb_style = match ((seed * 23.0) as u32) % 3 {
            0 => BulbStyle::Straight {
                width_factor: 0.6 + ((seed * 61.0) % 100.0) / 100.0 * 0.3, // 0.6 to 0.9
            },
            1 => BulbStyle::Circular {
                curvature: 0.3 + ((seed * 139.0) % 100.0) / 100.0 * 2.7, // 0.3 to 3.0
                width_factor: 0.6 + ((seed * 71.0) % 100.0) / 100.0 * 0.3, // 0.6 to 0.9
                curve_resolution: 15 + (((seed * 31.0) as u32) % 15) as usize, // 15 to 30
            },
            _ => BulbStyle::Circular {
                curvature: 1.0,
                width_factor: 0.75,
                curve_resolution: 20,
            },
        };

        // Random neck style
        let neck_style = match ((seed * 47.0) as u32) % 2 {
            0 => NeckStyle::Straight {
                width: 6.0 + ((seed * 83.0) % 100.0) / 100.0 * 12.0, // 6 to 18
                height: 4.0 + ((seed * 97.0) % 100.0) / 100.0 * 12.0, // 4 to 16
            },
            _ => NeckStyle::Curved {
                curvature: 0.1 + ((seed * 179.0) % 100.0) / 100.0 * 0.9, // 0.1 to 1.0
                width: 6.0 + ((seed * 149.0) % 100.0) / 100.0 * 12.0,    // 6 to 18
                height: 4.0 + ((seed * 163.0) % 100.0) / 100.0 * 12.0,   // 4 to 16
                curve_resolution: 3 + (((seed * 53.0) as u32) % 8) as usize, // 3 to 10
            },
        };

        // Random plate dimensions
        let plate_width = height * 0.7 + ((seed * 67.0) % 100.0) / 100.0 * height * 0.3; // 0.7 to 1.0 of height
        let plate_height = 4.0 + ((seed * 101.0) % 100.0) / 100.0 * 8.0; // 4 to 12

        // Random sand fill
        let sand_fill = 0.3 + ((seed * 191.0) % 100.0) / 100.0 * 0.4; // 0.3 to 0.7
        let wall_offset = 3.0 + ((seed * 211.0) % 100.0) / 100.0 * 5.0; // 3 to 8

        // Create the hourglass with all random parameters
        HourglassMeshBuilder::new(
            Transform::from_translation(position).with_scale(Vec3::splat(scale)),
        )
        .with_body(HourglassMeshBodyConfig {
            total_height: height,
            bulb_style,
            neck_style,
            color: glass_color,
        })
        .with_plates(HourglassMeshPlatesConfig {
            width: plate_width,
            height: plate_height,
            color: plate_color,
        })
        .with_sand(HourglassMeshSandConfig {
            color: sand_color,
            fill_percent: sand_fill,
            wall_offset,
            bottom_mound_factor: ((seed * 229.0) % 100.0) / 100.0 * 0.5, // 0.0 to 0.5 random mound
        })
        .with_timing(flip_duration)
        .with_auto_flip(true)
        .with_flip_duration(0.1 + ((seed * 223.0) % 100.0) / 100.0 * 2.9) // 0.1 to 3.0 seconds flip animation
        .build(&mut commands, &mut meshes, &mut materials);
    }

    // Add title
    commands.spawn((
        Text2d::new("Auto-Flip Hourglass Mayhem!"),
        TextFont {
            font_size: 36.0,
            ..default()
        },
        TextColor::from(Color::srgb(1.0, 1.0, 0.0)), // Yellow
        Transform::from_translation(Vec3::new(0.0, 320.0, 1.0)),
    ));

    // Add subtitle
    commands.spawn((
        Text2d::new("Grid of hourglasses with random colors, sizes, shapes, and flip durations"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor::from(Color::srgb(0.9, 0.9, 0.9)), // Light gray
        Transform::from_translation(Vec3::new(0.0, 290.0, 1.0)),
    ));
}
