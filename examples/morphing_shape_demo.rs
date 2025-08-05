//! Demonstrates sand splash animation with a morphing hourglass shape.
//!
//! This example showcases how to create an animated hourglass that morphs its shape over time while
//! also using the sand splash animation. The hourglass cycles through four different shapes.

use bevy::prelude::*;
use bevy::render::{mesh::Indices, render_resource::PrimitiveTopology};
use bevy_hourglass::{
    spawn_mesh_hourglass_with_timer, BulbStyle, Hourglass, HourglassMeshBody,
    HourglassMeshBodyConfig, HourglassMeshBuilder, HourglassMeshPlate, HourglassMeshPlatesConfig,
    HourglassMeshSandConfig, HourglassMeshSandState, HourglassPlugin, HourglassShapeBuilder,
    NeckStyle, SandSplashConfig,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HourglassPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_morphing_shape)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn camera
    commands.spawn(Camera2d);

    // Hourglass with default sand splash
    let default_splash_config = SandSplashConfig::default();

    HourglassMeshBuilder::new(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)))
        .with_body(HourglassMeshBodyConfig::default())
        .with_plates(HourglassMeshPlatesConfig::default())
        .with_sand(HourglassMeshSandConfig::default())
        .with_sand_splash(default_splash_config)
        .with_timing(10.0)
        .build(&mut commands, &mut meshes, &mut materials);
}

fn update_morphing_shape(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut query: Query<(Entity, &Hourglass, &mut HourglassMeshSandState)>,
    children_query: Query<&Children>,
    body_query: Query<(
        Entity,
        &HourglassMeshBody,
        Option<&Mesh2d>,
        Option<&MeshMaterial2d<ColorMaterial>>,
    )>,
    plate_query: Query<(
        Entity,
        &HourglassMeshPlate,
        Option<&Mesh2d>,
        Option<&MeshMaterial2d<ColorMaterial>>,
    )>,
) {
    // Don't interrupt the hourglass if it's currently flipping
    for (_, hourglass, _) in query.iter() {
        if hourglass.flipping {
            return;
        }
    }

    // Cycle through shapes over time (complete cycle every 8 seconds)
    let cycle_time = 8.0;
    let t = (time.elapsed_secs() % cycle_time) / cycle_time;

    // Create morphed shape parameters
    let (body_config, plates_config) = get_morphed_shape_config(t);

    // Update the hourglass components
    for (hourglass_entity, hourglass, mut sand_state) in query.iter_mut() {
        // Update the body configuration for sand
        sand_state.body_config = body_config.clone();

        // Preserve the current fill percentage
        let fill_percent = hourglass.remaining_time / hourglass.total_time;
        sand_state.fill_percent = fill_percent;
        sand_state.sand_config.fill_percent = fill_percent;

        // Set the flag to trigger sand mesh regeneration
        sand_state.needs_update = true;

        // Now update the body and plates
        if let Ok(children) = children_query.get(hourglass_entity) {
            // Update body
            for child in children.iter() {
                // Try to find the body entity
                if let Ok((body_entity, _, mesh_handle_opt, _)) = body_query.get(child) {
                    // Create the hourglass shape builder from the config
                    let shape_builder = HourglassShapeBuilder {
                        total_height: body_config.total_height,
                        bulb_style: body_config.bulb_style.clone(),
                        neck_style: body_config.neck_style.clone(),
                    };

                    // Generate the hourglass outline
                    let outline_points = shape_builder.generate_outline();

                    // Create new mesh from the generated points
                    if let Some(new_mesh) =
                        HourglassMeshBuilder::create_mesh_from_points(outline_points)
                    {
                        let mesh_handle = meshes.add(new_mesh);

                        if let Some(_) = mesh_handle_opt {
                            // Replace the mesh component entirely
                            commands.entity(body_entity).insert(Mesh2d(mesh_handle));
                        } else {
                            // This shouldn't happen, but just in case
                            commands.entity(body_entity).insert(Mesh2d(mesh_handle));
                        }
                    }
                }

                // Try to find the plate entities
                if let Ok((plate_entity, plate_type, mesh_handle_opt, material_opt)) =
                    plate_query.get(child)
                {
                    // Create new plate mesh
                    let mut plate_mesh =
                        Mesh::new(PrimitiveTopology::TriangleList, Default::default());

                    // Rectangle vertices (centered at origin)
                    let half_width = plates_config.width / 2.0;
                    let half_height = plates_config.height / 2.0;
                    let points_3d = vec![
                        [-half_width, -half_height, 0.0], // bottom left
                        [half_width, -half_height, 0.0],  // bottom right
                        [half_width, half_height, 0.0],   // top right
                        [-half_width, half_height, 0.0],  // top left
                    ];

                    // Indices for two triangles making up the rectangle
                    let indices = vec![0, 1, 2, 0, 2, 3];

                    plate_mesh.insert_indices(Indices::U32(indices));
                    plate_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points_3d);
                    plate_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 4]);
                    plate_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; 4]);

                    let mesh_handle = meshes.add(plate_mesh);

                    if let Some(_) = mesh_handle_opt {
                        // Replace the mesh component entirely
                        commands.entity(plate_entity).insert(Mesh2d(mesh_handle));
                    } else {
                        // This shouldn't happen, but just in case
                        commands.entity(plate_entity).insert(Mesh2d(mesh_handle));
                    }

                    // Also update the plate positions based on the new body height
                    let half_total_height = body_config.total_height / 2.0;
                    match plate_type {
                        HourglassMeshPlate::Top => {
                            commands.entity(plate_entity).insert(Transform::from_xyz(
                                0.0,
                                half_total_height + plates_config.height / 2.0,
                                0.0,
                            ));
                        }
                        HourglassMeshPlate::Bottom => {
                            commands.entity(plate_entity).insert(Transform::from_xyz(
                                0.0,
                                -half_total_height - plates_config.height / 2.0,
                                0.0,
                            ));
                        }
                    }
                }
            }
        }
    }
}

