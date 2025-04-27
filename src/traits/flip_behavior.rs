//! Defines the FlipBehavior trait for hourglass flipping animations.

use std::time::Duration;

use super::hourglass::FlipStatus;

/// Trait for defining how an hourglass flips
pub trait FlipBehavior {
    /// Initialize a new flip animation
    fn start_flip(&mut self);
    
    /// Update the flip animation
    /// 
    /// Returns the current status of the flip operation
    fn update_flip(&mut self, delta: Duration) -> FlipStatus;
    
    /// Get the current rotation of the hourglass (in radians)
    fn get_rotation(&self) -> f32;
    
    /// Set the rotation of the hourglass (in radians)
    fn set_rotation(&mut self, rotation: f32);
    
    /// Get the target rotation for the current flip
    fn get_target_rotation(&self) -> f32;
    
    /// Set the target rotation for the current flip
    fn set_target_rotation(&mut self, rotation: f32);
    
    /// Check if the hourglass is currently flipping
    fn is_flipping(&self) -> bool;
    
    /// Get the current flip progress (0.0 - 1.0)
    fn get_flip_progress(&self) -> f32;
}

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
