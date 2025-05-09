//! Defines the global configuration resource for the hourglass plugin.

use bevy::prelude::*;

/// Global configuration for the hourglass plugin
#[derive(Resource, Debug, Clone)]
pub struct HourglassConfig {
    /// Default color for hourglass containers
    pub default_container_color: Color,
    
    /// Default color for hourglass sand
    pub default_sand_color: Color,
    
    /// Default size for hourglasses
    pub default_size: Vec2,
}

impl Default for HourglassConfig {
    fn default() -> Self {
        Self {
            default_container_color: Color::srgb(0.8, 0.8, 0.8),
            default_sand_color: Color::srgb(0.8, 0.6, 0.2),
            default_size: Vec2::new(100.0, 200.0),
        }
    }
}

impl HourglassConfig {
    /// Create a new hourglass configuration with default settings
    pub fn new() -> Self {
        Self::default()
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
}
