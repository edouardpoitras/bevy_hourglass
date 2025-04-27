//! Defines the hourglass plugin.

use bevy::prelude::*;
use crate::resources::HourglassConfig;
use crate::events::*;
use crate::systems::*;

/// Plugin for adding hourglass functionality to Bevy apps
#[derive(Default)]
pub struct HourglassPlugin;

impl Plugin for HourglassPlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.init_resource::<HourglassConfig>();
        
        // Register events
        app.add_event::<HourglassFlipStartEvent>()
           .add_event::<HourglassFlipCompleteEvent>()
           .add_event::<HourglassEmptyEvent>()
           .add_event::<HourglassInteractionEvent>();
        
        // Add systems
        app.add_systems(Update, (
            update_hourglasses,
            update_rotations,
            handle_hourglass_interaction,
        ));
    }
}
