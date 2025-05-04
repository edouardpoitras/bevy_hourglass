//! Systems for handling hourglass rotation and flipping.

use bevy::prelude::*;
use crate::components::Hourglass;
use crate::events::{HourglassFlipStartEvent, HourglassFlipCompleteEvent};

/// System that updates hourglass rotations
pub fn update_rotations(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Hourglass, &mut Transform)>,
    mut flip_start_events: EventWriter<HourglassFlipStartEvent>,
    mut flip_complete_events: EventWriter<HourglassFlipCompleteEvent>,
) {
    for (entity, mut hourglass, mut transform) in query.iter_mut() {
        // Apply the rotation to the transform
        transform.rotation = Quat::from_rotation_z(hourglass.current_rotation);
        
        // Handle flip events
        if hourglass.flipping {
            // Check if the flip just started (flip_elapsed is 0)
            if hourglass.flip_elapsed == 0.0 {
                // Send flip start event
                flip_start_events.write(HourglassFlipStartEvent { entity });
            }
            
            // Check if the flip just completed
            let was_flipping = hourglass.flipping;
            
            // The update method will handle the flip animation and state changes
            hourglass.update(time.delta());
            
            // If the flip just completed, send the event
            if was_flipping && !hourglass.flipping {
                flip_complete_events.write(HourglassFlipCompleteEvent {
                    entity,
                    is_flipped: hourglass.flipped,
                });
            }
        } else {
            // If not flipping, just update the hourglass
            hourglass.update(time.delta());
        }
    }
}
