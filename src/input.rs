/// Input system with motion detection for fighting games
/// Supports directional inputs, buttons, and special move motions

use crate::constants::*;
use crate::types::Facing;

/// Button inputs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Light,   // Light attack
    Medium,  // Medium attack
    Heavy,   // Heavy attack
    Special, // Special button
}

/// Directional inputs using numpad notation
/// 7 8 9    (up-left, up, up-right)
/// 4 5 6    (left, neutral, right)
/// 1 2 3    (down-left, down, down-right)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Neutral = 5,
    Down = 2,
    DownBack = 1,
    Back = 4,
    UpBack = 7,
    Up = 8,
    UpForward = 9,
    Forward = 6,
    DownForward = 3,
}

impl Direction {
    /// Convert from directional bools
    pub fn from_directions(up: bool, down: bool, left: bool, right: bool, facing: Facing) -> Self {
        // Adjust based on facing (Back/Forward are relative)
        let (back, forward) = match facing {
            Facing::Right => (left, right),
            Facing::Left => (right, left),
        };

        match (up, down, back, forward) {
            (false, false, false, false) => Direction::Neutral,
            (false, true, false, false) => Direction::Down,
            (false, true, true, false) => Direction::DownBack,
            (false, true, false, true) => Direction::DownForward,
            (false, false, true, false) => Direction::Back,
            (false, false, false, true) => Direction::Forward,
            (true, false, false, false) => Direction::Up,
            (true, false, true, false) => Direction::UpBack,
            (true, false, false, true) => Direction::UpForward,
            _ => Direction::Neutral, // Invalid combinations default to neutral
        }
    }

    pub fn is_down(&self) -> bool {
        matches!(self, Direction::Down | Direction::DownBack | Direction::DownForward)
    }

    pub fn is_up(&self) -> bool {
        matches!(self, Direction::Up | Direction::UpBack | Direction::UpForward)
    }

    pub fn is_back(&self) -> bool {
        matches!(self, Direction::Back | Direction::DownBack | Direction::UpBack)
    }

    pub fn is_forward(&self) -> bool {
        matches!(self, Direction::Forward | Direction::DownForward | Direction::UpForward)
    }
}

/// Motion input patterns (special moves)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionInput {
    /// Quarter circle forward: 236 (down, down-forward, forward)
    QuarterCircleForward,
    /// Quarter circle back: 214 (down, down-back, back)
    QuarterCircleBack,
    /// Dragon punch: 623 (forward, down, down-forward)
    DragonPunch,
    /// Half circle forward: 41236
    HalfCircleForward,
    /// Half circle back: 63214
    HalfCircleBack,
    /// Charge back then forward: [4]6
    ChargeBackForward,
    /// Charge down then up: [2]8
    ChargeDownUp,
}

/// Input state for a single frame
#[derive(Debug, Clone, Copy)]
pub struct InputState {
    pub direction: Direction,
    pub light: bool,
    pub medium: bool,
    pub heavy: bool,
    pub special: bool,
}

impl InputState {
    pub const fn neutral() -> Self {
        Self {
            direction: Direction::Neutral,
            light: false,
            medium: false,
            heavy: false,
            special: false,
        }
    }

    pub fn button_pressed(&self, button: Button) -> bool {
        match button {
            Button::Light => self.light,
            Button::Medium => self.medium,
            Button::Heavy => self.heavy,
            Button::Special => self.special,
        }
    }
}

/// Input buffer for motion detection
/// Keeps last INPUT_BUFFER_SIZE frames (0.5 seconds at 60fps)
pub struct InputBuffer {
    buffer: [InputState; INPUT_BUFFER_SIZE],
    write_index: usize,
    facing: Facing,
}

impl InputBuffer {
    pub fn new(facing: Facing) -> Self {
        Self {
            buffer: [InputState::neutral(); INPUT_BUFFER_SIZE],
            write_index: 0,
            facing,
        }
    }

    /// Push new input state to buffer
    pub fn push(&mut self, input: InputState) {
        self.buffer[self.write_index] = input;
        self.write_index = (self.write_index + 1) % INPUT_BUFFER_SIZE;
    }

    /// Get most recent input
    pub fn current(&self) -> InputState {
        let prev_index = if self.write_index == 0 {
            INPUT_BUFFER_SIZE - 1
        } else {
            self.write_index - 1
        };
        self.buffer[prev_index]
    }

