//! Defines components and bundles for creating sprite-based hourglasses.

use bevy::prelude::*;
use std::time::Duration;

use crate::traits::EasingFunction;
use super::SpriteHourglass;

/// Marker component for the container sprite
#[derive(Component, Clone)]
pub struct ContainerSprite;

/// Marker component for the top sand sprite
#[derive(Component, Clone)]
pub struct TopSandSprite;

/// Marker component for the bottom sand sprite
#[derive(Component, Clone)]
pub struct BottomSandSprite;

/// Bundle for creating a sprite-based hourglass
#[derive(Bundle, Clone)]
pub struct SpriteHourglassBundle {
    /// The sprite hourglass component
    pub hourglass: SpriteHourglass,
    
    /// Sprite component for the container
    pub container_sprite: ContainerSprite,
    
    /// Container sprite
    pub container: Sprite,
    
    /// Container transform
    pub container_transform: Transform,
    
    /// Marker component for the top sand sprite
    pub top_sand_sprite: TopSandSprite,
    
    /// Top sand sprite
    pub top_sand: Sprite,
    
    /// Top sand transform
    pub top_sand_transform: Transform,
    
    /// Marker component for the bottom sand sprite
    pub bottom_sand_sprite: BottomSandSprite,
    
    /// Bottom sand sprite
    pub bottom_sand: Sprite,
    
    /// Bottom sand transform
    pub bottom_sand_transform: Transform,
}

impl Default for SpriteHourglassBundle {
    fn default() -> Self {
        // Create default sprites
        let container = Sprite {
            color: Color::srgb(0.8, 0.8, 0.8),
            custom_size: Some(Vec2::new(100.0, 200.0)),
            ..Default::default()
        };
        let container_transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        
        let top_sand = Sprite {
            color: Color::srgb(0.8, 0.6, 0.2),
            custom_size: Some(Vec2::new(80.0, 80.0)),
            ..Default::default()
        };
        let top_sand_transform = Transform::from_translation(Vec3::new(0.0, 50.0, 0.1));
        
        let bottom_sand = Sprite {
            color: Color::srgb(0.8, 0.6, 0.2),
            custom_size: Some(Vec2::new(80.0, 0.0)), // Initially empty
            ..Default::default()
        };
        let bottom_sand_transform = Transform::from_translation(Vec3::new(0.0, -50.0, 0.1));
        
        Self {
            hourglass: SpriteHourglass::default(),
            container_sprite: ContainerSprite,
            container,
            container_transform,
            top_sand_sprite: TopSandSprite,
            top_sand,
            top_sand_transform,
            bottom_sand_sprite: BottomSandSprite,
            bottom_sand,
            bottom_sand_transform,
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
        self.container.color = color;
        self.hourglass.container.color = color;
        self
    }
    
    /// Set the sand color
    pub fn with_sand_color(mut self, color: Color) -> Self {
        self.top_sand.color = color;
        self.bottom_sand.color = color;
        self.hourglass.sand.color = color;
        self
    }
    
    /// Set the size of the hourglass
    pub fn with_size(mut self, size: Vec2) -> Self {
        // Update container size
        self.container.custom_size = Some(size);
        self.hourglass.container.size = size;
        
        // Update sand size (80% of container width)
        let sand_width = size.x * 0.8;
        let sand_height = size.y * 0.4; // 40% of container height for each chamber
        
        self.top_sand.custom_size = Some(Vec2::new(sand_width, sand_height));
        self.bottom_sand.custom_size = Some(Vec2::new(sand_width, 0.0)); // Initially empty
        
        // Update positions
        self.top_sand_transform.translation.y = size.y * 0.25;
        self.bottom_sand_transform.translation.y = -size.y * 0.25;
        
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
    let bundle = SpriteHourglassBundle::new(duration)
        .with_size(size)
        .with_container_color(container_color)
        .with_sand_color(sand_color);
    
    // Create the main entity with the hourglass component and transform
    let entity = commands
        .spawn((
            bundle.hourglass.clone(),
            Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
        ))
        .id();
    
    // Add container as a child entity
    commands.entity(entity).with_children(|parent| {
        parent.spawn((
            bundle.container_sprite,
            bundle.container,
            bundle.container_transform,
        ));
    });
    
    // Add top sand as a child entity
    commands.entity(entity).with_children(|parent| {
        parent.spawn((
            bundle.top_sand_sprite,
            bundle.top_sand,
            bundle.top_sand_transform,
        ));
    });
    
    // Add bottom sand as a child entity
    commands.entity(entity).with_children(|parent| {
        parent.spawn((
            bundle.bottom_sand_sprite,
            bundle.bottom_sand,
            bundle.bottom_sand_transform,
        ));
    });
    
    entity
}
