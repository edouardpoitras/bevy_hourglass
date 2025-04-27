//! Defines the global configuration resource for the hourglass plugin.

use bevy::prelude::*;
use crate::traits::EasingFunction;

/// Global configuration for the hourglass plugin
#[derive(Resource, Debug, Clone)]
pub struct HourglassConfig {
    /// Default duration for flip animations in seconds
    pub default_flip_duration: f32,
    
    /// Default easing function for flip animations
    pub default_easing: EasingFunction,
    
    /// Default color for hourglass containers
    pub default_container_color: Color,
    
    /// Default color for hourglass sand
    pub default_sand_color: Color,
    
    /// Default size for hourglasses
    pub default_size: Vec2,
    
    /// Whether to update sand while flipping by default
    pub update_during_flip: bool,
}

impl Default for HourglassConfig {
    fn default() -> Self {
        Self {
            default_flip_duration: 1.0,
            default_easing: EasingFunction::QuadraticInOut,
            default_container_color: Color::srgb(0.8, 0.8, 0.8),
            default_sand_color: Color::srgb(0.8, 0.6, 0.2),
            default_size: Vec2::new(100.0, 200.0),
            update_during_flip: false,
        }
    }
}

impl HourglassConfig {
    /// Create a new hourglass configuration with default settings
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the default flip duration
    pub fn with_flip_duration(mut self, duration: f32) -> Self {
        self.default_flip_duration = duration;
        self
    }
    
    /// Set the default easing function
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.default_easing = easing;
        self
    }
    
    /// Set the default container color
    pub fn with_container_color(mut self, color: Color) -> Self {
        self.default_container_color = color;
        self
    }
    
    /// Set the default sand color
    pub fn with_sand_color(mut self, color: Color) -> Self {
        self.default_sand_color = color;
        self
    }
    
    /// Set the default size
    pub fn with_size(mut self, size: Vec2) -> Self {
        self.default_size = size;
        self
    }
    
    /// Set whether to update sand while flipping
    pub fn with_update_during_flip(mut self, update: bool) -> Self {
        self.update_during_flip = update;
        self
    }
}
