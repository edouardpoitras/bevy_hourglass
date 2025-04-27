//! A simple example demonstrating the bevy_hourglass plugin.

use bevy::prelude::*;
use bevy_hourglass::*;
use bevy_hourglass::implementations::sprite::*;
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
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add a 2D camera
    commands.spawn(Camera2dBundle::default());
    
    // Add UI camera and text
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        // Add text for displaying hourglass info
        parent.spawn((
            TextBundle::from_section(
                "Hourglass: 60s remaining",
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            }),
            HourglassInfoText,
        ));
    });
    
    // Spawn a sprite-based hourglass
    commands.spawn((
        SpriteHourglassBundle::new(Duration::from_secs(60))
            .with_size(Vec2::new(100.0, 200.0))
            .with_container_color(Color::srgb(0.8, 0.8, 0.8))
            .with_sand_color(Color::srgb(0.9, 0.7, 0.2))
            .with_flip_duration(1.0)
            .with_update_during_flip(true),
        MainHourglass,
    ));
    
    // Add instructions
    commands.spawn(
        TextBundle::from_section(
            "Press SPACE to flip the hourglass",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        }),
    );
}

/// Handle keyboard input to flip the hourglass
fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut hourglasses: Query<&mut SpriteHourglass, With<MainHourglass>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(mut hourglass) = hourglasses.get_single_mut() {
            // Flip the hourglass when space is pressed
            hourglass.flip();
        }
    }
}

/// Update the UI text with hourglass information
fn update_ui(
    hourglasses: Query<&SpriteHourglass, With<MainHourglass>>,
    mut text_query: Query<&mut Text, With<HourglassInfoText>>,
) {
    if let (Ok(hourglass), Ok(mut text)) = (hourglasses.get_single(), text_query.get_single_mut()) {
        let remaining_secs = hourglass.get_time_remaining().as_secs();
        let status = if hourglass.is_flipping() {
            "Flipping"
        } else if hourglass.is_flipped() {
            "Flipped"
        } else {
            "Upright"
        };
        
        let running = if hourglass.is_running() { "Running" } else { "Stopped" };
        
        text.sections[0].value = format!(
            "Hourglass: {}s remaining | Status: {} | {}",
            remaining_secs, status, running
        );
    }
}
