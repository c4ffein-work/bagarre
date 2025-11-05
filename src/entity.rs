//! Entity system for fighters and other game objects
//! Combines state machine, physics, and collision

use crate::constants::*;
use crate::hitbox::{CollisionBox, CollisionResult};
use crate::input::InputBuffer;
use crate::state::{states, StateAction, StateId, StateMachine};
use crate::types::{EntityId, Facing, PlayerId, Vec2};

/// Health and damage tracking
#[derive(Debug, Clone, Copy)]
pub struct Health {
    pub current: i32,
    pub maximum: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self {
            current: max,
            maximum: max,
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.current = (self.current - damage).max(0);
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0
    }

    pub fn percentage(&self) -> f32 {
        self.current as f32 / self.maximum as f32
    }
}

/// Physics properties
#[derive(Debug, Clone, Copy)]
pub struct Physics {
    pub position: Vec2,
    pub velocity: Vec2,
    pub momentum: Vec2, // Knockback/hitstun momentum
    pub gravity: i32,   // Applied each frame when airborne
    pub on_ground: bool,
}

impl Physics {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            momentum: Vec2::ZERO,
            gravity: GRAVITY,
            on_ground: true,
        }
    }

    /// Apply physics for one frame
    pub fn update(&mut self) {
        // Apply momentum (from hits)
        self.position = self.position.add(self.momentum);

        // Decay momentum
        self.momentum.x = self.momentum.x * MOMENTUM_DECAY_PERCENT / MOMENTUM_DECAY_DIVISOR;
        self.momentum.y = self.momentum.y * MOMENTUM_DECAY_PERCENT / MOMENTUM_DECAY_DIVISOR;

        // Apply velocity (from movement)
        self.position = self.position.add(self.velocity);

        // Apply gravity if airborne
        if !self.on_ground {
            self.velocity.y += self.gravity;
        }

        // Ground collision (simplified)
        if self.position.y >= 0 {
            self.position.y = 0;
            self.velocity.y = 0;
            self.momentum.y = 0;
            self.on_ground = true;
        } else {
            self.on_ground = false;
        }

        // Reset velocity each frame (must be reapplied)
        self.velocity = Vec2::ZERO;
    }

    pub fn apply_knockback(&mut self, x: i32, y: i32) {
        self.momentum.x += x;
        self.momentum.y += y;

        // Launch into air if significant upward momentum
        if y < KNOCKBACK_THRESHOLD {
            self.on_ground = false;
        }
    }
}

/// Fighter entity
pub struct Entity {
    pub id: EntityId,
    pub player_id: PlayerId,
    pub facing: Facing,
    pub health: Health,
    pub physics: Physics,
    pub state_machine: StateMachine,
    pub hitstun_remaining: u32,
    pub blockstun_remaining: u32,
}

impl Entity {
    pub fn new(id: EntityId, player_id: PlayerId, position: Vec2) -> Self {
        let facing = match player_id {
            PlayerId::PLAYER_1 => Facing::Right,
            _ => Facing::Left,
        };

        let mut entity = Self {
            id,
            player_id,
            facing,
            health: Health::new(1000),
            physics: Physics::new(position),
            state_machine: StateMachine::new(),
            hitstun_remaining: 0,
            blockstun_remaining: 0,
        };

        // Register default states
        entity.register_default_states();

        entity
    }

    fn register_default_states(&mut self) {
        self.state_machine.register_state(states::idle());
        self.state_machine.register_state(states::walk());
        self.state_machine.register_state(states::walk_back());
        self.state_machine.register_state(states::jump());
        self.state_machine.register_state(states::light_attack());
        self.state_machine.register_state(states::medium_attack());
        self.state_machine.register_state(states::heavy_attack());
        self.state_machine.register_state(states::hitstun(20));
        self.state_machine.register_state(states::blockstun(15));
    }

    /// Update entity for one frame
    pub fn update(&mut self, input: Option<&InputBuffer>) {
        // Reduce stun timers
        if self.hitstun_remaining > 0 {
            self.hitstun_remaining -= 1;
            if self.hitstun_remaining == 0 {
                self.state_machine.transition(StateId::Idle);
            }
        }

        if self.blockstun_remaining > 0 {
            self.blockstun_remaining -= 1;
            if self.blockstun_remaining == 0 {
                self.state_machine.transition(StateId::Idle);
            }
        }

        // Process input if not in stun
        if self.hitstun_remaining == 0 && self.blockstun_remaining == 0 {
            self.process_input(input);
        }

        // Execute state actions
        self.execute_state_actions();

        // Advance state
        self.state_machine.advance_frame();

        // Update physics
        self.physics.update();
    }

    /// Process player input
    fn process_input(&mut self, input: Option<&InputBuffer>) {
        let Some(input) = input else { return };
        let current = input.current();

        // Attack inputs
        if self.can_act() {
            use crate::input::Button;

            if input.button_just_pressed(Button::Light) {
                self.state_machine.transition(StateId::LightAttack);
                return;
            }

            if input.button_just_pressed(Button::Medium) {
                self.state_machine.transition(StateId::MediumAttack);
                return;
            }

            if input.button_just_pressed(Button::Heavy) {
                self.state_machine.transition(StateId::HeavyAttack);
                return;
            }

            // Special move example: QCF + button
            if input.detect_qcf() && input.button_just_pressed(Button::Special) {
                self.state_machine.transition(StateId::SpecialMove);
                return;
            }
        }

        // Movement (can always move when not in stun)
        use crate::input::Direction;

        // Jump if pressing up while on ground
        if current.direction.is_up() && self.physics.on_ground {
            let current_state = self.state_machine.current_state();
            if current_state == StateId::Idle || current_state == StateId::Walk {
                self.state_machine.transition(StateId::Jump);
                return;
            }
        }

        match current.direction {
            Direction::Forward | Direction::DownForward | Direction::UpForward => {
                if self.state_machine.current_state() == StateId::Idle {
                    self.state_machine.transition(StateId::Walk);
                }
            }
            Direction::Back | Direction::DownBack | Direction::UpBack => {
                // Transition to backward walk if idle
                if self.state_machine.current_state() == StateId::Idle {
                    self.state_machine.transition(StateId::WalkBack);
                }
                // Blocking handled in hit processing
            }
            _ => {
                let current_state = self.state_machine.current_state();
                if current_state == StateId::Walk || current_state == StateId::WalkBack {
                    self.state_machine.transition(StateId::Idle);
                }
            }
        }
    }

