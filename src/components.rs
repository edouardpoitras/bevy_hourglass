use bevy::prelude::*;
use std::time::Duration;

/// Core component for an hourglass
#[derive(Component, Debug, Clone)]
pub struct Hourglass {
    // Timer properties
    /// Total time the hourglass can measure
    pub total_time: Duration,
    /// Remaining time in the hourglass
    pub remaining_time: Duration,
    /// Whether the hourglass is currently running
    pub running: bool,

    // State properties
    /// Whether the hourglass is currently flipped (upside down)
    pub flipped: bool,
    /// Whether the hourglass is currently in the process of flipping
    pub flipping: bool,
    /// Duration of flip animation in seconds
    pub flip_duration: f32,
    /// Current flip progress (0.0 to 1.0)
    pub flip_progress: f32,
    /// Whether this hourglass should auto-flip when empty
    pub auto_flip_when_empty: bool,

    // Rotation properties
    /// Current rotation in radians
    pub current_rotation: f32,
    /// Target rotation for flip animation
    pub target_rotation: f32,

    // Visual properties
    /// Color of the container
    pub container_color: Color,
    /// Color of the sand
    pub sand_color: Color,
    /// Size of the hourglass
    pub size: Vec2,

    // Sand properties
    /// Fill percentage of the upper chamber (0.0 - 1.0)
    pub upper_chamber: f32,
    /// Fill percentage of the lower chamber (0.0 - 1.0)
    pub lower_chamber: f32,
    /// Flow rate in percentage per second
    pub flow_rate: f32,
}

impl Default for Hourglass {
    fn default() -> Self {
        Self {
            // Timer properties
            total_time: Duration::from_secs(60),
            remaining_time: Duration::from_secs(60),
            running: true,

            // State properties
            flipped: false,
            flipping: false,
            flip_duration: 1.0,
            flip_progress: 0.0,
            auto_flip_when_empty: false,

            // Rotation properties
            current_rotation: 0.0,
            target_rotation: 0.0,

            // Visual properties
            container_color: Color::srgb(0.8, 0.8, 0.8),
            sand_color: Color::srgb(0.8, 0.6, 0.2),
            size: Vec2::new(100.0, 200.0),

            // Sand properties
            upper_chamber: 1.0,
            lower_chamber: 0.0,
            flow_rate: 1.0 / 60.0,
        }
    }
}

impl Hourglass {
    /// Create a new hourglass with the specified total time
    pub fn new(total_time: Duration) -> Self {
        // Calculate flow rate based on total time
        // Flow rate should be 1.0 / total_time_in_seconds
        // This ensures the hourglass empties exactly when the time is up
        let flow_rate = if total_time.as_secs_f32() > 0.0 {
            1.0 / total_time.as_secs_f32()
        } else {
            1.0 / 60.0 // Default to 60s if total_time is zero
        };

        Self {
            total_time,
            remaining_time: total_time,
            flow_rate,
            ..Default::default()
        }
    }

    /// Update the hourglass state
    pub fn update(&mut self, delta: Duration) {
        // Handle flip animation first
        if self.flipping {
            self.flip_progress += delta.as_secs_f32() / self.flip_duration;

            if self.flip_progress >= 1.0 {
                // Flip animation complete
                self.flip_progress = 1.0;
                self.flipping = false;

                // Snap back to upright orientation
                self.current_rotation = 0.0;
                self.target_rotation = 0.0;

                // Invert the sand fill percentages (flip effect)
                std::mem::swap(&mut self.upper_chamber, &mut self.lower_chamber);

                // Invert the timer (if 2s left in a 10s timer, it should read 8s after flipping)
                self.remaining_time = self.total_time - self.remaining_time;

                // Always ensure the timer is running if there's sand in the upper chamber
                if !self.running && self.upper_chamber > 0.0 {
                    self.running = true;
                }
            } else {
                // Interpolate rotation during flip (always from 0 to PI)
                self.current_rotation = self.flip_progress * std::f32::consts::PI;
            }
        }

        // Only update sand levels and time if not flipping
        if self.running && !self.flipping {
            // Update sand flow
            self.update_sand(delta);

            // Update remaining time based on sand in the upper chamber
            self.remaining_time =
                Duration::from_secs_f32(self.upper_chamber * self.total_time.as_secs_f32());

            // Check if the hourglass is empty (no sand in the upper chamber)
            if self.upper_chamber <= 0.0 {
                self.running = false;

                // Auto-flip if enabled
                if self.auto_flip_when_empty {
                    self.flip();
                }
            }
        }
    }

    /// Update the sand levels
    fn update_sand(&mut self, delta: Duration) {
        // Calculate the amount to transfer based on flow rate and delta time
        let transfer_amount = self.flow_rate * delta.as_secs_f32();

        // Sand always flows from upper to lower (gravity)
        let transfer = transfer_amount.min(self.upper_chamber);
        self.upper_chamber -= transfer;
        self.lower_chamber += transfer;

        // Ensure values stay in valid range
        self.upper_chamber = self.upper_chamber.clamp(0.0, 1.0);
        self.lower_chamber = self.lower_chamber.clamp(0.0, 1.0);
    }

    /// Start flipping the hourglass
    pub fn flip(&mut self) {
        if !self.flipping {
            self.flipping = true;
            self.flip_progress = 0.0;
            // Always flip 180 degrees (PI radians) from current upright position
            self.target_rotation = std::f32::consts::PI;
        }
    }

    /// Check if the hourglass is ready to be flipped (not currently flipping)
    pub fn can_flip(&self) -> bool {
        !self.flipping
    }
}
