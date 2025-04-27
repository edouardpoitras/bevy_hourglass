//! Defines the core hourglass component.

use std::time::Duration;
use bevy::prelude::*;

/// Core component for an hourglass
#[derive(Component, Debug, Clone)]
pub struct HourglassComponent {
    /// Total time the hourglass can measure
    pub total_time: Duration,
    
    /// Remaining time in the hourglass
    pub remaining_time: Duration,
    
    /// Whether the hourglass is currently running
    pub running: bool,
    
    /// Whether the hourglass is currently flipped (upside down)
    pub flipped: bool,
    
    /// Whether the hourglass is currently in the process of flipping
    pub flipping: bool,
    
    /// Whether to update the sand while flipping
    pub update_during_flip: bool,
}

impl Default for HourglassComponent {
    fn default() -> Self {
        Self {
            total_time: Duration::from_secs(60),
            remaining_time: Duration::from_secs(60),
            running: true,
            flipped: false,
            flipping: false,
            update_during_flip: false,
        }
    }
}

impl HourglassComponent {
    /// Create a new hourglass component with the specified total time
    pub fn new(total_time: Duration) -> Self {
        Self {
            total_time,
            remaining_time: total_time,
            ..Default::default()
        }
    }
    
    /// Update the hourglass state
    pub fn update(&mut self, delta: Duration) {
        if !self.running || self.flipping {
            return;
        }
        
        if self.remaining_time > delta {
            self.remaining_time -= delta;
        } else {
            self.remaining_time = Duration::ZERO;
            self.running = false;
        }
    }
    
    /// Flip the hourglass
    pub fn flip(&mut self) {
        self.flipped = !self.flipped;
        
        // Reset the timer if it's empty
        if self.remaining_time.is_zero() {
            self.remaining_time = self.total_time;
            self.running = true;
        }
    }
    
    /// Get the fill percentage (0.0 - 1.0)
    pub fn get_fill_percentage(&self) -> f32 {
        self.remaining_time.as_secs_f32() / self.total_time.as_secs_f32()
    }
    
    /// Start the flipping animation
    pub fn start_flip(&mut self) {
        self.flipping = true;
    }
    
    /// Complete the flipping animation
    pub fn complete_flip(&mut self) {
        self.flipping = false;
        self.flip();
    }
}
