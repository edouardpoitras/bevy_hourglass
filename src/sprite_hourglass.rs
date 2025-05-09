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
    // Create the hourglass component using the new method to ensure proper flow rate calculation
    let mut hourglass = Hourglass::new(duration);
    
    // Set additional properties
    hourglass.size = size;
    hourglass.container_color = container_color;
    hourglass.sand_color = sand_color;
    hourglass.upper_chamber = 1.0;
    hourglass.lower_chamber = 0.0;
    
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

/// System to update the container sprite rotation
pub fn update_container_sprite(
    hourglass_query: Query<(Entity, &Hourglass)>,
    mut container_query: Query<&mut Transform, With<ContainerSprite>>,
    children_query: Query<&Children>,
) {
    for (entity, hourglass) in hourglass_query.iter() {
        // Get the children of the hourglass entity
        if let Ok(children) = children_query.get(entity) {
            // Apply rotation to the container sprite
            for child in children.iter() {
                if let Ok(mut transform) = container_query.get_mut(child) {
                    transform.rotation = Quat::from_rotation_z(hourglass.current_rotation);
                }
            }
        }
    }
}

/// System to update the top sand sprite
pub fn update_top_sand_sprite(
    hourglass_query: Query<(Entity, &Hourglass)>,
    mut top_sand_query: Query<(&mut Sprite, &mut Transform), With<TopSandSprite>>,
    children_query: Query<&Children>,
) {
    for (entity, hourglass) in hourglass_query.iter() {
        // Get the children of the hourglass entity
        if let Ok(children) = children_query.get(entity) {
            let upper_fill = hourglass.upper_chamber;
            
            // Find the top sand sprite
            for child in children.iter() {
                if let Ok((mut sprite, mut transform)) = top_sand_query.get_mut(child) {
                    let sand_width = hourglass.size.x * 0.8;
                    let max_height = hourglass.size.y * 0.4;
                    let height = max_height * upper_fill;
                    
                    // Create a new sprite with the updated size
                    *sprite = Sprite::from_color(sprite.color, Vec2::new(sand_width, height));
                    
                    // Apply rotation to match the hourglass orientation
                    transform.rotation = Quat::from_rotation_z(hourglass.current_rotation);
                    
                    // Position the sand based on the chamber fill
                    // When not flipped, this is at the top of the hourglass
                    // When flipped, this is at the bottom of the hourglass (but visually at the top due to rotation)
                    let base_y = hourglass.size.y * 0.25;
                    transform.translation.y = base_y - (max_height - height) * 0.5;
                }
            }
        }
    }
}

/// System to update the bottom sand sprite
pub fn update_bottom_sand_sprite(
    hourglass_query: Query<(Entity, &Hourglass)>,
    mut bottom_sand_query: Query<(&mut Sprite, &mut Transform), With<BottomSandSprite>>,
    children_query: Query<&Children>,
) {
    for (entity, hourglass) in hourglass_query.iter() {
        // Get the children of the hourglass entity
        if let Ok(children) = children_query.get(entity) {
            let lower_fill = hourglass.lower_chamber;
            
            // Find the bottom sand sprite
            for child in children.iter() {
                if let Ok((mut sprite, mut transform)) = bottom_sand_query.get_mut(child) {
                    let sand_width = hourglass.size.x * 0.8;
                    let max_height = hourglass.size.y * 0.4;
                    let height = max_height * lower_fill;
                    
                    // Create a new sprite with the updated size
                    *sprite = Sprite::from_color(sprite.color, Vec2::new(sand_width, height));
                    
                    // Apply rotation to match the hourglass orientation
                    transform.rotation = Quat::from_rotation_z(hourglass.current_rotation);
                    
                    // Position the sand based on the chamber fill
                    // When not flipped, this is at the bottom of the hourglass
                    // When flipped, this is at the top of the hourglass (but visually at the bottom due to rotation)
                    let base_y = -hourglass.size.y * 0.45;
                    transform.translation.y = base_y + height * 0.5;
                }
            }
        }
    }
}
