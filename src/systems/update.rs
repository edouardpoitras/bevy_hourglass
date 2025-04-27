//! Systems for updating hourglass state.

use bevy::prelude::*;
use crate::components::HourglassComponent;
use crate::events::HourglassEmptyEvent;

/// System that updates all hourglasses
pub fn update_hourglasses(
    time: Res<Time>,
    mut hourglasses: Query<(Entity, &mut HourglassComponent)>,
    mut empty_events: EventWriter<HourglassEmptyEvent>,
) {
    let delta = time.delta();
    
    for (entity, mut hourglass) in hourglasses.iter_mut() {
        // Check if the hourglass was running and had time remaining before the update
        let was_running = hourglass.running && hourglass.remaining_time > std::time::Duration::ZERO;
        
        // Update the hourglass
        hourglass.update(delta);
        
        // Check if the hourglass just became empty
        if was_running && hourglass.remaining_time == std::time::Duration::ZERO {
            empty_events.write(HourglassEmptyEvent {
                entity,
                total_time: hourglass.total_time,
            });
        }
    }
}
