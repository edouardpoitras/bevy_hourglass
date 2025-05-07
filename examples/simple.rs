//! A simple example demonstrating the bevy_hourglass plugin.

use bevy::prelude::*;
use bevy_hourglass::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HourglassPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_keyboard_input,
            update_ui,
        ))
        .run();
}

/// Component to mark the UI text entity
#[derive(Component)]
struct HourglassInfoText;

/// Component to mark the hourglass entity
#[derive(Component)]
struct MainHourglass;

/// Set up the scene with camera, hourglass, and UI
fn setup(mut commands: Commands) {
    // Add a 2D camera
    commands.spawn(Camera2d::default());
    
    // Add UI node and text
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            ..default()
        },
    )).with_children(|parent| {
        // Add text for displaying hourglass info
        parent.spawn((
            Node {
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            Text::new("Hourglass: 60s remaining"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor::from(Color::WHITE),
            TextLayout::default(),
            HourglassInfoText,
        ));
    });
    
    // Spawn a sprite-based hourglass using the helper function
    let hourglass_entity = spawn_hourglass(
        &mut commands,
        Duration::from_secs(10),
        Vec2::ZERO,
        Vec2::new(100.0, 200.0),
        Color::srgb(0.8, 0.8, 0.8),
        Color::srgb(0.9, 0.7, 0.2)
    );
    
    // Add the MainHourglass marker and configure additional properties
    commands.entity(hourglass_entity)
        .insert(MainHourglass);
    
    // Add instructions
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        Text::new("Press SPACE to flip the hourglass"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor::from(Color::WHITE),
        TextLayout::default(),
    ));
}

/// Handle keyboard input to flip the hourglass
fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut hourglasses: Query<&mut Hourglass, With<MainHourglass>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(mut hourglass) = hourglasses.single_mut() {
            // Flip the hourglass when space is pressed
            hourglass.flip();
        }
    }
}

/// Update the UI text with hourglass information
fn update_ui(
    hourglasses: Query<&Hourglass, With<MainHourglass>>,
    mut text_query: Query<&mut Text, With<HourglassInfoText>>,
) {
    if let (Ok(hourglass), Ok(mut text)) = (hourglasses.single(), text_query.single_mut()) {
        let remaining_secs = hourglass.get_time_remaining().as_secs();
        let status = if hourglass.is_flipping() {
            "Flipping"
        } else if hourglass.is_flipped() {
            "Flipped"
        } else {
            "Upright"
        };
        
        let running = if hourglass.is_running() { "Running" } else { "Stopped" };
        
        text.0 = format!(
            "Hourglass: {}s remaining | Status: {} | {}",
            remaining_secs, status, running
        ).into();
    }
}
