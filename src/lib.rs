//! # Bevy Hourglass
//! 
//! A simplified hourglass plugin for Bevy applications.
//! 
//! This plugin allows you to spawn hourglasses in Bevy games/apps.
//! Hourglasses can be customized in terms of appearance, size, and behavior.

mod plugin;
mod components;
mod resources;
mod events;
mod systems;
mod sprite_hourglass;

pub use plugin::HourglassPlugin;
pub use components::*;
pub use resources::*;
pub use events::*;
pub use sprite_hourglass::*;
