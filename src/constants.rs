/// Constants for the Bagarre fighting game engine
///
/// This module contains all configuration constants used throughout the engine.
/// Modifying these values allows tuning of game physics, timing, and limits.

// =============================================================================
// Physics Constants
// =============================================================================

/// Gravity acceleration applied to entities each frame (internal units per frame)
/// Default: 80 units per frame
pub const GRAVITY: i32 = 80;

/// Ground level Y coordinate (internal units)
/// Entities below this level are considered grounded
pub const GROUND_LEVEL: i32 = 18000;

/// Momentum decay factor (percentage)
/// Each frame, momentum is multiplied by this percentage
/// Default: 90% (momentum decays by 10% per frame)
pub const MOMENTUM_DECAY_PERCENT: i32 = 90;

/// Momentum decay divisor
pub const MOMENTUM_DECAY_DIVISOR: i32 = 100;

/// Minimum knockback threshold (internal units)
/// Knockback velocities below this value are considered zero
pub const KNOCKBACK_THRESHOLD: i32 = -100;

// =============================================================================
// Input System Constants
// =============================================================================

/// Size of the input buffer in frames
/// Default: 30 frames (0.5 seconds at 60 FPS)
pub const INPUT_BUFFER_SIZE: usize = 30;

/// Motion detection window in frames
/// Default: 15 frames (0.25 seconds at 60 FPS)
pub const MOTION_DETECTION_WINDOW: usize = 15;

// =============================================================================
// State Machine Limits
// =============================================================================

/// Maximum number of states that can be registered in the state machine
pub const MAX_STATES: usize = 32;

/// Maximum number of frame data entries per state
pub const MAX_FRAME_DATA_PER_STATE: usize = 32;

/// Maximum number of actions that can execute in a single frame
pub const MAX_ACTIONS_PER_FRAME: usize = 8;

// =============================================================================
// Collision System Limits
// =============================================================================

/// Maximum number of hitboxes per entity
pub const MAX_HITBOXES: usize = 32;

/// Maximum number of hurtboxes per entity
pub const MAX_HURTBOXES: usize = 32;

/// Maximum number of collision results per frame
pub const MAX_COLLISIONS_PER_FRAME: usize = 16;

// =============================================================================
// Engine Limits
// =============================================================================

/// Maximum number of entities in the game
/// Default: 4 (2 fighters + 2 projectiles)
pub const MAX_ENTITIES: usize = 4;

/// Number of players in the game
pub const MAX_PLAYERS: usize = 2;

// =============================================================================
// Conversion Constants
// =============================================================================

/// Internal units to display units conversion factor
/// Divide internal units by this value to get display units
pub const INTERNAL_TO_DISPLAY: i32 = 1000;
