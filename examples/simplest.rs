use bevy::prelude::*;
use bevy_hourglass::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HourglassPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    spawn_sprite_hourglass(
        &mut commands,
        Duration::from_secs(10),
        Vec2::ZERO,
        Vec2::new(100.0, 200.0),
        Color::srgb(0.8, 0.8, 0.8),
        Color::srgb(0.9, 0.7, 0.2),
    );
}
