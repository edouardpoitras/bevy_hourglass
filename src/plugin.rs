//! Defines the hourglass plugin.

use crate::events::*;
use crate::resources::HourglassConfig;
use crate::sprite_hourglass::{
    update_bottom_sand_sprite, update_container_sprite, update_top_sand_sprite,
};
use crate::systems::update_hourglasses;
use bevy::prelude::*;

/// Plugin for adding hourglass functionality to Bevy apps
#[derive(Default)]
pub struct HourglassPlugin;

impl Plugin for HourglassPlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.init_resource::<HourglassConfig>();

        // Register events
        app.add_event::<HourglassFlipStartEvent>()
            .add_event::<HourglassEmptyEvent>();

        // Add all systems individually to avoid configuration issues
        app.add_systems(Update, update_hourglasses);

        // Sprite-based visualization systems
        app.add_systems(Update, update_container_sprite);
        app.add_systems(Update, update_top_sand_sprite);
        app.add_systems(Update, update_bottom_sand_sprite);
    }
}
