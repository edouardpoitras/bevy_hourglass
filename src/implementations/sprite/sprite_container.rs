//! Sprite-based implementation of the Container trait.

use bevy::prelude::*;
use crate::traits::{Container, ContainerShape};

/// A sprite-based container for an hourglass
#[derive(Component, Debug, Clone)]
pub struct SpriteContainer {
    /// Size of the container
    pub size: Vec2,
    
    /// Color of the container
    pub color: Color,
    
    /// Current rotation in radians
    pub rotation: f32,
    
    /// Shape of the container
    pub shape: ContainerShape,
    
    /// Handle to the container sprite
    pub sprite_handle: Option<Handle<Image>>,
}

impl Default for SpriteContainer {
    fn default() -> Self {
        Self {
            size: Vec2::new(100.0, 200.0),
            color: Color::srgb(0.8, 0.8, 0.8),
            rotation: 0.0,
            shape: ContainerShape::Classic,
            sprite_handle: None,
        }
    }
}

impl Container for SpriteContainer {
    fn render(&self) {
        // Rendering is handled by Bevy's sprite rendering system
    }
    
    fn get_size(&self) -> Vec2 {
        self.size
    }
    
    fn set_size(&mut self, size: Vec2) {
        self.size = size;
    }
    
    fn get_color(&self) -> Color {
        self.color
    }
    
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    
    fn get_rotation(&self) -> f32 {
        self.rotation
    }
    
    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }
    
    fn get_shape(&self) -> ContainerShape {
        self.shape
    }
    
    fn set_shape(&mut self, shape: ContainerShape) {
        self.shape = shape;
    }
}
