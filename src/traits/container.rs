//! Defines the Container trait for hourglass containers.

use bevy::prelude::*;

/// Trait for the physical container of an hourglass
pub trait Container {
    /// Render the container
    fn render(&self);
    
    /// Get the size of the container
    fn get_size(&self) -> Vec2;
    
    /// Set the size of the container
    fn set_size(&mut self, size: Vec2);
    
    /// Get the color of the container
    fn get_color(&self) -> Color;
    
    /// Set the color of the container
    fn set_color(&mut self, color: Color);
    
    /// Get the current rotation of the container in radians
    fn get_rotation(&self) -> f32;
    
    /// Set the rotation of the container in radians
    fn set_rotation(&mut self, rotation: f32);
    
    /// Get the shape of the container
    fn get_shape(&self) -> ContainerShape;
    
    /// Set the shape of the container
    fn set_shape(&mut self, shape: ContainerShape);
}

/// Defines the shape of the hourglass container
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContainerShape {
    /// Classic hourglass shape with two bulbs connected by a narrow neck
    Classic,
    /// Cylindrical shape with straight sides
    Cylindrical,
    /// Custom shape defined by a mesh or other means
    Custom,
}
