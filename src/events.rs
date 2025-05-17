//! Defines events for the hourglass plugin.

use bevy::prelude::*;
use std::time::Duration;

/// Event sent when an hourglass starts flipping
#[derive(Event, Debug, Clone)]
pub struct HourglassFlipStartEvent {
    /// Entity ID of the hourglass
    pub entity: Entity,
}

/// Event sent when an hourglass becomes empty
#[derive(Event, Debug, Clone)]
pub struct HourglassEmptyEvent {
    /// Entity ID of the hourglass
    pub entity: Entity,

    /// Total time the hourglass was running
    pub total_time: Duration,
}
