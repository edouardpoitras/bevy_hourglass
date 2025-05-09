use std::time::Duration;
use bevy::prelude::*;

/// Core component for an hourglass
#[derive(Component, Debug, Clone)]
pub struct Hourglass {
    // Timer properties
    /// Total time the hourglass can measure
    pub total_time: Duration,
    /// Remaining time in the hourglass
    pub remaining_time: Duration,
    /// Whether the hourglass is currently running
    pub running: bool,
    
    // State properties
    /// Whether the hourglass is currently flipped (upside down)
    pub flipped: bool,
    /// Whether the hourglass is currently in the process of flipping
    pub flipping: bool,
    
    // Rotation properties
    /// Current rotation in radians
    pub current_rotation: f32,
    
    // Visual properties
    /// Color of the container
    pub container_color: Color,
    /// Color of the sand
    pub sand_color: Color,
    /// Size of the hourglass
    pub size: Vec2,
    
    // Sand properties
    /// Fill percentage of the upper chamber (0.0 - 1.0)
    pub upper_chamber: f32,
    /// Fill percentage of the lower chamber (0.0 - 1.0)
    pub lower_chamber: f32,
    /// Flow rate in percentage per second
    pub flow_rate: f32,
}

impl Default for Hourglass {
    fn default() -> Self {
        Self {
            // Timer properties
            total_time: Duration::from_secs(60),
            remaining_time: Duration::from_secs(60),
            running: true,
            
            // State properties
            flipped: false,
            flipping: false,
            
            // Rotation properties
            current_rotation: 0.0,
            
            // Visual properties
            container_color: Color::srgb(0.8, 0.8, 0.8),
            sand_color: Color::srgb(0.8, 0.6, 0.2),
            size: Vec2::new(100.0, 200.0),
            
            // Sand properties
            upper_chamber: 1.0,
            lower_chamber: 0.0,
            flow_rate: 1.0 / 60.0,
        }
    }
}

impl Hourglass {
    /// Create a new hourglass with the specified total time
    pub fn new(total_time: Duration) -> Self {
        // Calculate flow rate based on total time
        // Flow rate should be 1.0 / total_time_in_seconds
        // This ensures the hourglass empties exactly when the time is up
        let flow_rate = if total_time.as_secs_f32() > 0.0 {
            1.0 / total_time.as_secs_f32()
        } else {
            1.0 / 60.0 // Default to 60s if total_time is zero
        };
        
        Self {
            total_time,
            remaining_time: total_time,
            flow_rate,
            ..Default::default()
        }
    }
    
    /// Update the hourglass state
    pub fn update(&mut self, delta: Duration) {
        // Update sand levels and remaining time
        if self.running {
            // Update sand flow
            self.update_sand(delta);
            
            // Update remaining time based on sand in the upper chamber
            // Since we swap chambers when flipping, upper is always physically at the top
            self.remaining_time = Duration::from_secs_f32(
                self.upper_chamber * self.total_time.as_secs_f32()
            );
            
            // Check if the hourglass is empty
            if self.upper_chamber <= 0.0 {
                self.running = false;
            }
        }
        
        // For flipping animation, simply update the state immediately
        if self.flipping {
            self.flipping = false;
            self.flipped = !self.flipped;
            
            // Invert the timer (if 2s left in a 10s timer, it should read 8s after flipping)
            self.remaining_time = self.total_time - self.remaining_time;
            
            // Swap chambers when flipped - this ensures upper is always physically at the top
            std::mem::swap(&mut self.upper_chamber, &mut self.lower_chamber);
            
            // Always ensure the timer is running if there's sand in the upper chamber
            if !self.running && self.upper_chamber > 0.0 {
                self.running = true;
            }
        }
    }
    
    /// Update the sand levels
    fn update_sand(&mut self, delta: Duration) {
        // Calculate the amount to transfer based on flow rate and delta time
        let transfer_amount = self.flow_rate * delta.as_secs_f32();
        
        // Sand always flows from upper to lower chamber (gravity pulls down)
        // Since we swap chambers when flipping, upper is always physically at the top
        let actual_transfer = transfer_amount.min(self.upper_chamber);
        self.upper_chamber -= actual_transfer;
        self.lower_chamber += actual_transfer;
        
        // Ensure values stay in valid range
        self.upper_chamber = self.upper_chamber.clamp(0.0, 1.0);
        self.lower_chamber = self.lower_chamber.clamp(0.0, 1.0);
    }
    
    /// Start flipping the hourglass
    pub fn flip(&mut self) {
        if !self.flipping {
            self.flipping = true;
        }
    }
}
