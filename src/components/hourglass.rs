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
            flow_rate: 0.02, // 2% per second
        }
    }
}

impl Hourglass {
    /// Create a new hourglass with the specified total time
    pub fn new(total_time: Duration) -> Self {
        Self {
            total_time,
            remaining_time: total_time,
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
            // regardless of hourglass orientation
            let active_chamber = if self.flipped {
                self.lower_chamber
            } else {
                self.upper_chamber
            };
            
            self.remaining_time = Duration::from_secs_f32(
                active_chamber * self.total_time.as_secs_f32()
            );
            
            // Check if the hourglass is empty
            if active_chamber <= 0.0 {
                self.running = false;
            }
        }
        
        // For flipping animation, simply update the state immediately
        if self.flipping {
            self.flipping = false;
            self.flipped = !self.flipped;
            
            // Update the rotation based on flipped state
            self.current_rotation = if self.flipped {
                std::f32::consts::PI // 180 degrees
            } else {
                0.0
            };
            
            // Reset the timer if it's empty
            if self.remaining_time.is_zero() {
                self.remaining_time = self.total_time;
                self.running = true;
            }
            
            // Swap chambers when flipped
            std::mem::swap(&mut self.upper_chamber, &mut self.lower_chamber);
        }
    }
    
    /// Update the sand levels
    fn update_sand(&mut self, delta: Duration) {
        // Calculate the amount to transfer based on flow rate and delta time
        let transfer_amount = self.flow_rate * delta.as_secs_f32();
        
        // Sand always flows from upper to lower chamber, regardless of orientation
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
    
    /// Get the fill percentage (0.0 - 1.0)
    pub fn get_fill_percentage(&self) -> f32 {
        self.remaining_time.as_secs_f32() / self.total_time.as_secs_f32()
    }
    
    /// Get the time remaining in the hourglass
    pub fn get_time_remaining(&self) -> Duration {
        self.remaining_time
    }
    
    /// Get the total time the hourglass can measure
    pub fn get_total_time(&self) -> Duration {
        self.total_time
    }
    
    /// Check if the hourglass is currently running
    pub fn is_running(&self) -> bool {
        self.running
    }
    
    /// Check if the hourglass is currently flipped (upside down)
    pub fn is_flipped(&self) -> bool {
        self.flipped
    }
    
    /// Check if the hourglass is currently in the process of flipping
    pub fn is_flipping(&self) -> bool {
        self.flipping
    }
    
    /// Get the current rotation of the hourglass
    pub fn get_rotation(&self) -> f32 {
        self.current_rotation
    }
    
    /// Get the upper chamber fill percentage (0.0 - 1.0)
    pub fn get_upper_fill_percentage(&self) -> f32 {
        self.upper_chamber
    }
    
    /// Get the lower chamber fill percentage (0.0 - 1.0)
    pub fn get_lower_fill_percentage(&self) -> f32 {
        self.lower_chamber
    }
}
