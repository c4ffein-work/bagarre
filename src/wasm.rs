//! WASM interface for browser integration
//!
//! This module provides a simple interface for running the engine in the browser.
//! For a truly zero-dependency build, compile with target-feature flags.
//!
//! To use with wasm-bindgen (recommended), enable it in Cargo.toml

use crate::engine::{Engine, GameResult};
use crate::input::{InputState, Direction, Button};
use crate::types::{PlayerId, Facing};

/// Global engine instance for WASM
static mut ENGINE: Option<Engine> = None;

/// Initialize the engine
#[no_mangle]
pub extern "C" fn init() {
    unsafe {
        let mut engine = Engine::new();
        engine.init_match();
        ENGINE = Some(engine);
    }
}

/// Update the game by one frame
/// Inputs are encoded as bit flags:
/// - Bits 0-3: Direction (0-9 numpad notation)
/// - Bit 4: Light button
/// - Bit 5: Medium button
/// - Bit 6: Heavy button
/// - Bit 7: Special button
#[no_mangle]
pub extern "C" fn tick(p1_input: u32, p2_input: u32) {
    unsafe {
        if let Some(engine) = &mut ENGINE {
            let p1 = decode_input(p1_input, Facing::Right);
            let p2 = decode_input(p2_input, Facing::Left);
            engine.tick(p1, p2);
        }
    }
}

/// Get current frame number
#[no_mangle]
pub extern "C" fn get_frame() -> u64 {
    unsafe {
        ENGINE.as_ref().map(|e| e.frame.0).unwrap_or(0)
    }
}

/// Get player 1 position X
#[no_mangle]
pub extern "C" fn get_p1_x() -> i32 {
    unsafe {
        ENGINE.as_ref()
            .and_then(|e| e.get_player_entity(PlayerId::PLAYER_1))
            .map(|p| p.physics.position.x)
            .unwrap_or(0)
    }
}

/// Get player 1 position Y
#[no_mangle]
pub extern "C" fn get_p1_y() -> i32 {
    unsafe {
        ENGINE.as_ref()
            .and_then(|e| e.get_player_entity(PlayerId::PLAYER_1))
            .map(|p| p.physics.position.y)
            .unwrap_or(0)
    }
}

/// Get player 1 health
#[no_mangle]
pub extern "C" fn get_p1_health() -> i32 {
    unsafe {
        ENGINE.as_ref()
            .and_then(|e| e.get_player_entity(PlayerId::PLAYER_1))
            .map(|p| p.health.current)
            .unwrap_or(0)
    }
}

/// Get player 1 state (encoded as integer)
#[no_mangle]
pub extern "C" fn get_p1_state() -> u32 {
    unsafe {
        ENGINE.as_ref()
            .and_then(|e| e.get_player_entity(PlayerId::PLAYER_1))
            .map(|p| encode_state(p.state_machine.current_state()))
            .unwrap_or(0)
    }
}

/// Get player 1 facing (1 = right, -1 = left)
#[no_mangle]
pub extern "C" fn get_p1_facing() -> i32 {
    unsafe {
        ENGINE.as_ref()
            .and_then(|e| e.get_player_entity(PlayerId::PLAYER_1))
            .map(|p| p.facing.sign())
            .unwrap_or(1)
    }
}

/// Get player 2 position X
#[no_mangle]
pub extern "C" fn get_p2_x() -> i32 {
    unsafe {
        ENGINE.as_ref()
            .and_then(|e| e.get_player_entity(PlayerId::PLAYER_2))
            .map(|p| p.physics.position.x)
            .unwrap_or(0)
    }
}

/// Get player 2 position Y
#[no_mangle]
pub extern "C" fn get_p2_y() -> i32 {
    unsafe {
        ENGINE.as_ref()
            .and_then(|e| e.get_player_entity(PlayerId::PLAYER_2))
            .map(|p| p.physics.position.y)
            .unwrap_or(0)
    }
}

/// Get player 2 health
#[no_mangle]
pub extern "C" fn get_p2_health() -> i32 {
    unsafe {
        ENGINE.as_ref()
            .and_then(|e| e.get_player_entity(PlayerId::PLAYER_2))
            .map(|p| p.health.current)
            .unwrap_or(0)
    }
}

/// Get player 2 state (encoded as integer)
#[no_mangle]
pub extern "C" fn get_p2_state() -> u32 {
    unsafe {
        ENGINE.as_ref()
            .and_then(|e| e.get_player_entity(PlayerId::PLAYER_2))
            .map(|p| encode_state(p.state_machine.current_state()))
            .unwrap_or(0)
    }
}

/// Get player 2 facing (1 = right, -1 = left)
#[no_mangle]
pub extern "C" fn get_p2_facing() -> i32 {
    unsafe {
        ENGINE.as_ref()
            .and_then(|e| e.get_player_entity(PlayerId::PLAYER_2))
            .map(|p| p.facing.sign())
            .unwrap_or(-1)
    }
}

/// Get game result (0 = in progress, 1 = P1 wins, 2 = P2 wins, 3 = draw)
#[no_mangle]
pub extern "C" fn get_result() -> u32 {
    unsafe {
        ENGINE.as_ref()
            .map(|e| match e.game_result {
                GameResult::InProgress => 0,
                GameResult::Player1Wins => 1,
                GameResult::Player2Wins => 2,
                GameResult::Draw => 3,
            })
            .unwrap_or(0)
    }
}

/// Decode input from bitfield
fn decode_input(input: u32, facing: Facing) -> InputState {
    let dir_value = (input & 0xF) as u8;
    let direction = match dir_value {
        5 | 0 => Direction::Neutral,
        2 => Direction::Down,
        1 => Direction::DownBack,
        4 => Direction::Back,
        7 => Direction::UpBack,
        8 => Direction::Up,
        9 => Direction::UpForward,
        6 => Direction::Forward,
        3 => Direction::DownForward,
        _ => Direction::Neutral,
    };

    InputState {
        direction,
        light: (input & 0x10) != 0,
        medium: (input & 0x20) != 0,
        heavy: (input & 0x40) != 0,
        special: (input & 0x80) != 0,
    }
}

/// Encode state to integer
fn encode_state(state: crate::state::StateId) -> u32 {
    use crate::state::StateId;
    match state {
        StateId::Idle => 0,
        StateId::Walk => 1,
        StateId::Crouch => 2,
        StateId::Jump => 3,
        StateId::LightAttack => 4,
        StateId::MediumAttack => 5,
        StateId::HeavyAttack => 6,
        StateId::SpecialMove => 7,
        StateId::Hitstun => 8,
        StateId::Blockstun => 9,
        StateId::Knockdown => 10,
        StateId::Custom(id) => 100 + id as u32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_encoding() {
        // Test neutral input
        let input = 0x00; // All bits off
        let decoded = decode_input(input, Facing::Right);
        assert_eq!(decoded.direction, Direction::Neutral);
        assert!(!decoded.light);

        // Test light button
        let input = 0x10; // Light button bit
        let decoded = decode_input(input, Facing::Right);
        assert!(decoded.light);

        // Test forward + light
        let input = 0x16; // Direction 6 (forward) + light
        let decoded = decode_input(input, Facing::Right);
        assert_eq!(decoded.direction, Direction::Forward);
        assert!(decoded.light);
    }

    #[test]
    fn test_state_encoding() {
        use crate::state::StateId;
        assert_eq!(encode_state(StateId::Idle), 0);
        assert_eq!(encode_state(StateId::LightAttack), 4);
        assert_eq!(encode_state(StateId::Custom(5)), 105);
    }
}