// Helper function to create morphed shape configurations
fn get_morphed_shape_config(t: f32) -> (HourglassMeshBodyConfig, HourglassMeshPlatesConfig) {
    // Define the 4 shape configurations
    let shapes = [
        HourglassShape::Classic,
        HourglassShape::Modern,
        HourglassShape::Slim,
        HourglassShape::Wide,
    ];

    // Determine which shapes to interpolate between
    let segment = t * 4.0; // 0-4 range
    let segment_index = segment.floor() as usize % 4;
    let next_index = (segment_index + 1) % 4;
    let local_t = segment - segment.floor(); // 0-1 within the segment

    let shape1 = shapes[segment_index];
    let shape2 = shapes[next_index];

    // Get the base configurations for both shapes
    let (config1, plates1) = get_main_shape_config(shape1);
    let (config2, plates2) = get_main_shape_config(shape2);

    // Interpolate between the configurations
    let interpolated_body = HourglassMeshBodyConfig {
        total_height: lerp_f32(config1.total_height, config2.total_height, local_t),
        bulb_style: interpolate_bulb_style(&config1.bulb_style, &config2.bulb_style, local_t),
        neck_style: interpolate_neck_style(&config1.neck_style, &config2.neck_style, local_t),
        color: Color::srgba(0.85, 0.95, 1.0, 0.2),
    };

    let interpolated_plates = HourglassMeshPlatesConfig {
        width: lerp_f32(plates1.width, plates2.width, local_t),
        height: lerp_f32(plates1.height, plates2.height, local_t),
        ..Default::default()
    };

    (interpolated_body, interpolated_plates)
}

/// Resource to track the current hourglass configuration
#[derive(Resource, Debug, Clone)]
pub struct HourglassConfig {
    pub color: Color,
    pub shape_type: HourglassShape,
}

