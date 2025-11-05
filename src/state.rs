/// State machine system for character states
/// Each state has frame data and can transition to other states

use crate::hitbox::AttackData;

/// State ID for character states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StateId {
    Idle,
    Walk,
    Crouch,
    Jump,
    LightAttack,
    MediumAttack,
    HeavyAttack,
    SpecialMove,
    Hitstun,
    Blockstun,
    Knockdown,
    Custom(u16),
}

/// State type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateType {
    /// Normal state (can cancel to attacks)
    Normal,
    /// Attack state (has hitboxes)
    Attack,
    /// Hurt state (being hit)
    Hurt,
    /// Invincible state
    Invincible,
}

/// Frame-based action within a state
#[derive(Debug, Clone, Copy)]
pub enum StateAction {
    /// Create a hitbox
    Hitbox {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        attack: AttackData,
    },
    /// Set velocity
    SetVelocity {
        x: i32,
        y: i32,
    },
    /// Add momentum
    AddMomentum {
        x: i32,
        y: i32,
    },
    /// Transition to another state
    Transition {
        target: StateId,
    },
    /// No action
    None,
}

/// Frame data for a specific frame in a state
#[derive(Debug, Clone, Copy)]
pub struct FrameData {
    pub frame: u32,
    pub action: StateAction,
}

impl FrameData {
    pub const fn new(frame: u32, action: StateAction) -> Self {
        Self { frame, action }
    }
}

/// State definition with frame data
#[derive(Clone, Copy)]
pub struct State {
    pub id: StateId,
    pub state_type: StateType,
    pub duration: u32,          // Total frames
    pub can_cancel: bool,       // Can cancel to other states?
    pub frame_data: [Option<FrameData>; 32], // Frame-specific actions
    pub frame_data_count: usize,
}

impl State {
    pub fn new(id: StateId, state_type: StateType, duration: u32) -> Self {
        Self {
            id,
            state_type,
            duration,
            can_cancel: false,
            frame_data: [None; 32],
            frame_data_count: 0,
        }
    }

    pub fn with_cancel(mut self) -> Self {
        self.can_cancel = true;
        self
    }

    /// Add frame data to this state
    pub fn add_frame_data(mut self, data: FrameData) -> Self {
        if self.frame_data_count < 32 {
            self.frame_data[self.frame_data_count] = Some(data);
            self.frame_data_count += 1;
        }
        self
    }

    /// Get actions for a specific frame
    pub fn get_actions(&self, frame: u32) -> [Option<StateAction>; 8] {
        let mut actions = [None; 8];
        let mut action_count = 0;

        for i in 0..self.frame_data_count {
            if let Some(data) = &self.frame_data[i] {
                if data.frame == frame && action_count < 8 {
                    actions[action_count] = Some(data.action);
                    action_count += 1;
                }
            }
        }

        actions
    }
}

/// State machine that tracks current state and transitions
pub struct StateMachine {
    current_state: StateId,
    state_frame: u32,        // Current frame within the state
    states: [Option<State>; 32],
    state_count: usize,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            current_state: StateId::Idle,
            state_frame: 0,
            states: [None; 32],
            state_count: 0,
        }
    }

    /// Register a state
    pub fn register_state(&mut self, state: State) {
        if self.state_count < 32 {
            self.states[self.state_count] = Some(state);
            self.state_count += 1;
        }
    }

    /// Get current state
    pub fn current_state(&self) -> StateId {
        self.current_state
    }

    /// Get current frame within state
    pub fn state_frame(&self) -> u32 {
        self.state_frame
    }

    /// Transition to a new state
    pub fn transition(&mut self, new_state: StateId) {
        if new_state != self.current_state {
            self.current_state = new_state;
            self.state_frame = 0;
        }
    }

    /// Check if we can cancel current state
    pub fn can_cancel(&self) -> bool {
        self.find_state(self.current_state)
            .map(|s| s.can_cancel)
            .unwrap_or(false)
    }

    /// Advance to next frame
    pub fn advance_frame(&mut self) {
        self.state_frame += 1;

        // Auto-transition at end of state
        if let Some(state) = self.find_state(self.current_state) {
            if self.state_frame >= state.duration {
                // Default behavior: return to idle
                self.transition(StateId::Idle);
            }
        }
    }

    /// Get actions for current frame
    pub fn get_current_actions(&self) -> [Option<StateAction>; 8] {
        if let Some(state) = self.find_state(self.current_state) {
            state.get_actions(self.state_frame)
        } else {
            [None; 8]
        }
    }

    /// Find a state by ID
    fn find_state(&self, id: StateId) -> Option<&State> {
        for i in 0..self.state_count {
            if let Some(state) = &self.states[i] {
                if state.id == id {
                    return Some(state);
                }
            }
        }
        None
    }
}

