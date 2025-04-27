//! Defines the Sand trait for hourglass sand behavior.

use std::time::Duration;
use bevy::prelude::*;

/// Trait for the sand/particles inside an hourglass
pub trait Sand {
    /// Update the sand state
    fn update(&mut self, delta_time: Duration, is_flipped: bool);
    
    /// Render the sand
    fn render(&self);
    
    /// Get the color of the sand
    fn get_color(&self) -> Color;
    
    /// Set the color of the sand
    fn set_color(&mut self, color: Color);
    
    /// Get the fill percentage (0.0 - 1.0) of the top chamber
    fn get_top_fill_percentage(&self) -> f32;
    
    /// Get the fill percentage (0.0 - 1.0) of the bottom chamber
    fn get_bottom_fill_percentage(&self) -> f32;
}
