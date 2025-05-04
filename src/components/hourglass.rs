use std::time::Duration;
use bevy::prelude::*;

/// Easing function for smooth animations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EasingFunction {
    /// Linear interpolation (constant speed)
    Linear,
    /// Quadratic ease-in (accelerating)
    QuadraticIn,
    /// Quadratic ease-out (decelerating)
    QuadraticOut,
    /// Quadratic ease-in-out (accelerating then decelerating)
    QuadraticInOut,
    /// Cubic ease-in (stronger acceleration)
    CubicIn,
    /// Cubic ease-out (stronger deceleration)
    CubicOut,
    /// Cubic ease-in-out (stronger acceleration then deceleration)
    CubicInOut,
}

impl EasingFunction {
    /// Apply the easing function to a value between 0.0 and 1.0
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            EasingFunction::Linear => t,
            EasingFunction::QuadraticIn => t * t,
            EasingFunction::QuadraticOut => t * (2.0 - t),
            EasingFunction::QuadraticInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            EasingFunction::CubicIn => t * t * t,
            EasingFunction::CubicOut => {
                let t1 = t - 1.0;
                1.0 + t1 * t1 * t1
            }
            EasingFunction::CubicInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let t1 = t - 1.0;
                    1.0 + 4.0 * t1 * t1 * t1
                }
            }
        }
    }
}

/// Defines the shape of the hourglass container
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContainerShape {
    /// Classic hourglass shape with two bulbs connected by a narrow neck
    Classic,
    /// Cylindrical shape with straight sides
    Cylindrical,
}

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
    /// Whether to update the sand while flipping
    pub update_during_flip: bool,
    
    // Rotation properties
    /// Current rotation in radians
    pub current_rotation: f32,
    /// Target rotation in radians
    pub target_rotation: f32,
    /// Duration of the flip animation in seconds
    pub flip_duration: f32,
    /// Elapsed time of the current flip animation in seconds
    pub flip_elapsed: f32,
    /// Easing function to use for the flip animation
    pub easing: EasingFunction,
    
    // Visual properties
    /// Color of the container
    pub container_color: Color,
    /// Color of the sand
    pub sand_color: Color,
    /// Size of the hourglass
    pub size: Vec2,
    /// Shape of the container
    pub shape: ContainerShape,
    
    // Sand properties
    /// Fill percentage of the top chamber (0.0 - 1.0)
    pub top_fill: f32,
    /// Fill percentage of the bottom chamber (0.0 - 1.0)
    pub bottom_fill: f32,
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
            update_during_flip: false,
            
            // Rotation properties
            current_rotation: 0.0,
            target_rotation: 0.0,
            flip_duration: 1.0,
            flip_elapsed: 0.0,
            easing: EasingFunction::QuadraticInOut,
            
            // Visual properties
            container_color: Color::srgb(0.8, 0.8, 0.8),
            sand_color: Color::srgb(0.8, 0.6, 0.2),
            size: Vec2::new(100.0, 200.0),
            shape: ContainerShape::Classic,
            
            // Sand properties
            top_fill: 1.0,
            bottom_fill: 0.0,
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
        let delta_seconds = delta.as_secs_f32();
        
        // Update flip animation if flipping
        if self.flipping {
            self.flip_elapsed += delta_seconds;
            let progress = (self.flip_elapsed / self.flip_duration).min(1.0);
            
            // Apply easing function
            let eased_progress = self.easing.apply(progress);
            
            // Interpolate between current and target rotation
            // Calculate exact 180 degree rotation
            let start_rotation = self.current_rotation;
            let target_rotation = if self.flipped {
                0.0 // Rotate to upright position
            } else {
                std::f32::consts::PI // Rotate to upside down position
            };
            
            // Ensure we're taking the shortest path to the target rotation
            let mut rotation_diff = target_rotation - start_rotation;
            while rotation_diff > std::f32::consts::PI {
                rotation_diff -= 2.0 * std::f32::consts::PI;
            }
            while rotation_diff < -std::f32::consts::PI {
                rotation_diff += 2.0 * std::f32::consts::PI;
            }
            
            self.current_rotation = start_rotation + rotation_diff * eased_progress;
            
            // Check if the flip is complete
            if progress >= 1.0 {
                self.flipping = false;
                self.flipped = !self.flipped;
                self.current_rotation = target_rotation; // Ensure exact target rotation
                
                // Reset the timer if it's empty
                if self.remaining_time.is_zero() {
                    self.remaining_time = self.total_time;
                    self.running = true;
                }
            }
        }
        
        // Only update sand if not flipping or if update_during_flip is enabled
        if !self.flipping || self.update_during_flip {
            // Update sand levels
            self.update_sand(delta);
            
            // Update the remaining time based on sand levels
            if self.running {
                let fill_percentage = if self.flipped {
                    self.bottom_fill
                } else {
                    self.top_fill
                };
                
                self.remaining_time = Duration::from_secs_f32(
                    fill_percentage * self.total_time.as_secs_f32()
                );
                
                // Check if the hourglass is empty
                if fill_percentage <= 0.0 {
                    self.running = false;
                }
            }
        }
    }
    
    /// Update the sand levels
    fn update_sand(&mut self, delta: Duration) {
        // Calculate the amount to transfer based on flow rate and delta time
        let transfer_amount = self.flow_rate * delta.as_secs_f32();
        
        if self.flipped {
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
    
    /// Start flipping the hourglass
    pub fn flip(&mut self) {
        if !self.flipping {
            self.flipping = true;
            self.flip_elapsed = 0.0;
            // Target rotation is handled in the update method
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
    
    /// Get the top fill percentage (0.0 - 1.0)
    pub fn get_top_fill_percentage(&self) -> f32 {
        self.top_fill
    }
    
    /// Get the bottom fill percentage (0.0 - 1.0)
    pub fn get_bottom_fill_percentage(&self) -> f32 {
        self.bottom_fill
    }
}