/// Helper to build common states
pub mod states {
    use super::*;

    /// Create idle state
    pub fn idle() -> State {
        State::new(StateId::Idle, StateType::Normal, 1)
    }

    /// Create walk state
    pub fn walk() -> State {
        State::new(StateId::Walk, StateType::Normal, 1)
            .add_frame_data(FrameData::new(0, StateAction::SetVelocity { x: 300, y: 0 }))
    }

    /// Create basic light attack (fast, low damage)
    pub fn light_attack() -> State {
        State::new(StateId::LightAttack, StateType::Attack, 18)
            .with_cancel()
            .add_frame_data(FrameData::new(5, StateAction::Hitbox {
                x: 15000,
                y: 10000,
                width: 12000,
                height: 8000,
                attack: AttackData::new(50)
                    .with_stun(8, 6)
                    .with_knockback(400, 0),
            }))
    }

    /// Create medium attack (balanced)
    pub fn medium_attack() -> State {
        State::new(StateId::MediumAttack, StateType::Attack, 24)
            .with_cancel()
            .add_frame_data(FrameData::new(8, StateAction::Hitbox {
                x: 18000,
                y: 10000,
                width: 15000,
                height: 10000,
                attack: AttackData::new(100)
                    .with_stun(12, 8)
                    .with_knockback(800, 0),
            }))
    }

    /// Create heavy attack (slow, high damage)
    pub fn heavy_attack() -> State {
        State::new(StateId::HeavyAttack, StateType::Attack, 36)
            .add_frame_data(FrameData::new(12, StateAction::Hitbox {
                x: 20000,
                y: 10000,
                width: 18000,
                height: 12000,
                attack: AttackData::new(200)
                    .with_stun(18, 12)
                    .with_knockback(1500, -500), // Launcher
            }))
    }

    /// Create hitstun state
    pub fn hitstun(duration: u32) -> State {
        State::new(StateId::Hitstun, StateType::Hurt, duration)
    }

    /// Create blockstun state
    pub fn blockstun(duration: u32) -> State {
        State::new(StateId::Blockstun, StateType::Hurt, duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine_transition() {
        let mut sm = StateMachine::new();
        sm.register_state(states::idle());
        sm.register_state(states::light_attack());

        assert_eq!(sm.current_state(), StateId::Idle);

        sm.transition(StateId::LightAttack);
        assert_eq!(sm.current_state(), StateId::LightAttack);
        assert_eq!(sm.state_frame(), 0);
    }

    #[test]
    fn test_state_frame_advance() {
        let mut sm = StateMachine::new();
        sm.register_state(states::light_attack());
        sm.transition(StateId::LightAttack);

        for _ in 0..5 {
            sm.advance_frame();
        }
        assert_eq!(sm.state_frame(), 5);
    }

    #[test]
    fn test_state_actions() {
        let state = states::light_attack();
        let actions = state.get_actions(5);

        assert!(actions[0].is_some());
        if let Some(StateAction::Hitbox { attack, .. }) = actions[0] {
            assert_eq!(attack.damage, 50);
        } else {
            panic!("Expected hitbox action");
        }
    }
}
