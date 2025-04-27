//! Defines bundles for creating sprite-based hourglasses.

use bevy::prelude::*;
use std::time::Duration;

use crate::bundles::HourglassBundle;
use crate::traits::EasingFunction;
use super::{SpriteHourglass, SpriteSand, SpriteContainer, TimedFlipBehavior};

/// Bundle for creating a sprite-based hourglass
#[derive(Bundle, Clone)]
pub struct SpriteHourglassBundle {
    /// The sprite hourglass component
    pub hourglass: SpriteHourglass,
    
    /// Sprite component for the container
    pub container_sprite: SpriteBundle,
    
    /// Sprite component for the top sand
    pub top_sand_sprite: SpriteBundle,
    
    /// Sprite component for the bottom sand
    pub bottom_sand_sprite: SpriteBundle,
}

impl Default for SpriteHourglassBundle {
    fn default() -> Self {
        // Create default sprites
        let container_sprite = SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.8, 0.8, 0.8),
                custom_size: Some(Vec2::new(100.0, 200.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        };
        
        let top_sand_sprite = SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.8, 0.6, 0.2),
                custom_size: Some(Vec2::new(80.0, 80.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 50.0, 0.1)),
            ..Default::default()
        };
        
        let bottom_sand_sprite = SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.8, 0.6, 0.2),
                custom_size: Some(Vec2::new(80.0, 0.0)), // Initially empty
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 0.1)),
            ..Default::default()
        };
        
        Self {
            hourglass: SpriteHourglass::default(),
            container_sprite,
            top_sand_sprite,
            bottom_sand_sprite,
        }
    }
}

impl SpriteHourglassBundle {
    /// Create a new sprite hourglass bundle with the specified duration
    pub fn new(duration: Duration) -> Self {
        let mut bundle = Self::default();
        
        // Set the duration
        bundle.hourglass.total_time = duration;
        bundle.hourglass.remaining_time = duration;
        
        bundle
    }
    
    /// Set the container color
    pub fn with_container_color(mut self, color: Color) -> Self {
        self.container_sprite.sprite.color = color;
        self.hourglass.container.color = color;
        self
    }
    
    /// Set the sand color
    pub fn with_sand_color(mut self, color: Color) -> Self {
        self.top_sand_sprite.sprite.color = color;
        self.bottom_sand_sprite.sprite.color = color;
        self.hourglass.sand.color = color;
        self
    }
    
    /// Set the size of the hourglass
    pub fn with_size(mut self, size: Vec2) -> Self {
        // Update container size
        self.container_sprite.sprite.custom_size = Some(size);
        self.hourglass.container.size = size;
        
        // Update sand size (80% of container width)
        let sand_width = size.x * 0.8;
        let sand_height = size.y * 0.4; // 40% of container height for each chamber
        
        self.top_sand_sprite.sprite.custom_size = Some(Vec2::new(sand_width, sand_height));
        self.bottom_sand_sprite.sprite.custom_size = Some(Vec2::new(sand_width, 0.0)); // Initially empty
        
        // Update positions
        self.top_sand_sprite.transform.translation.y = size.y * 0.25;
        self.bottom_sand_sprite.transform.translation.y = -size.y * 0.25;
        
        self
    }
    
    /// Set the easing function for the flip animation
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.hourglass.flip_behavior.easing = easing;
        self
    }
    
    /// Set the flip duration for the rotation animation
    pub fn with_flip_duration(mut self, duration: f32) -> Self {
        self.hourglass.flip_behavior.flip_duration = duration;
        self
    }
    
    /// Set whether to update the sand while flipping
    pub fn with_update_during_flip(mut self, update: bool) -> Self {
        self.hourglass.update_during_flip = update;
        self
    }
}

/// Helper function to spawn a sprite hourglass
pub fn spawn_sprite_hourglass(
    commands: &mut Commands,
    duration: Duration,
    position: Vec2,
    size: Vec2,
    container_color: Color,
    sand_color: Color,
) -> Entity {
    commands
        .spawn(SpriteHourglassBundle::new(duration)
            .with_size(size)
            .with_container_color(container_color)
            .with_sand_color(sand_color))
        .insert(Transform::from_translation(Vec3::new(position.x, position.y, 0.0)))
        .id()
}
