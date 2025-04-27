//! Defines events for the hourglass plugin.

use bevy::prelude::*;
use std::time::Duration;

/// Event sent when an hourglass starts flipping
#[derive(Event, Debug, Clone)]
pub struct HourglassFlipStartEvent {
    /// Entity ID of the hourglass
    pub entity: Entity,
}

/// Event sent when an hourglass completes flipping
#[derive(Event, Debug, Clone)]
pub struct HourglassFlipCompleteEvent {
    /// Entity ID of the hourglass
    pub entity: Entity,
    
    /// Whether the hourglass is now flipped (upside down)
    pub is_flipped: bool,
}

/// Event sent when an hourglass becomes empty
#[derive(Event, Debug, Clone)]
pub struct HourglassEmptyEvent {
    /// Entity ID of the hourglass
    pub entity: Entity,
    
    /// Total time the hourglass was running
    pub total_time: Duration,
}

/// Event sent when an hourglass is interacted with
#[derive(Event, Debug, Clone)]
pub struct HourglassInteractionEvent {
    /// Entity ID of the hourglass
    pub entity: Entity,
    
    /// Type of interaction
    pub interaction_type: InteractionType,
}

/// Types of interactions with an hourglass
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InteractionType {
    /// Clicked on the hourglass
    Click,
    
    /// Started dragging the hourglass
    DragStart,
    
    /// Dragging the hourglass
    Drag,
    
    /// Stopped dragging the hourglass
    DragEnd,
    
    /// Hovering over the hourglass
    Hover,
    
    /// No longer hovering over the hourglass
    HoverExit,
}
