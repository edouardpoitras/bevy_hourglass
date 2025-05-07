//! Defines the interactable component for user interaction with hourglasses.

use bevy::prelude::*;
use crate::components::Hourglass;

/// Component for making an hourglass interactable
#[derive(Component, Debug, Clone)]
#[require(Hourglass)]
pub struct InteractableHourglass {
    /// Whether the hourglass is currently being interacted with
    pub is_interacting: bool,
    
    /// Whether the hourglass can be flipped by the user
    pub can_flip: bool,
}

impl Default for InteractableHourglass {
    fn default() -> Self {
        Self {
            is_interacting: false,
            can_flip: true,
        }
    }
}

impl InteractableHourglass {
    /// Create a new interactable hourglass with default settings
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set whether the hourglass can be flipped
    pub fn with_can_flip(mut self, can_flip: bool) -> Self {
        self.can_flip = can_flip;
        self
    }
}
