//! Configuration system for customizing game physics and parameters
//!
//! This module provides runtime configuration for various engine parameters.
//! By default, all values are set to match the constants, but can be customized
//! per-game or per-character.

use crate::constants::*;

/// Physics configuration for entity movement and knockback
#[derive(Debug, Clone, Copy)]
pub struct PhysicsConfig {
    /// Gravity acceleration applied each frame
    pub gravity: i32,
    /// Ground level Y coordinate
    pub ground_level: i32,
    /// Momentum decay percentage (0-100)
    pub momentum_decay_percent: i32,
    /// Knockback threshold for launching into air
    pub knockback_threshold: i32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: GRAVITY,
            ground_level: GROUND_LEVEL,
            momentum_decay_percent: MOMENTUM_DECAY_PERCENT,
            knockback_threshold: KNOCKBACK_THRESHOLD,
        }
    }
}

impl PhysicsConfig {
    /// Creates a new physics config with custom values
    pub fn new(gravity: i32, ground_level: i32, momentum_decay_percent: i32) -> Self {
        Self {
            gravity,
            ground_level,
            momentum_decay_percent,
            knockback_threshold: KNOCKBACK_THRESHOLD,
        }
    }

    /// Creates a config with high gravity (fast falling)
    pub fn high_gravity() -> Self {
        Self {
            gravity: GRAVITY * 2,
            ..Default::default()
        }
    }

    /// Creates a config with low gravity (floaty)
    pub fn low_gravity() -> Self {
        Self {
            gravity: GRAVITY / 2,
            ..Default::default()
        }
    }

    /// Creates a config with fast momentum decay (less slidey)
    pub fn fast_decay() -> Self {
        Self {
            momentum_decay_percent: 70,
            ..Default::default()
        }
    }

    /// Creates a config with slow momentum decay (more slidey)
    pub fn slow_decay() -> Self {
        Self {
            momentum_decay_percent: 95,
            ..Default::default()
        }
    }
}

/// Input configuration for motion detection and buffering
#[derive(Debug, Clone, Copy)]
pub struct InputConfig {
    /// Size of the input buffer in frames
    pub buffer_size: usize,
    /// Motion detection window in frames
    pub detection_window: usize,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            buffer_size: INPUT_BUFFER_SIZE,
            detection_window: MOTION_DETECTION_WINDOW,
        }
    }
}

impl InputConfig {
    /// Creates a new input config with custom values
    pub fn new(buffer_size: usize, detection_window: usize) -> Self {
        Self {
            buffer_size,
            detection_window,
        }
    }

    /// Creates a config with lenient motion detection (larger window)
    pub fn lenient() -> Self {
        Self {
            detection_window: 20,
            ..Default::default()
        }
    }

    /// Creates a config with strict motion detection (smaller window)
    pub fn strict() -> Self {
        Self {
            detection_window: 10,
            ..Default::default()
        }
    }
}

/// Game rule configuration
#[derive(Debug, Clone, Copy)]
pub struct GameConfig {
    /// Starting health for each player
    pub starting_health: i32,
    /// Round time limit in frames (0 = no limit)
    pub time_limit_frames: u64,
    /// Number of rounds to win
    pub rounds_to_win: u32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            starting_health: 1000,
            time_limit_frames: 3600, // 60 seconds at 60 FPS
            rounds_to_win: 2,
        }
    }
}

impl GameConfig {
    /// Creates a new game config with custom values
    pub fn new(starting_health: i32, time_limit_frames: u64, rounds_to_win: u32) -> Self {
        Self {
            starting_health,
            time_limit_frames,
            rounds_to_win,
        }
    }

    /// Creates a config for quick matches
    pub fn quick_match() -> Self {
        Self {
            starting_health: 500,
            time_limit_frames: 1800, // 30 seconds
            rounds_to_win: 1,
        }
    }

    /// Creates a config for extended matches
    pub fn extended_match() -> Self {
        Self {
            starting_health: 2000,
            time_limit_frames: 7200, // 120 seconds
            rounds_to_win: 3,
        }
    }

    /// Creates a config with no time limit
    pub fn no_time_limit() -> Self {
        Self {
            time_limit_frames: 0,
            ..Default::default()
        }
    }
}

/// Complete engine configuration
#[derive(Debug, Clone, Copy, Default)]
pub struct EngineConfig {
    /// Physics parameters
    pub physics: PhysicsConfig,
    /// Input parameters
    pub input: InputConfig,
    /// Game rules
    pub game: GameConfig,
}

impl EngineConfig {
    /// Creates a new engine config with all custom values
    pub fn new(physics: PhysicsConfig, input: InputConfig, game: GameConfig) -> Self {
        Self {
            physics,
            input,
            game,
        }
    }

    /// Creates a config for casual play (lenient inputs, lower health)
    pub fn casual() -> Self {
        Self {
            input: InputConfig::lenient(),
            game: GameConfig::quick_match(),
            ..Default::default()
        }
    }

    /// Creates a config for competitive play (strict inputs, standard rules)
    pub fn competitive() -> Self {
        Self {
            input: InputConfig::strict(),
            game: GameConfig::default(),
            ..Default::default()
        }
    }

    /// Creates a config for training mode (no time limit, high health)
    pub fn training() -> Self {
        Self {
            game: GameConfig {
                starting_health: 10000,
                time_limit_frames: 0,
                rounds_to_win: 1,
            },
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_configs() {
        let physics = PhysicsConfig::default();
        assert_eq!(physics.gravity, GRAVITY);

        let input = InputConfig::default();
        assert_eq!(input.buffer_size, INPUT_BUFFER_SIZE);

        let game = GameConfig::default();
        assert_eq!(game.starting_health, 1000);
    }

    #[test]
    fn test_preset_configs() {
        let casual = EngineConfig::casual();
        assert_eq!(casual.input.detection_window, 20);

        let competitive = EngineConfig::competitive();
        assert_eq!(competitive.input.detection_window, 10);

        let training = EngineConfig::training();
        assert_eq!(training.game.time_limit_frames, 0);
    }

    #[test]
    fn test_physics_presets() {
        let high_g = PhysicsConfig::high_gravity();
        assert_eq!(high_g.gravity, GRAVITY * 2);

        let low_g = PhysicsConfig::low_gravity();
        assert_eq!(low_g.gravity, GRAVITY / 2);
    }
}
