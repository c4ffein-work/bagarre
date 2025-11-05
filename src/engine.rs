//! Main game engine - ties together all systems
//! Inspired by Castagne's phase-based execution model

use crate::constants::*;
use crate::entity::Entity;
use crate::hitbox::{CollisionResult, CollisionSystem};
use crate::input::{InputManager, InputState};
use crate::types::{EntityId, Frame, PlayerId, Vec2};

/// Game result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    InProgress,
    Player1Wins,
    Player2Wins,
    Draw,
}

/// Main game engine state
pub struct Engine {
    pub frame: Frame,
    pub entities: [Option<Entity>; MAX_ENTITIES],
    pub entity_count: usize,
    pub collision_system: CollisionSystem,
    pub input_manager: InputManager,
    pub game_result: GameResult,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            frame: Frame::ZERO,
            entities: [None, None, None, None],
            entity_count: 0,
            collision_system: CollisionSystem::new(),
            input_manager: InputManager::new(),
            game_result: GameResult::InProgress,
        }
    }

    /// Initialize a standard 2-player match
    pub fn init_match(&mut self) {
        // Player 1 on left
        let p1 = Entity::new(EntityId(0), PlayerId::PLAYER_1, Vec2::new(-50000, 0));

        // Player 2 on right
        let p2 = Entity::new(EntityId(1), PlayerId::PLAYER_2, Vec2::new(50000, 0));

        self.entities[0] = Some(p1);
        self.entities[1] = Some(p2);
        self.entity_count = 2;

        self.frame = Frame::ZERO;
        self.game_result = GameResult::InProgress;
    }

    /// Main game tick - advances one frame
    /// This follows a phase-based execution model like Castagne
    pub fn tick(&mut self, p1_input: InputState, p2_input: InputState) {
        if self.game_result != GameResult::InProgress {
            return; // Game over
        }

        // PHASE 1: INPUT
        self.input_manager.update_player_input(0, p1_input);
        self.input_manager.update_player_input(1, p2_input);

        // PHASE 2: UPDATE ENTITIES (Action phase)
        self.update_entities();

        // PHASE 3: COLLISION DETECTION (Physics phase)
        self.detect_collisions();

        // PHASE 4: RESOLVE HITS (Reaction phase)
        self.resolve_hits();

        // PHASE 5: CHECK WIN CONDITIONS
        self.check_win_conditions();

        // PHASE 6: UPDATE FACING
        self.update_facing();

        // Advance frame counter
        self.frame = self.frame.next();
    }

    /// Update all entities
    fn update_entities(&mut self) {
        for i in 0..self.entity_count {
            if let Some(entity) = &mut self.entities[i] {
                let player_id = entity.player_id.0 as usize;
                let input = self.input_manager.get_player_input(player_id);
                entity.update(input);
            }
        }
    }

    /// Detect all collisions this frame
    fn detect_collisions(&mut self) {
        self.collision_system.clear();

        // Gather all hitboxes and hurtboxes
        for i in 0..self.entity_count {
            if let Some(entity) = &self.entities[i] {
                // Add hitboxes
                let hitboxes = entity.get_hitboxes();
                for hitbox in hitboxes.iter().flatten() {
                    self.collision_system.add_hitbox(*hitbox);
                }

                // Add hurtboxes
                let hurtboxes = entity.get_hurtboxes();
                for hurtbox in hurtboxes.iter().flatten() {
                    self.collision_system.add_hurtbox(*hurtbox);
                }
            }
        }
    }

    /// Resolve all hit events
    fn resolve_hits(&mut self) {
        let collisions = self.collision_system.check_collisions();

        for collision in collisions.iter().flatten() {
            self.apply_hit(collision);
        }
    }

    /// Apply a single hit to defender
    fn apply_hit(&mut self, collision: &CollisionResult) {
        // Find defender
        let defender_idx = self.find_entity_index(collision.defender);
        let Some(defender_idx) = defender_idx else {
            return;
        };

        // Check if defender is blocking
        let is_blocking = {
            if let Some(defender) = &self.entities[defender_idx] {
                let player_id = defender.player_id.0 as usize;
                if let Some(input) = self.input_manager.get_player_input(player_id) {
                    let current = input.current();
                    // Blocking if holding back
                    current.direction.is_back()
                } else {
                    false
                }
            } else {
                false
            }
        };

        // Apply hit
        if let Some(defender) = &mut self.entities[defender_idx] {
            defender.take_hit(collision, is_blocking);
        }
    }

    /// Update all entities to face their opponents
    fn update_facing(&mut self) {
        if self.entity_count >= 2 {
            // Get positions first (avoid borrow checker issues)
            let p1_pos = self.entities[0].as_ref().map(|e| e.physics.position);
            let p2_pos = self.entities[1].as_ref().map(|e| e.physics.position);

            // Update p1 facing
            if let (Some(p1), Some(pos)) = (&mut self.entities[0], p2_pos) {
                p1.update_facing(pos);
            }

            // Update p2 facing
            if let (Some(p2), Some(pos)) = (&mut self.entities[1], p1_pos) {
                p2.update_facing(pos);
            }
        }
    }

    /// Check win conditions
    fn check_win_conditions(&mut self) {
        if self.entity_count < 2 {
            return;
        }

        let p1_alive = self.entities[0]
            .as_ref()
            .map(|e| e.health.is_alive())
            .unwrap_or(false);
        let p2_alive = self.entities[1]
            .as_ref()
            .map(|e| e.health.is_alive())
            .unwrap_or(false);

        self.game_result = match (p1_alive, p2_alive) {
            (true, true) => GameResult::InProgress,
            (true, false) => GameResult::Player1Wins,
            (false, true) => GameResult::Player2Wins,
            (false, false) => GameResult::Draw,
        };
    }

    /// Get entity by ID
    pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
        for i in 0..self.entity_count {
            if let Some(entity) = &self.entities[i] {
                if entity.id == id {
                    return Some(entity);
                }
            }
        }
        None
    }

    /// Get entity by player ID
    pub fn get_player_entity(&self, player: PlayerId) -> Option<&Entity> {
        for i in 0..self.entity_count {
            if let Some(entity) = &self.entities[i] {
                if entity.player_id == player {
                    return Some(entity);
                }
            }
        }
        None
    }

    fn find_entity_index(&self, id: EntityId) -> Option<usize> {
        for i in 0..self.entity_count {
            if let Some(entity) = &self.entities[i] {
                if entity.id == id {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Get game state summary for rendering/display
    pub fn get_state(&self) -> GameState<'_> {
        let p1 = self.get_player_entity(PlayerId::PLAYER_1);
        let p2 = self.get_player_entity(PlayerId::PLAYER_2);

        GameState {
            frame: self.frame.0,
            p1_pos: p1.map(|e| e.physics.position).unwrap_or(Vec2::ZERO),
            p1_health: p1.map(|e| e.health.current).unwrap_or(0),
            p1_state: p1
                .map(|e| state_to_string(e.state_machine.current_state()))
                .unwrap_or("Unknown"),
            p1_facing: p1.map(|e| e.facing).unwrap_or(crate::types::Facing::Right),
            p2_pos: p2.map(|e| e.physics.position).unwrap_or(Vec2::ZERO),
            p2_health: p2.map(|e| e.health.current).unwrap_or(0),
            p2_state: p2
                .map(|e| state_to_string(e.state_machine.current_state()))
                .unwrap_or("Unknown"),
            p2_facing: p2.map(|e| e.facing).unwrap_or(crate::types::Facing::Left),
            result: self.game_result,
        }
    }
}

/// Game state snapshot for display/serialization
#[derive(Debug, Clone)]
pub struct GameState<'a> {
    pub frame: u64,
    pub p1_pos: Vec2,
    pub p1_health: i32,
    pub p1_state: &'a str,
    pub p1_facing: crate::types::Facing,
    pub p2_pos: Vec2,
    pub p2_health: i32,
    pub p2_state: &'a str,
    pub p2_facing: crate::types::Facing,
    pub result: GameResult,
}

fn state_to_string(state: crate::state::StateId) -> &'static str {
    use crate::state::StateId;
    match state {
        StateId::Idle => "Idle",
        StateId::Walk => "Walk",
        StateId::WalkBack => "WalkBack",
        StateId::Crouch => "Crouch",
        StateId::Jump => "Jump",
        StateId::LightAttack => "Light",
        StateId::MediumAttack => "Medium",
        StateId::HeavyAttack => "Heavy",
        StateId::SpecialMove => "Special",
        StateId::Hitstun => "Hit",
        StateId::Blockstun => "Block",
        StateId::Knockdown => "Down",
        StateId::Custom(_) => "Custom",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_initialization() {
        let mut engine = Engine::new();
        engine.init_match();

        assert_eq!(engine.entity_count, 2);
        assert_eq!(engine.frame.0, 0);
        assert_eq!(engine.game_result, GameResult::InProgress);
    }

    #[test]
    fn test_engine_tick() {
        let mut engine = Engine::new();
        engine.init_match();

        let neutral = InputState::neutral();
        engine.tick(neutral, neutral);

        assert_eq!(engine.frame.0, 1);
    }

    #[test]
    fn test_win_condition() {
        let mut engine = Engine::new();
        engine.init_match();

        // Kill player 2
        if let Some(p2) = &mut engine.entities[1] {
            p2.health.current = 0;
        }

        engine.check_win_conditions();
        assert_eq!(engine.game_result, GameResult::Player1Wins);
    }
}
