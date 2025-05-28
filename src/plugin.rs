//! Defines the hourglass plugin.

use crate::events::*;
use crate::mesh_hourglass::{sync_mesh_hourglass_with_timer, update_mesh_hourglass_sand};
use crate::resources::HourglassConfig;
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

        // Add core hourglass update system
        app.add_systems(Update, update_hourglasses);

        // Mesh-based visualization systems
        app.add_systems(
            Update,
            (sync_mesh_hourglass_with_timer, update_mesh_hourglass_sand).chain(),
        );
    }
}
