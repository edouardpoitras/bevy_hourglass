//! Systems for handling hourglass rotation and flipping.

use bevy::prelude::*;
use crate::components::{HourglassComponent, RotationState};
use crate::events::{HourglassFlipStartEvent, HourglassFlipCompleteEvent};

/// System that updates hourglass rotations
pub fn update_rotations(
    time: Res<Time>,
    mut query: Query<(Entity, &mut RotationState, &mut HourglassComponent, &mut Transform)>,
    mut flip_start_events: EventWriter<HourglassFlipStartEvent>,
    mut flip_complete_events: EventWriter<HourglassFlipCompleteEvent>,
) {
    let delta_seconds = time.delta_secs();
    
    for (entity, mut rotation, mut hourglass, mut transform) in query.iter_mut() {
        // If the hourglass is flipping but the rotation state isn't, start the flip
        if hourglass.flipping && !rotation.is_flipping {
            // Calculate target rotation (180 degrees from current)
            let target_rotation = rotation.current_rotation + std::f32::consts::PI;
            
            // Start the flip animation
            rotation.start_flip(target_rotation);
            
            // Send flip start event
            flip_start_events.write(HourglassFlipStartEvent { entity });
        }
        
        // Update the rotation if flipping
        if rotation.is_flipping {
            // Update the rotation animation
            let flip_complete = rotation.update_flip(delta_seconds);
            
            // Apply the rotation to the transform
            transform.rotation = Quat::from_rotation_z(rotation.current_rotation);
            
            // If the flip is complete, update the hourglass state
            if flip_complete {
                hourglass.complete_flip();
                
                // Send flip complete event
                flip_complete_events.write(HourglassFlipCompleteEvent {
                    entity,
                    is_flipped: hourglass.flipped,
                });
            }
        }
    }
}
