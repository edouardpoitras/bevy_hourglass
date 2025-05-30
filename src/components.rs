use bevy::prelude::*;

/// Core component for an hourglass
#[derive(Component, Debug, Clone)]
pub struct Hourglass {
    // Timer properties
    /// Total time the hourglass can measure (in seconds)
    pub total_time: f32,
    /// Remaining time in the hourglass (in seconds)
    pub remaining_time: f32,
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
            total_time: 60.0,
            remaining_time: 60.0,
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

/// Configuration for sand splash animation
#[derive(Debug, Clone)]
pub struct SandSplashConfig {
    /// Radius around impact point where sand particles appear
    pub splash_radius: f32,
    /// Number of splash particles to spawn
    pub particle_count: u32,
    /// Duration each splash particle stays visible (in seconds)
    pub particle_duration: f32,
    /// How often to spawn new splash particles (in seconds)
    pub spawn_interval: f32,
    /// Color of splash particles
    pub particle_color: Color,
    /// Size of each splash particle
    pub particle_size: f32,
    /// Vertical offset of splash particles from the impact point
    pub vertical_offset: f32,
}

impl Default for SandSplashConfig {
    fn default() -> Self {
        Self {
            splash_radius: 15.0,
            particle_count: 8,
            particle_duration: 0.25,
            spawn_interval: 0.1,
            particle_color: Color::srgb(0.8, 0.6, 0.2),
            particle_size: 1.0,
            vertical_offset: 5.0, // Slightly above the bottom plate
        }
    }
}

/// Component that tracks sand splash state for an hourglass
#[derive(Component, Debug, Clone)]
pub struct SandSplash {
    pub config: SandSplashConfig,
    /// Timer for spawning new splash particles
    pub spawn_timer: f32,
    /// Track if sand was flowing in the previous frame (to detect start of impact)
    pub was_flowing: bool,
}

impl SandSplash {
    pub fn new(config: SandSplashConfig) -> Self {
        Self {
            config,
            spawn_timer: 0.0,
            was_flowing: false,
        }
    }
}

/// Marker component for sand splash particles
#[derive(Component, Debug)]
pub struct SandSplashParticle {
    /// Time remaining before particle disappears
    pub lifetime: f32,
}

impl Hourglass {
    /// Create a new hourglass with the specified total time in seconds
    pub fn new(total_time: f32) -> Self {
        // Calculate flow rate based on total time
        // Flow rate should be 1.0 / total_time_in_seconds
        // This ensures the hourglass empties exactly when the time is up
        let flow_rate = if total_time > 0.0 {
            1.0 / total_time
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
    pub fn update(&mut self, delta: f32) {
        // Handle flip animation first
        if self.flipping {
            self.flip_progress += delta / self.flip_duration;

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
            self.remaining_time = self.upper_chamber * self.total_time;

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
    fn update_sand(&mut self, delta: f32) {
        // Calculate the amount to transfer based on flow rate and delta time
        let transfer_amount = self.flow_rate * delta;

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