    /// Check if button was just pressed (not held)
    pub fn button_just_pressed(&self, button: Button) -> bool {
        let current = self.current();
        let prev_index = if self.write_index < 2 {
            INPUT_BUFFER_SIZE - 2 + self.write_index
        } else {
            self.write_index - 2
        };
        let previous = self.buffer[prev_index];

        current.button_pressed(button) && !previous.button_pressed(button)
    }

    /// Detect quarter circle forward motion (236)
    pub fn detect_qcf(&self) -> bool {
        self.detect_sequence(&[Direction::Down, Direction::DownForward, Direction::Forward])
    }

    /// Detect quarter circle back motion (214)
    pub fn detect_qcb(&self) -> bool {
        self.detect_sequence(&[Direction::Down, Direction::DownBack, Direction::Back])
    }

    /// Detect dragon punch motion (623)
    pub fn detect_dp(&self) -> bool {
        self.detect_sequence(&[Direction::Forward, Direction::Down, Direction::DownForward])
    }

    /// Check if a sequence of directions appears in recent inputs
    fn detect_sequence(&self, sequence: &[Direction]) -> bool {
        if sequence.is_empty() {
            return false;
        }

        // Check last MOTION_DETECTION_WINDOW frames (0.25 seconds at 60 FPS)
        for start_back in 0..MOTION_DETECTION_WINDOW {
            let mut matched = true;

            // Try to match the full sequence starting from this point
            for seq_offset in 0..sequence.len() {
                let buffer_idx = if self.write_index >= start_back + seq_offset + 1 {
                    self.write_index - start_back - seq_offset - 1
                } else {
                    INPUT_BUFFER_SIZE + self.write_index - start_back - seq_offset - 1
                };

                let dir = self.buffer[buffer_idx].direction;
                let expected = sequence[sequence.len() - 1 - seq_offset];

                if dir != expected {
                    matched = false;
                    break;
                }
            }

            if matched {
                return true;
            }
        }

        false
    }

    pub fn set_facing(&mut self, facing: Facing) {
        self.facing = facing;
    }
}

/// Input manager for multiple players
pub struct InputManager {
    pub player_inputs: [InputBuffer; MAX_PLAYERS],
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            player_inputs: [
                InputBuffer::new(Facing::Right),
                InputBuffer::new(Facing::Left),
            ],
        }
    }

    pub fn update_player_input(&mut self, player: usize, input: InputState) {
        if player < MAX_PLAYERS {
            self.player_inputs[player].push(input);
        }
    }

    pub fn get_player_input(&self, player: usize) -> Option<&InputBuffer> {
        if player < MAX_PLAYERS {
            Some(&self.player_inputs[player])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_detection() {
        let dir = Direction::from_directions(false, true, false, true, Facing::Right);
        assert_eq!(dir, Direction::DownForward);

        let dir = Direction::from_directions(false, true, true, false, Facing::Right);
        assert_eq!(dir, Direction::DownBack);
    }

    #[test]
    fn test_button_just_pressed() {
        let mut buffer = InputBuffer::new(Facing::Right);

        // First frame: no buttons
        buffer.push(InputState::neutral());

        // Second frame: light pressed
        let mut input = InputState::neutral();
        input.light = true;
        buffer.push(input);

        assert!(buffer.button_just_pressed(Button::Light));

        // Third frame: light still held
        buffer.push(input);
        assert!(!buffer.button_just_pressed(Button::Light)); // Not "just" pressed
    }

    #[test]
    fn test_qcf_detection() {
        let mut buffer = InputBuffer::new(Facing::Right);

        // Simulate quarter circle forward
        buffer.push(InputState { direction: Direction::Down, ..InputState::neutral() });
        buffer.push(InputState { direction: Direction::DownForward, ..InputState::neutral() });
        buffer.push(InputState { direction: Direction::Forward, ..InputState::neutral() });

        assert!(buffer.detect_qcf());
        assert!(!buffer.detect_qcb());
    }

    #[test]
    fn test_dp_detection() {
        let mut buffer = InputBuffer::new(Facing::Right);

        // Simulate dragon punch motion (forward, down, down-forward)
        buffer.push(InputState { direction: Direction::Forward, ..InputState::neutral() });
        buffer.push(InputState { direction: Direction::Down, ..InputState::neutral() });
        buffer.push(InputState { direction: Direction::DownForward, ..InputState::neutral() });

        assert!(buffer.detect_dp());
    }
}
