//! Sprite-based implementation of the Sand trait.

use bevy::prelude::*;
use std::time::Duration;
use crate::traits::Sand;

/// A sprite-based sand implementation for an hourglass
#[derive(Component, Debug, Clone)]
pub struct SpriteSand {
    /// Color of the sand
    pub color: Color,
    
    /// Fill percentage of the top chamber (0.0 - 1.0)
    pub top_fill: f32,
    
    /// Fill percentage of the bottom chamber (0.0 - 1.0)
    pub bottom_fill: f32,
    
    /// Flow rate in percentage per second
    pub flow_rate: f32,
    
    /// Handle to the top sand sprite
    pub top_sprite_handle: Option<Handle<Image>>,
    
    /// Handle to the bottom sand sprite
    pub bottom_sprite_handle: Option<Handle<Image>>,
}

impl Default for SpriteSand {
    fn default() -> Self {
        Self {
            color: Color::srgb(0.8, 0.6, 0.2),
            top_fill: 1.0,
            bottom_fill: 0.0,
            flow_rate: 0.02, // 2% per second
            top_sprite_handle: None,
            bottom_sprite_handle: None,
        }
    }
}

impl Sand for SpriteSand {
    fn update(&mut self, delta_time: Duration, is_flipped: bool) {
        // Calculate the amount to transfer based on flow rate and delta time
        let transfer_amount = self.flow_rate * delta_time.as_secs_f32();
        
        if is_flipped {
            // Sand flows from bottom to top
            let actual_transfer = transfer_amount.min(self.bottom_fill);
            self.bottom_fill -= actual_transfer;
            self.top_fill += actual_transfer;
        } else {
            // Sand flows from top to bottom
            let actual_transfer = transfer_amount.min(self.top_fill);
            self.top_fill -= actual_transfer;
            self.bottom_fill += actual_transfer;
        }
        
        // Ensure values stay in valid range
        self.top_fill = self.top_fill.clamp(0.0, 1.0);
        self.bottom_fill = self.bottom_fill.clamp(0.0, 1.0);
    }
    
    fn render(&self) {
        // Rendering is handled by Bevy's sprite rendering system
    }
    
    fn get_color(&self) -> Color {
        self.color
    }
    
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    
    fn get_top_fill_percentage(&self) -> f32 {
        self.top_fill
    }
    
    fn get_bottom_fill_percentage(&self) -> f32 {
        self.bottom_fill
    }
}
