//! Defines the hourglass plugin.

use bevy::prelude::*;
use crate::resources::HourglassConfig;
use crate::events::*;
use crate::systems::update_hourglasses;
use crate::sprite_hourglass::{update_container_sprite, update_top_sand_sprite, update_bottom_sand_sprite};

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
        
        // Add systems - note that update_rotations was removed as it's now part of update_hourglasses
        app.add_systems(Update, (
            update_hourglasses,
            update_container_sprite,
            update_top_sand_sprite,
            update_bottom_sand_sprite,
        ));
    }
}
