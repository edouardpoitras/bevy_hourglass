//! # Bevy Hourglass
//! 
//! A flexible hourglass plugin for Bevy applications.
//! 
//! This plugin allows you to spawn and interact with hourglasses in Bevy games/apps.
//! Hourglasses can be customized in terms of appearance, behavior, and interaction.

mod plugin;
mod traits;
mod components;
mod resources;
mod events;
mod systems;
mod bundles;

pub use plugin::HourglassPlugin;
pub use traits::*;
pub use components::*;
pub use resources::*;
pub use events::*;
pub use bundles::*;

// Re-export implementations
pub mod implementations;
