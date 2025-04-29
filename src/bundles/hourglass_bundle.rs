//! Defines bundles for creating hourglasses.

use bevy::prelude::*;
use std::time::Duration;

use crate::components::{HourglassComponent, RotationState, InteractableHourglass};
use crate::traits::EasingFunction;

/// Bundle for creating a basic hourglass
#[derive(Bundle, Clone)]
pub struct HourglassBundle {
    /// Core hourglass component
    pub hourglass: HourglassComponent,
    
    /// Rotation state for the hourglass
    pub rotation: RotationState,
    
    /// Spatial transform - required for positioning
    pub transform: Transform,
}

impl Default for HourglassBundle {
    fn default() -> Self {
        Self {
            hourglass: HourglassComponent::default(),
            rotation: RotationState::default(),
            transform: Transform::default(),
        }
    }
}

/// Bundle for creating an interactable hourglass
#[derive(Bundle, Clone)]
pub struct InteractableHourglassBundle {
    /// Core hourglass component
    pub hourglass: HourglassComponent,
    
    /// Rotation state for the hourglass
    pub rotation: RotationState,
    
    /// Spatial transform - required for positioning
    pub transform: Transform,
    
    /// Interactable component
    pub interactable: InteractableHourglass,
}

impl Default for InteractableHourglassBundle {
    fn default() -> Self {
        Self {
            hourglass: HourglassComponent::default(),
            rotation: RotationState::default(),
            transform: Transform::default(),
            interactable: InteractableHourglass::default(),
        }
    }
}

impl InteractableHourglassBundle {
    /// Create a new interactable hourglass with the specified duration
    pub fn new(duration: Duration) -> Self {
        Self {
            hourglass: HourglassComponent::new(duration),
            rotation: RotationState::default(),
            transform: Transform::default(),
            interactable: InteractableHourglass::default(),
        }
    }
    
    /// Create a new interactable hourglass with mouse following enabled
    pub fn with_mouse_following(duration: Duration) -> Self {
        Self {
            hourglass: HourglassComponent::new(duration),
            rotation: RotationState::default(),
            transform: Transform::default(),
            interactable: InteractableHourglass::with_mouse_following(),
        }
    }
    
    /// Set the easing function for the rotation animation
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.rotation.easing = easing;
        self
    }
    
    /// Set the flip duration for the rotation animation
    pub fn with_flip_duration(mut self, duration: f32) -> Self {
        self.rotation.flip_duration = duration;
        self
    }
}
