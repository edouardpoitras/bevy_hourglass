//! Systems for updating hourglass state.

use crate::components::{Hourglass, SandSplash, SandSplashParticle};
use crate::events::{HourglassEmptyEvent, HourglassFlipStartEvent};
use bevy::prelude::*;
use bevy::sprite::{AlphaMode2d, Mesh2d};
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
    mut hourglass_query: Query<(Entity, &Hourglass, &mut SandSplash, &GlobalTransform)>,
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
    for (hourglass_entity, hourglass, mut sand_splash, global_transform) in
        hourglass_query.iter_mut()
    {
        if !sand_splash.config.enabled {
            continue;
        }

        let is_currently_flowing = hourglass.running && hourglass.upper_chamber > 0.0;

        // Update spawn timer
        sand_splash.spawn_timer -= delta;

        // Check if sand is actively flowing and hitting the bottom
        if is_currently_flowing
            && sand_splash.spawn_timer <= 0.0
            && hourglass.lower_chamber > 0.01
        {
            // Reset spawn timer
            sand_splash.spawn_timer = sand_splash.config.spawn_interval;

            // Calculate the impact point (bottom of the hourglass)
            let hourglass_pos = global_transform.translation();
            let impact_y = hourglass_pos.y - 80.0; // Approximate bottom of hourglass

            // Spawn splash particles
            for _ in 0..sand_splash.config.particle_count {
                spawn_splash_particle(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    Vec3::new(hourglass_pos.x, impact_y, hourglass_pos.z + 0.2),
                    &sand_splash.config,
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
    let mut rng = thread_rng();

    // Random offset within splash radius
    let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
    let distance = rng.gen::<f32>() * config.splash_radius;
    let offset_x = angle.cos() * distance;
    let offset_y = rng.gen::<f32>() * 10.0 - 5.0; // Small vertical variation

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