impl Default for HourglassConfig {
    fn default() -> Self {
        Self {
            color: Color::srgb(0.8, 0.6, 0.2), // Sandy color
            shape_type: HourglassShape::Classic,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HourglassShape {
    Classic,
    Modern,
    Slim,
    Wide,
}

// Helper function to create main hourglass configurations for different shapes
fn get_main_shape_config(
    shape: HourglassShape,
) -> (HourglassMeshBodyConfig, HourglassMeshPlatesConfig) {
    let base_height = 400.0; // Full size for main hourglass

    match shape {
        HourglassShape::Classic => (
            HourglassMeshBodyConfig {
                total_height: base_height,
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
            },
            HourglassMeshPlatesConfig {
                width: 400.0,
                height: 10.0,
                ..Default::default()
            },
        ),
        HourglassShape::Modern => (
            HourglassMeshBodyConfig {
                total_height: base_height,
                bulb_style: BulbStyle::Circular {
                    curvature: 0.0,
                    width_factor: 1.0,
                    curve_resolution: 10,
                },
                neck_style: NeckStyle::Straight {
                    width: 12.0,
                    height: 32.0,
                },
                color: Color::srgba(0.85, 0.95, 1.0, 0.2),
            },
            HourglassMeshPlatesConfig {
                width: 380.0,
                height: 12.0,
                ..Default::default()
            },
        ),
        HourglassShape::Slim => (
            HourglassMeshBodyConfig {
                total_height: base_height * 1.2, // Taller
                bulb_style: BulbStyle::Circular {
                    curvature: 1.5,
                    width_factor: 0.7, // Narrower
                    curve_resolution: 18,
                },
                neck_style: NeckStyle::Curved {
                    curvature: 1.5,
                    width: 12.0, // Thinner neck
                    height: 24.0,
                    curve_resolution: 8,
                },
                color: Color::srgba(0.85, 0.95, 1.0, 0.2),
            },
            HourglassMeshPlatesConfig {
                width: 340.0, // Narrower plates
                height: 8.0,
                ..Default::default()
            },
        ),
        HourglassShape::Wide => (
            HourglassMeshBodyConfig {
                total_height: base_height * 0.8, // Shorter
                bulb_style: BulbStyle::Circular {
                    curvature: 1.0,
                    width_factor: 1.2, // Wider
                    curve_resolution: 24,
                },
                neck_style: NeckStyle::Curved {
                    curvature: 0.7,
                    width: 20.0, // Thicker neck
                    height: 16.0,
                    curve_resolution: 12,
                },
                color: Color::srgba(0.85, 0.95, 1.0, 0.2),
            },
            HourglassMeshPlatesConfig {
                width: 390.0, // Wider plates
                height: 14.0,
                ..Default::default()
            },
        ),
    }
}

// Helper functions for interpolation
fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn interpolate_bulb_style(style1: &BulbStyle, style2: &BulbStyle, t: f32) -> BulbStyle {
    match (style1, style2) {
        (
            BulbStyle::Circular {
                curvature: c1,
                width_factor: w1,
                curve_resolution: r1,
            },
            BulbStyle::Circular {
                curvature: c2,
                width_factor: w2,
                curve_resolution: r2,
            },
        ) => BulbStyle::Circular {
            curvature: lerp_f32(*c1, *c2, t),
            width_factor: lerp_f32(*w1, *w2, t),
            curve_resolution: (lerp_f32(*r1 as f32, *r2 as f32, t) as usize).max(5),
        },
        // If styles are different types, just switch at halfway point
        (style1, style2) => {
            if t < 0.5 {
                style1.clone()
            } else {
                style2.clone()
            }
        }
    }
}

fn interpolate_neck_style(style1: &NeckStyle, style2: &NeckStyle, t: f32) -> NeckStyle {
    match (style1, style2) {
        (
            NeckStyle::Curved {
                curvature: c1,
                width: w1,
                height: h1,
                curve_resolution: r1,
            },
            NeckStyle::Curved {
                curvature: c2,
                width: w2,
                height: h2,
                curve_resolution: r2,
            },
        ) => NeckStyle::Curved {
            curvature: lerp_f32(*c1, *c2, t),
            width: lerp_f32(*w1, *w2, t),
            height: lerp_f32(*h1, *h2, t),
            curve_resolution: (lerp_f32(*r1 as f32, *r2 as f32, t) as usize).max(3),
        },
        (
            NeckStyle::Straight {
                width: w1,
                height: h1,
            },
            NeckStyle::Straight {
                width: w2,
                height: h2,
            },
        ) => NeckStyle::Straight {
            width: lerp_f32(*w1, *w2, t),
            height: lerp_f32(*h1, *h2, t),
        },
        // Mixed types - convert straight to curved for interpolation
        (
            NeckStyle::Straight {
                width: w1,
                height: h1,
            },
            NeckStyle::Curved {
                curvature: c2,
                width: w2,
                height: h2,
                curve_resolution: r2,
            },
        ) => NeckStyle::Curved {
            curvature: lerp_f32(0.0, *c2, t),
            width: lerp_f32(*w1, *w2, t),
            height: lerp_f32(*h1, *h2, t),
            curve_resolution: *r2,
        },
        (
            NeckStyle::Curved {
                curvature: c1,
                width: w1,
                height: h1,
                curve_resolution: r1,
            },
            NeckStyle::Straight {
                width: w2,
                height: h2,
            },
        ) => NeckStyle::Curved {
            curvature: lerp_f32(*c1, 0.0, t),
            width: lerp_f32(*w1, *w2, t),
            height: lerp_f32(*h1, *h2, t),
            curve_resolution: *r1,
        },
    }
}
