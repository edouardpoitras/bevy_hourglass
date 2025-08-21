//! Systems for updating hourglass state.

use crate::components::{Hourglass, SandSplash, SandSplashParticle};
use crate::events::{HourglassEmptyEvent, HourglassFlipStartEvent};
use crate::{HourglassMeshSandState, SandSplashConfig};
use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;
use rand::prelude::*;

/// System that updates all hourglasses
pub fn update_hourglasses(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Hourglass, &mut Transform)>,
    mut empty_events: EventWriter<HourglassEmptyEvent>,
    mut flip_start_events: EventWriter<HourglassFlipStartEvent>,
) {
    let delta = time.delta_secs();

    for (entity, mut hourglass, mut transform) in query.iter_mut() {
        // Check if the hourglass was running and had time remaining before the update
        let was_running = hourglass.running && hourglass.remaining_time > 0.0;

        // Handle flip events if the hourglass is starting to flip
        if hourglass.flipping {
            // Send flip start event
            flip_start_events.write(HourglassFlipStartEvent { entity });
        }

        // Normal update
        hourglass.update(delta);

        // Apply the rotation to the transform
        transform.rotation = Quat::from_rotation_z(hourglass.current_rotation);

        // Check if the hourglass just became empty
        if was_running && hourglass.remaining_time == 0.0 {
            empty_events.write(HourglassEmptyEvent {
                entity,
                total_time: hourglass.total_time,
            });
        }
    }
}

/// System that handles sand splash animation for mesh hourglasses
pub fn update_sand_splash(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut hourglass_query: Query<(
        &Hourglass,
        &HourglassMeshSandState,
        &mut SandSplash,
        &GlobalTransform,
    )>,
    mut particle_query: Query<(Entity, &mut SandSplashParticle)>,
) {
    let delta = time.delta_secs();

    // Update existing splash particles
    for (entity, mut particle) in particle_query.iter_mut() {
        particle.lifetime -= delta;
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }

    // Process hourglasses with sand splash
    for (hourglass, sand_state, mut sand_splash, global_transform) in hourglass_query.iter_mut() {
        let is_currently_flowing =
            hourglass.running && hourglass.upper_chamber > 0.0 && !hourglass.flipping;

        // Update spawn timer
        sand_splash.spawn_timer -= delta;

        // Check if sand is actively flowing and hitting the bottom
        if is_currently_flowing && sand_splash.spawn_timer <= 0.0 && hourglass.lower_chamber > 0.01
        {
            // Reset spawn timer
            sand_splash.spawn_timer = sand_splash.config.spawn_interval;

            // Calculate scale factor based on remaining sand in upper chamber
            // Full effect when > 50% sand, gradually reduces to near zero at 10% sand
            let scale_factor = if hourglass.upper_chamber > 0.5 {
                1.0
            } else if hourglass.upper_chamber > 0.1 {
                // Smooth transition from 1.0 to 0.1 between 50% and 10% sand
                let normalized = (hourglass.upper_chamber - 0.1) / (0.5 - 0.1);
                0.1 + (normalized * 0.9)
            } else {
                // Very minimal effect when less than 10% sand remains
                0.05
            };

            // Calculate the impact point based on sand level in bottom bulb
            let hourglass_pos = global_transform.translation();

            // Calculate the actual sand surface position in the bottom bulb
            let total_height = sand_state.body_config.total_height;
            let half_height = total_height / 2.0;
            let neck_height = sand_state.body_config.neck_style.height();
            let neck_bottom = -neck_height / 2.0;

            // Bottom sand fill line calculation (from curves.rs logic)
            // When fill_percent = 0.0 (empty top), bottom is full (at neck_bottom)
            // When fill_percent = 1.0 (full top), bottom is empty (at min_y)
            let min_y = -half_height;
            let bottom_fill_line =
                min_y + ((1.0 - sand_state.fill_percent) * (neck_bottom - min_y));

            // Apply to global position
            let impact_y = hourglass_pos.y + bottom_fill_line + sand_splash.config.vertical_offset;

            // Scale particle count based on remaining sand
            let scaled_particle_count =
                (sand_splash.config.particle_count as f32 * scale_factor).round() as u32;

            // Create scaled config for this spawn
            let scaled_config = SandSplashConfig {
                splash_radius: sand_splash.config.splash_radius * scale_factor,
                particle_count: scaled_particle_count,
                spawn_interval: sand_splash.config.spawn_interval * scale_factor, // Scale spawn interval
                particle_duration: sand_splash.config.particle_duration
                    * (0.3 + scale_factor * 0.7), // Don't scale duration as much
                particle_color: sand_splash.config.particle_color,
                particle_size: sand_splash.config.particle_size * (0.5 + scale_factor * 0.5), // Minimum 50% size
                vertical_offset: sand_splash.config.vertical_offset,
            };

            // Spawn splash particles with scaled parameters
            for _ in 0..scaled_particle_count {
                spawn_splash_particle(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    Vec3::new(hourglass_pos.x, impact_y, hourglass_pos.z + 0.2),
                    &scaled_config,
                );
            }
        }

        sand_splash.was_flowing = is_currently_flowing;
    }
}

/// Spawns a single sand splash particle at the given position
fn spawn_splash_particle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    impact_position: Vec3,
    config: &crate::components::SandSplashConfig,
) {
    let mut rng = rand::rng();

    // Random offset within splash radius
    let angle = rng.random::<f32>() * 2.0 * std::f32::consts::PI;
    let distance = rng.random::<f32>() * config.splash_radius;
    let offset_x = angle.cos() * distance;
    let offset_y = rng.random::<f32>() * 10.0 - 5.0; // Small vertical variation

    let particle_position = impact_position + Vec3::new(offset_x, offset_y, 0.0);

    // Create a simple rectangle mesh for the particle
    let size = config.particle_size;
    let mesh = meshes.add(Rectangle::new(size, size));
    let material = materials.add(ColorMaterial {
        color: config.particle_color,
        alpha_mode: AlphaMode2d::Blend,
        ..default()
    });

    commands.spawn((
        SandSplashParticle {
            lifetime: config.particle_duration,
        },
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_translation(particle_position),
    ));
}
