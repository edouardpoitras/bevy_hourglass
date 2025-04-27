//! Systems for handling user interaction with hourglasses.

use bevy::prelude::*;
use crate::components::{HourglassComponent, RotationState, InteractableHourglass};
use crate::events::HourglassInteractionEvent;
use crate::events::InteractionType;

/// System that handles mouse interaction with hourglasses
pub fn handle_hourglass_interaction(
    mut hourglasses: Query<(
        Entity,
        &mut HourglassComponent,
        &mut RotationState,
        &mut InteractableHourglass,
        &GlobalTransform,
    )>,
    windows: Query<&Window>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut interaction_events: EventWriter<HourglassInteractionEvent>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    // Get the primary window
    let window = windows.single().expect("No window found");
    
    // Get the cursor position if available
    let cursor_position = window.cursor_position();
    
    // Get the camera
    let (camera, camera_transform) = camera_q.single().expect("No camera found");
    
    if let Some(cursor_position) = cursor_position {
        // Convert cursor position to world coordinates
        let cursor_world_position = cursor_to_world(camera, camera_transform, cursor_position);
        
        for (entity, mut hourglass, mut rotation, mut interactable, transform) in hourglasses.iter_mut() {
            // Check if the cursor is over the hourglass
            let hourglass_position = transform.translation().truncate();
            let hourglass_size = Vec2::new(100.0, 200.0); // TODO: Get actual size from container
            
            let is_hovering = is_point_in_rect(
                cursor_world_position,
                hourglass_position,
                hourglass_size,
                transform.rotation().to_euler(EulerRot::ZYX).0,
            );
            
            // Handle hover events
            if is_hovering && !interactable.is_interacting {
                interaction_events.write(HourglassInteractionEvent {
                    entity,
                    interaction_type: InteractionType::Hover,
                });
            } else if !is_hovering && interactable.is_interacting {
                interaction_events.write(HourglassInteractionEvent {
                    entity,
                    interaction_type: InteractionType::HoverExit,
                });
            }
            
            // Update interaction state
            interactable.is_interacting = is_hovering;
            
            // Handle mouse clicks
            if is_hovering && mouse_input.just_pressed(MouseButton::Left) {
                interaction_events.write(HourglassInteractionEvent {
                    entity,
                    interaction_type: InteractionType::Click,
                });
                
                // If the hourglass can be flipped, start flipping
                if interactable.can_flip && !hourglass.flipping {
                    if interactable.mouse_follow {
                        // For mouse following, we'll update the rotation in the drag system
                        interaction_events.write(HourglassInteractionEvent {
                            entity,
                            interaction_type: InteractionType::DragStart,
                        });
                    } else {
                        // For normal flipping, just start the flip animation
                        hourglass.start_flip();
                    }
                }
            }
            
            // Handle mouse following for rotation
            if interactable.mouse_follow && interactable.is_interacting && mouse_input.pressed(MouseButton::Left) {
                // Calculate angle between hourglass and cursor
                let hourglass_to_cursor = cursor_world_position - hourglass_position;
                let angle = hourglass_to_cursor.y.atan2(hourglass_to_cursor.x);
                
                // Apply sensitivity
                let target_angle = angle * interactable.mouse_sensitivity;
                
                // Clamp to min/max angles
                let clamped_angle = target_angle.clamp(interactable.min_angle, interactable.max_angle);
                
                // Apply the rotation directly
                rotation.current_rotation = clamped_angle;
                
                // Send drag event
                interaction_events.write(HourglassInteractionEvent {
                    entity,
                    interaction_type: InteractionType::Drag,
                });
                
                // Check if we should snap to an extreme
                let snap_to_min = (clamped_angle - interactable.min_angle).abs() < interactable.snap_threshold;
                let snap_to_max = (clamped_angle - interactable.max_angle).abs() < interactable.snap_threshold;
                
                if snap_to_min || snap_to_max {
                    // Snap to the extreme
                    rotation.current_rotation = if snap_to_min {
                        interactable.min_angle
                    } else {
                        interactable.max_angle
                    };
                    
                    // Update the hourglass flipped state based on which extreme we snapped to
                    let new_flipped = snap_to_min;
                    if hourglass.flipped != new_flipped {
                        hourglass.flip();
                    }
                }
            }
            
            // Handle mouse release
            if interactable.mouse_follow && mouse_input.just_released(MouseButton::Left) {
                interaction_events.write(HourglassInteractionEvent {
                    entity,
                    interaction_type: InteractionType::DragEnd,
                });
            }
        }
    }
}

/// Convert cursor position to world coordinates
fn cursor_to_world(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    cursor_pos: Vec2,
) -> Vec2 {
    let world_pos = camera.viewport_to_world(camera_transform, cursor_pos)
        .expect("Failed to convert viewport to world")
        .origin;
    
    world_pos.truncate()
}

/// Check if a point is inside a rotated rectangle
fn is_point_in_rect(point: Vec2, rect_center: Vec2, rect_size: Vec2, rect_rotation: f32) -> bool {
    // Translate point to rectangle's coordinate system
    let translated_point = point - rect_center;
    
    // Rotate point to align with rectangle's axes
    let cos_rot = rect_rotation.cos();
    let sin_rot = rect_rotation.sin();
    let rotated_point = Vec2::new(
        translated_point.x * cos_rot + translated_point.y * sin_rot,
        -translated_point.x * sin_rot + translated_point.y * cos_rot,
    );
    
    // Check if the rotated point is inside the rectangle
    let half_size = rect_size * 0.5;
    rotated_point.x >= -half_size.x
        && rotated_point.x <= half_size.x
        && rotated_point.y >= -half_size.y
        && rotated_point.y <= half_size.y
}
