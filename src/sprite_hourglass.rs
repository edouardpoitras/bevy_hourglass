//! Sprite-based implementation for the hourglass.

use bevy::prelude::*;
use std::time::Duration;

use crate::components::Hourglass;

/// Marker component for the container sprite
#[derive(Component, Clone)]
pub struct ContainerSprite;

/// Marker component for the top sand sprite
#[derive(Component, Clone)]
pub struct TopSandSprite;

/// Marker component for the bottom sand sprite
#[derive(Component, Clone)]
pub struct BottomSandSprite;

/// Spawn a sprite-based hourglass
pub fn spawn_hourglass(
    commands: &mut Commands,
    duration: Duration,
    position: Vec2,
    size: Vec2,
    container_color: Color,
    sand_color: Color,
) -> Entity {
    // Create the hourglass component
    let hourglass = Hourglass {
        total_time: duration,
        remaining_time: duration,
        size,
        container_color,
        sand_color,
        upper_chamber: 1.0,
        lower_chamber: 0.0,
        ..Default::default()
    };
    
    // Create the main entity with the hourglass component and transform
    let entity = commands
        .spawn((
            hourglass,
            Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
        ))
        .id();
    
    // Add container as a child entity
    commands.entity(entity).with_children(|parent| {
        parent.spawn((
            ContainerSprite,
            Sprite::from_color(container_color, size),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ));
    });
    
    // Add top sand as a child entity
    commands.entity(entity).with_children(|parent| {
        parent.spawn((
            TopSandSprite,
            Sprite::from_color(sand_color, Vec2::new(size.x * 0.8, size.y * 0.4)),
            Transform::from_translation(Vec3::new(0.0, size.y * 0.25, 0.1)),
        ));
    });
    
    // Add bottom sand as a child entity
    commands.entity(entity).with_children(|parent| {
        parent.spawn((
            BottomSandSprite,
            Sprite::from_color(sand_color, Vec2::new(size.x * 0.8, 0.0)), // Initially empty
            Transform::from_translation(Vec3::new(0.0, -size.y * 0.25, 0.1)),
        ));
    });
    
    entity
}

/// System to update the sand sprites based on the hourglass state
pub fn update_sand_sprites(
    hourglass_query: Query<(Entity, &Hourglass)>,
    mut top_sand_query: Query<(&mut Sprite, &mut Transform), (With<TopSandSprite>, Without<BottomSandSprite>)>,
    mut bottom_sand_query: Query<(&mut Sprite, &mut Transform), (With<BottomSandSprite>, Without<TopSandSprite>)>,
    children_query: Query<&Children>,
) {
    for (entity, hourglass) in hourglass_query.iter() {
        // Get the children of the hourglass entity
        if let Ok(children) = children_query.get(entity) {
            // The visuals need to adjust depending on orientation
            // When flipped, the "top" visual becomes the lower chamber
            let (upper_fill, lower_fill) = if !hourglass.flipped {
                (hourglass.upper_chamber, hourglass.lower_chamber)
            } else {
                (hourglass.lower_chamber, hourglass.upper_chamber)
            };
            
            // Find the top and bottom sand sprites
            for child in children.iter() {
                // Update top sand sprite (visual top, which could be either chamber)
                if let Ok((mut sprite, mut transform)) = top_sand_query.get_mut(child) {
                    let sand_width = hourglass.size.x * 0.8;
                    let max_height = hourglass.size.y * 0.4;
                    let height = max_height * upper_fill;
                    
                    // Create a new sprite with the updated size
                    *sprite = Sprite::from_color(sprite.color, Vec2::new(sand_width, height));
                    
                    // Position depends on whether this is actually the top or bottom chamber
                    if !hourglass.flipped {
                        // This is the actual top chamber
                        // Update transform to keep the sand anchored at the top of the chamber
                        // The bottom of the top chamber should remain fixed
                        // Only the height of the sprite should change as sand drains
                        transform.translation.y = hourglass.size.y * 0.25 - (max_height - height) * 0.5;
                    } else {
                        // This is actually the bottom chamber (visually at the top due to flipping)
                        // Update transform to keep the sand anchored at the very bottom of the hourglass
                        // As sand fills, the sprite should grow upward from the bottom
                        transform.translation.y = hourglass.size.y * 0.25 - (max_height - height);
                    }
                }
                
                // Update bottom sand sprite (visual bottom, which could be either chamber)
                if let Ok((mut sprite, mut transform)) = bottom_sand_query.get_mut(child) {
                    let sand_width = hourglass.size.x * 0.8;
                    let max_height = hourglass.size.y * 0.4;
                    let height = max_height * lower_fill;
                    
                    // Create a new sprite with the updated size
                    *sprite = Sprite::from_color(sprite.color, Vec2::new(sand_width, height));
                    
                    // Position depends on whether this is actually the bottom or top chamber
                    if !hourglass.flipped {
                        // This is the actual bottom chamber
                        // Update transform to keep the sand anchored at the very bottom of the hourglass
                        // As sand fills, the sprite should grow upward from the bottom
                        // The origin of the sprite is at its center, so we need to offset by half its height
                        transform.translation.y = -hourglass.size.y * 0.45 + height * 0.5;
                    } else {
                        // This is actually the top chamber (visually at the bottom due to flipping)
                        // Update transform to keep the sand anchored at the top of the chamber
                        // Only the height of the sprite should change as sand drains
                        transform.translation.y = -hourglass.size.y * 0.25 - (max_height - height) * 0.5;
                    }
                }
            }
        }
    }
}
