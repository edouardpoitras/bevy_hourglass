//! # Bevy Hourglass
//!
//! A simplified hourglass plugin for Bevy applications.
//!
//! This plugin allows you to spawn hourglasses in Bevy games/apps.
//! Hourglasses can be customized in terms of appearance, size, and behavior.

mod components;
mod events;
mod mesh_hourglass;
mod plugin;
mod resources;
mod sprite_hourglass;
mod systems;

pub use components::*;
pub use events::*;
pub use mesh_hourglass::*;
pub use plugin::HourglassPlugin;
pub use resources::*;
pub use sprite_hourglass::*;
