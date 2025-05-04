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
    
    /// Whether the hourglass can be moved by the user
    pub can_move: bool,
    
    /// Whether the hourglass can be resized by the user
    pub can_resize: bool,
    
    /// Whether the hourglass follows the mouse when flipping
    pub mouse_follow: bool,
    
    /// Sensitivity of mouse following (1.0 = normal)
    pub mouse_sensitivity: f32,
    
    /// Minimum rotation angle in radians (for mouse following)
    pub min_angle: f32,
    
    /// Maximum rotation angle in radians (for mouse following)
    pub max_angle: f32,
    
    /// Threshold for snapping to min/max angles
    pub snap_threshold: f32,
}

impl Default for InteractableHourglass {
    fn default() -> Self {
        Self {
            is_interacting: false,
            can_flip: true,
            can_move: false,
            can_resize: false,
            mouse_follow: false,
            mouse_sensitivity: 1.0,
            min_angle: -std::f32::consts::PI,
            max_angle: 0.0,
            snap_threshold: 0.1,
        }
    }
}

impl InteractableHourglass {
    /// Create a new interactable hourglass with default settings
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a new interactable hourglass with mouse following enabled
    pub fn with_mouse_following() -> Self {
        Self {
            mouse_follow: true,
            ..Default::default()
        }
    }
    
    /// Set whether the hourglass can be flipped
    pub fn with_can_flip(mut self, can_flip: bool) -> Self {
        self.can_flip = can_flip;
        self
    }
    
    /// Set whether the hourglass can be moved
    pub fn with_can_move(mut self, can_move: bool) -> Self {
        self.can_move = can_move;
        self
    }
    
    /// Set whether the hourglass can be resized
    pub fn with_can_resize(mut self, can_resize: bool) -> Self {
        self.can_resize = can_resize;
        self
    }
    
    /// Set the mouse sensitivity
    pub fn with_sensitivity(mut self, sensitivity: f32) -> Self {
        self.mouse_sensitivity = sensitivity;
        self
    }
    
    /// Set the minimum and maximum rotation angles
    pub fn with_angle_limits(mut self, min_angle: f32, max_angle: f32) -> Self {
        self.min_angle = min_angle;
        self.max_angle = max_angle;
        self
    }
}
