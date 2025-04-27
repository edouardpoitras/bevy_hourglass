//! Defines the rotation state component for hourglass flipping animations.

use bevy::prelude::*;
use crate::traits::EasingFunction;

/// Component for tracking the rotation state of an hourglass
#[derive(Component, Debug, Clone)]
pub struct RotationState {
    /// Current rotation in radians
    pub current_rotation: f32,
    
    /// Target rotation in radians
    pub target_rotation: f32,
    
    /// Whether the hourglass is currently flipping
    pub is_flipping: bool,
    
    /// Current progress of the flip animation (0.0 - 1.0)
    pub flip_progress: f32,
    
    /// Duration of the flip animation in seconds
    pub flip_duration: f32,
    
    /// Elapsed time of the current flip animation in seconds
    pub flip_elapsed: f32,
    
    /// Easing function to use for the flip animation
    pub easing: EasingFunction,
}

impl Default for RotationState {
    fn default() -> Self {
        Self {
            current_rotation: 0.0,
            target_rotation: 0.0,
            is_flipping: false,
            flip_progress: 0.0,
            flip_duration: 1.0,
            flip_elapsed: 0.0,
            easing: EasingFunction::QuadraticInOut,
        }
    }
}

impl RotationState {
    /// Create a new rotation state with the specified initial rotation
    pub fn new(initial_rotation: f32) -> Self {
        Self {
            current_rotation: initial_rotation,
            target_rotation: initial_rotation,
            ..Default::default()
        }
    }
    
    /// Start a flip animation to the target rotation
    pub fn start_flip(&mut self, target_rotation: f32) {
        self.target_rotation = target_rotation;
        self.is_flipping = true;
        self.flip_progress = 0.0;
        self.flip_elapsed = 0.0;
    }
    
    /// Update the flip animation
    /// 
    /// Returns true if the flip animation is complete
    pub fn update_flip(&mut self, delta_seconds: f32) -> bool {
        if !self.is_flipping {
            return false;
        }
        
        self.flip_elapsed += delta_seconds;
        self.flip_progress = (self.flip_elapsed / self.flip_duration).min(1.0);
        
        // Apply easing function
        let eased_progress = self.easing.apply(self.flip_progress);
        
        // Interpolate between current and target rotation
        let start_rotation = self.current_rotation;
        let rotation_diff = self.target_rotation - start_rotation;
        self.current_rotation = start_rotation + rotation_diff * eased_progress;
        
        // Check if the flip is complete
        if self.flip_progress >= 1.0 {
            self.is_flipping = false;
            self.current_rotation = self.target_rotation;
            return true;
        }
        
        false
    }
}
