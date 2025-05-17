//! Systems for updating hourglass state.

use crate::components::Hourglass;
use crate::events::{HourglassEmptyEvent, HourglassFlipStartEvent};
use bevy::prelude::*;

/// System that updates all hourglasses
pub fn update_hourglasses(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Hourglass, &mut Transform)>,
    mut empty_events: EventWriter<HourglassEmptyEvent>,
    mut flip_start_events: EventWriter<HourglassFlipStartEvent>,
) {
    let delta = time.delta();

    for (entity, mut hourglass, mut transform) in query.iter_mut() {
        // Check if the hourglass was running and had time remaining before the update
        let was_running = hourglass.running && hourglass.remaining_time > std::time::Duration::ZERO;

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
        if was_running && hourglass.remaining_time == std::time::Duration::ZERO {
            empty_events.write(HourglassEmptyEvent {
                entity,
                total_time: hourglass.total_time,
            });
        }
    }
}
