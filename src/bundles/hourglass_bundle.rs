//! Defines bundles for creating hourglasses.

use bevy::prelude::*;
use std::time::Duration;

use crate::components::{Hourglass, InteractableHourglass};

/// Bundle for creating a basic hourglass
#[derive(Bundle, Clone)]
pub struct HourglassBundle {
    /// Core hourglass component
    pub hourglass: Hourglass,
    
    /// Spatial transform - required for positioning
    pub transform: Transform,
}

impl Default for HourglassBundle {
    fn default() -> Self {
        Self {
            hourglass: Hourglass::default(),
            transform: Transform::default(),
        }
    }
}

impl HourglassBundle {
    /// Create a new hourglass bundle with the specified duration
    pub fn new(duration: Duration) -> Self {
        Self {
            hourglass: Hourglass::new(duration),
            ..Default::default()
        }
    }
    
    /// Set the container color
    pub fn with_container_color(mut self, color: Color) -> Self {
        self.hourglass.container_color = color;
        self
    }
    
    /// Set the sand color
    pub fn with_sand_color(mut self, color: Color) -> Self {
        self.hourglass.sand_color = color;
        self
    }
    
    /// Set the size of the hourglass
    pub fn with_size(mut self, size: Vec2) -> Self {
        self.hourglass.size = size;
        self
    }
    
}

/// Bundle for creating an interactable hourglass
#[derive(Bundle, Clone)]
pub struct InteractableHourglassBundle {
    /// Core hourglass bundle
    pub hourglass_bundle: HourglassBundle,
    
    /// Interactable component
    pub interactable: InteractableHourglass,
}

impl Default for InteractableHourglassBundle {
    fn default() -> Self {
        Self {
            hourglass_bundle: HourglassBundle::default(),
            interactable: InteractableHourglass::default(),
        }
    }
}

impl InteractableHourglassBundle {
    /// Create a new interactable hourglass with the specified duration
    pub fn new(duration: Duration) -> Self {
        Self {
            hourglass_bundle: HourglassBundle::new(duration),
            interactable: InteractableHourglass::default(),
        }
    }
    
    
    /// Set the container color
    pub fn with_container_color(mut self, color: Color) -> Self {
        self.hourglass_bundle = self.hourglass_bundle.with_container_color(color);
        self
    }
    
    /// Set the sand color
    pub fn with_sand_color(mut self, color: Color) -> Self {
        self.hourglass_bundle = self.hourglass_bundle.with_sand_color(color);
        self
    }
    
    /// Set the size of the hourglass
    pub fn with_size(mut self, size: Vec2) -> Self {
        self.hourglass_bundle = self.hourglass_bundle.with_size(size);
        self
    }
    
}
