//! # Bevy Hourglass
//! 
//! A simplified hourglass plugin for Bevy applications.
//! 
//! This plugin allows you to spawn and interact with hourglasses in Bevy games/apps.
//! Hourglasses can be customized in terms of appearance and interaction.

mod plugin;
mod components;
mod resources;
mod events;
mod systems;
mod bundles;
mod sprite_hourglass;

pub use plugin::HourglassPlugin;
pub use components::*;
pub use resources::*;
pub use events::*;
pub use bundles::*;
pub use sprite_hourglass::*;
