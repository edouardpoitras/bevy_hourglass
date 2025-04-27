//! Defines the core Hourglass trait.

use std::time::Duration;

use super::{Sand, Container, FlipBehavior};

/// Status of the hourglass flip operation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlipStatus {
    /// Flip is in progress with a completion percentage (0.0 - 1.0)
    InProgress(f32),
    /// Flip has completed
    Complete,
}

/// Core trait defining hourglass behavior
pub trait Hourglass {
    /// The type implementing the Sand trait
    type SandImpl: Sand;
    
    /// The type implementing the Container trait
    type ContainerImpl: Container;
    
    /// The type implementing the FlipBehavior trait
    type FlipBehaviorImpl: FlipBehavior;
    
    /// Flip the hourglass
    fn flip(&mut self);
    
    /// Get the remaining time in the hourglass
    fn get_time_remaining(&self) -> Duration;
    
    /// Get the total time the hourglass can measure
    fn get_total_time(&self) -> Duration;
    
    /// Check if the hourglass is currently running
    fn is_running(&self) -> bool;
    
    /// Check if the hourglass is currently flipped (upside down)
    fn is_flipped(&self) -> bool;
    
    /// Check if the hourglass is currently in the process of flipping
    fn is_flipping(&self) -> bool;
    
    /// Update the hourglass state
    fn update(&mut self, delta: Duration);
    
    /// Get a reference to the sand implementation
    fn sand(&self) -> &Self::SandImpl;
    
    /// Get a mutable reference to the sand implementation
    fn sand_mut(&mut self) -> &mut Self::SandImpl;
    
    /// Get a reference to the container implementation
    fn container(&self) -> &Self::ContainerImpl;
    
    /// Get a mutable reference to the container implementation
    fn container_mut(&mut self) -> &mut Self::ContainerImpl;
    
    /// Get a reference to the flip behavior implementation
    fn flip_behavior(&self) -> &Self::FlipBehaviorImpl;
    
    /// Get a mutable reference to the flip behavior implementation
    fn flip_behavior_mut(&mut self) -> &mut Self::FlipBehaviorImpl;
}
