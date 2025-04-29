//! Sprite-based implementation of the Hourglass trait.

use bevy::prelude::*;
use std::time::Duration;

use crate::{traits::{Container, FlipBehavior, FlipStatus, Hourglass, Sand}, HourglassComponent};
use super::{SpriteSand, SpriteContainer};

/// A sprite-based implementation of an hourglass
#[derive(Component, Debug, Clone)]
#[require(HourglassComponent)]
pub struct SpriteHourglass {
    /// The sand component
    pub sand: SpriteSand,
    
    /// The container component
    pub container: SpriteContainer,
    
    /// The flip behavior
    pub flip_behavior: TimedFlipBehavior,
    
    /// Whether the hourglass is currently running
    pub running: bool,
    
    /// Whether the hourglass is currently flipped (upside down)
    pub flipped: bool,
    
    /// Whether the hourglass is currently in the process of flipping
    pub flipping: bool,
    
    /// Total time the hourglass can measure
    pub total_time: Duration,
    
    /// Remaining time in the hourglass
    pub remaining_time: Duration,
    
    /// Whether to update the sand while flipping
    pub update_during_flip: bool,
}

impl Default for SpriteHourglass {
    fn default() -> Self {
        let total_time = Duration::from_secs(60);
        Self {
            sand: SpriteSand::default(),
            container: SpriteContainer::default(),
            flip_behavior: TimedFlipBehavior::default(),
            running: true,
            flipped: false,
            flipping: false,
            total_time,
            remaining_time: total_time,
            update_during_flip: false,
        }
    }
}

impl Hourglass for SpriteHourglass {
    type SandImpl = SpriteSand;
    type ContainerImpl = SpriteContainer;
    type FlipBehaviorImpl = TimedFlipBehavior;
    
    fn flip(&mut self) {
        if !self.flipping {
            self.flipping = true;
            self.flip_behavior.start_flip();
        }
    }
    
    fn get_time_remaining(&self) -> Duration {
        self.remaining_time
    }
    
    fn get_total_time(&self) -> Duration {
        self.total_time
    }
    
    fn is_running(&self) -> bool {
        self.running
    }
    
    fn is_flipped(&self) -> bool {
        self.flipped
    }
    
    fn is_flipping(&self) -> bool {
        self.flipping
    }
    
    fn update(&mut self, delta: Duration) {
        // Only update sand if not flipping or if update_during_flip is enabled
        if !self.flipping || self.update_during_flip {
            // Update the sand
            self.sand.update(delta, self.flipped);
            
            // Update the remaining time based on sand levels
            if self.running {
                let fill_percentage = if self.flipped {
                    self.sand.get_bottom_fill_percentage()
                } else {
                    self.sand.get_top_fill_percentage()
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
        
        // Update the flip animation if flipping
        if self.flipping {
            match self.flip_behavior.update_flip(delta) {
                FlipStatus::InProgress(_) => {
                    // Update the container rotation
                    self.container.set_rotation(self.flip_behavior.get_rotation());
                }
                FlipStatus::Complete => {
                    // Flip is complete
                    self.flipping = false;
                    self.flipped = !self.flipped;
                    
                    // Reset the timer if it's empty
                    if self.remaining_time.is_zero() {
                        self.remaining_time = self.total_time;
                        self.running = true;
                    }
                }
            }
        }
    }
    
    fn sand(&self) -> &Self::SandImpl {
        &self.sand
    }
    
    fn sand_mut(&mut self) -> &mut Self::SandImpl {
        &mut self.sand
    }
    
    fn container(&self) -> &Self::ContainerImpl {
        &self.container
    }
    
    fn container_mut(&mut self) -> &mut Self::ContainerImpl {
        &mut self.container
    }
    
    fn flip_behavior(&self) -> &Self::FlipBehaviorImpl {
        &self.flip_behavior
    }
    
    fn flip_behavior_mut(&mut self) -> &mut Self::FlipBehaviorImpl {
        &mut self.flip_behavior
    }
}

/// A timed flip behavior for sprite hourglasses
#[derive(Component, Debug, Clone)]
pub struct TimedFlipBehavior {
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
    pub easing: crate::traits::EasingFunction,
}

impl Default for TimedFlipBehavior {
    fn default() -> Self {
        Self {
            current_rotation: 0.0,
            target_rotation: 0.0,
            is_flipping: false,
            flip_progress: 0.0,
            flip_duration: 1.0,
            flip_elapsed: 0.0,
            easing: crate::traits::EasingFunction::QuadraticInOut,
        }
    }
}

impl FlipBehavior for TimedFlipBehavior {
    fn start_flip(&mut self) {
        // Calculate target rotation (180 degrees from current)
        self.target_rotation = self.current_rotation + std::f32::consts::PI;
        
        // Start the flip animation
        self.is_flipping = true;
        self.flip_progress = 0.0;
        self.flip_elapsed = 0.0;
    }
    
    fn update_flip(&mut self, delta: Duration) -> FlipStatus {
        if !self.is_flipping {
            return FlipStatus::Complete;
        }
        
        // Update elapsed time and progress
        self.flip_elapsed += delta.as_secs_f32();
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
            return FlipStatus::Complete;
        }
        
        FlipStatus::InProgress(self.flip_progress)
    }
    
    fn get_rotation(&self) -> f32 {
        self.current_rotation
    }
    
    fn set_rotation(&mut self, rotation: f32) {
        self.current_rotation = rotation;
    }
    
    fn get_target_rotation(&self) -> f32 {
        self.target_rotation
    }
    
    fn set_target_rotation(&mut self, rotation: f32) {
        self.target_rotation = rotation;
    }
    
    fn is_flipping(&self) -> bool {
        self.is_flipping
    }
    
    fn get_flip_progress(&self) -> f32 {
        self.flip_progress
    }
}
