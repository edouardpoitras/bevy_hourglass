//! A WebAssembly-compatible example of the mesh-based hourglass with UI and user interaction.

use bevy::prelude::*;
use bevy_hourglass::{
    BulbStyle, Hourglass, HourglassMeshBodyConfig, HourglassMeshBuilder, HourglassMeshPlatesConfig,
    HourglassMeshSandConfig, HourglassPlugin, NeckStyle,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::console;

// This is the main entry point for all targets
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // For native builds, just run the app
        run();
    }

    // For WASM builds, main is not the entry point, but needs to exist for the example
    #[cfg(target_arch = "wasm32")]
    {
        // This won't actually run in WASM context
        println!("Note: When targeting WASM, this main function is not the entry point");
    }
}

// Entry point for wasm
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Redirect panic messages to the browser console
    console_error_panic_hook::set_once();

    // Start the Bevy app
    run();

    Ok(())
}

// Shared run function for both wasm and native
fn run() {
    App::new()
        .add_plugins((DefaultPlugins, HourglassPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, update_ui))
        .run();
}

/// Component to mark the UI text entity
#[derive(Component)]
struct HourglassInfoText;

/// Component to mark the hourglass entity
#[derive(Component)]
struct MainHourglass;

/// Set up the scene with camera, mesh hourglass, and UI
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Add a 2D camera
    commands.spawn(Camera2d::default());

    // Add UI node and text
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            ..default()
        },))
        .with_children(|parent| {
            // Add text for displaying hourglass info
            parent.spawn((
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                Text::new("Hourglass: 30s remaining"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor::from(Color::WHITE),
                TextLayout::default(),
                HourglassInfoText,
            ));
        });

    // Create a mesh-based hourglass using the builder pattern
    let hourglass_entity = HourglassMeshBuilder::new(Transform::from_xyz(0.0, 0.0, 0.0))
        .with_body(HourglassMeshBodyConfig {
            total_height: 200.0,
            bulb_style: BulbStyle::Circular {
                curvature: 1.0,
                width_factor: 0.75,
                curve_resolution: 20,
            },
            neck_style: NeckStyle::Curved {
                curvature: 0.2,
                width: 12.0,
                height: 8.0,
                curve_resolution: 5,
            },
            color: Color::srgba(0.85, 0.95, 1.0, 0.2),
        })
        .with_plates(HourglassMeshPlatesConfig {
            width: 165.0,
            height: 10.0,
            color: Color::srgb(0.6, 0.4, 0.2),
        })
        .with_sand(HourglassMeshSandConfig {
            color: Color::srgb(0.9, 0.8, 0.6),
            fill_percent: 1.0, // Start with full top bulb
            wall_offset: 8.0,  // 8 pixels from glass walls
        })
        .with_timing(30.0) // 30-second timer for automatic animation
        .build(&mut commands, &mut meshes, &mut materials);

    // Add the MainHourglass marker to track this specific hourglass
    commands.entity(hourglass_entity).insert(MainHourglass);

    // Add instructions - platform-aware message
    #[cfg(target_arch = "wasm32")]
    let instructions = "Tap/Click to flip the hourglass";

    #[cfg(not(target_arch = "wasm32"))]
    let instructions = "Press SPACE to flip the hourglass";

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        Text::new(instructions),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor::from(Color::WHITE),
        TextLayout::default(),
    ));

    // Log initialization message to console in WASM mode
    #[cfg(target_arch = "wasm32")]
    console::log_1(&"2D Mesh Hourglass WASM example initialized".into());
}

/// Handle input to flip the hourglass - works on both native and web
fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut hourglasses: Query<&mut Hourglass, With<MainHourglass>>,
) {
    // Check for space key press or mouse/touch input
    let should_flip =
        keyboard_input.just_pressed(KeyCode::Space) || mouse_button.just_pressed(MouseButton::Left);

    if should_flip {
        if let Ok(mut hourglass) = hourglasses.single_mut() {
            // Flip the hourglass
            hourglass.flip();

            // Log flip action in WASM mode
            #[cfg(target_arch = "wasm32")]
            console::log_1(&"Mesh hourglass flipped!".into());
        }
    }
}

/// Update the UI text with hourglass information
fn update_ui(
    hourglasses: Query<&Hourglass, With<MainHourglass>>,
    mut text_query: Query<&mut Text, With<HourglassInfoText>>,
) {
    if let (Ok(hourglass), Ok(mut text)) = (hourglasses.single(), text_query.single_mut()) {
        let remaining_secs = hourglass.remaining_time as u32;
        let status = if hourglass.flipping {
            "Flipping"
        } else if hourglass.flipped {
            "Flipped"
        } else {
            "Upright"
        };

        let running = if hourglass.running {
            "Running"
        } else {
            "Stopped"
        };

        text.0 = format!(
            "Hourglass: {}s remaining | Status: {} | {}",
            remaining_secs, status, running
        )
        .into();
    }
}