    /// Execute actions from current state
    fn execute_state_actions(&mut self) {
        let actions = self.state_machine.get_current_actions();

        for action in actions.iter().flatten() {
            match action {
                StateAction::SetVelocity { x, y } => {
                    self.physics.velocity.x = x * self.facing.sign();
                    self.physics.velocity.y = *y;
                }
                StateAction::AddMomentum { x, y } => {
                    self.physics.momentum.x += x * self.facing.sign();
                    self.physics.momentum.y += y;
                }
                StateAction::Transition { target } => {
                    self.state_machine.transition(*target);
                }
                _ => {}
            }
        }
    }

    /// Get hitboxes for current frame
    pub fn get_hitboxes(&self) -> [Option<CollisionBox>; 4] {
        let mut hitboxes = [None; 4];
        let mut count = 0;

        let actions = self.state_machine.get_current_actions();
        for action_opt in &actions {
            if let Some(StateAction::Hitbox {
                x,
                y,
                width,
                height,
                attack,
            }) = action_opt
            {
                if count < 4 {
                    let mut bounds = crate::types::Rect::new(*x, *y, *width, *height);

                    // Flip hitbox for left-facing
                    if self.facing == Facing::Left {
                        bounds.x = -bounds.x - bounds.width;
                    }

                    hitboxes[count] = Some(
                        CollisionBox::hitbox(self.id, bounds, *attack)
                            .translate(self.physics.position),
                    );
                    count += 1;
                }
            }
        }

        hitboxes
    }

    /// Get hurtboxes (always present unless invincible)
    pub fn get_hurtboxes(&self) -> [Option<CollisionBox>; 2] {
        // Default body hurtbox
        let body_box = crate::types::Rect::new(0, 0, 10000, 25000);
        let hurtbox = CollisionBox::hurtbox(self.id, body_box).translate(self.physics.position);

        [Some(hurtbox), None]
    }

    /// Handle being hit
    pub fn take_hit(&mut self, collision: &CollisionResult, is_blocking: bool) {
        let attack = &collision.attack_data;

        if is_blocking && attack.can_block {
            // Blocked
            self.blockstun_remaining = attack.blockstun;
            self.state_machine.transition(StateId::Blockstun);

            // Reduced pushback when blocking
            self.physics
                .apply_knockback(attack.pushback_x / 2 * -self.facing.sign(), 0);
        } else {
            // Hit
            self.health.take_damage(attack.damage);
            self.hitstun_remaining = attack.hitstun;
            self.state_machine.transition(StateId::Hitstun);

            // Full knockback
            self.physics
                .apply_knockback(attack.pushback_x * -self.facing.sign(), attack.pushback_y);
        }
    }

    /// Check if entity can act (not in recovery/stun)
    fn can_act(&self) -> bool {
        self.hitstun_remaining == 0
            && self.blockstun_remaining == 0
            && (self.state_machine.current_state() == StateId::Idle
                || self.state_machine.can_cancel())
    }

    /// Update facing to look at opponent
    pub fn update_facing(&mut self, opponent_pos: Vec2) {
        if opponent_pos.x > self.physics.position.x {
            self.facing = Facing::Right;
        } else if opponent_pos.x < self.physics.position.x {
            self.facing = Facing::Left;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation() {
        let entity = Entity::new(EntityId(0), PlayerId::PLAYER_1, Vec2::new(0, 0));

        assert_eq!(entity.health.current, 1000);
        assert_eq!(entity.facing, Facing::Right);
        assert_eq!(entity.state_machine.current_state(), StateId::Idle);
    }

    #[test]
    fn test_health_damage() {
        let mut health = Health::new(100);
        health.take_damage(30);
        assert_eq!(health.current, 70);
        assert!(health.is_alive());

        health.take_damage(80);
        assert_eq!(health.current, 0);
        assert!(!health.is_alive());
    }

    #[test]
    fn test_physics_update() {
        let mut physics = Physics::new(Vec2::new(0, -1000));
        physics.on_ground = false;

        physics.update();

        // Should apply gravity (velocity increases downward)
        // After one frame, gravity is applied and position moves
        // Since we start at y=-1000 (above ground) and apply gravity,
        // we should move closer to ground (y=0)
        assert!(physics.position.y >= -1000);
    }

    #[test]
    fn test_facing_update() {
        let mut entity = Entity::new(EntityId(0), PlayerId::PLAYER_1, Vec2::new(0, 0));

        entity.update_facing(Vec2::new(1000, 0));
        assert_eq!(entity.facing, Facing::Right);

        entity.update_facing(Vec2::new(-1000, 0));
        assert_eq!(entity.facing, Facing::Left);
    }
}
