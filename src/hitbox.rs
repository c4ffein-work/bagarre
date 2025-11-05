/// Hitbox and hurtbox system for collision detection
/// Inspired by Castagne's attack/defense collision model

use crate::constants::*;
use crate::types::{Rect, Vec2, EntityId};

/// Type of collision box
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxType {
    /// Attack box - can hit hurtboxes
    Hitbox,
    /// Defense box - can be hit by hitboxes
    Hurtbox,
    /// Collision box - for pushbox/walls
    Pushbox,
}

/// Attack properties for hitboxes
#[derive(Debug, Clone, Copy)]
pub struct AttackData {
    pub damage: i32,
    pub hitstun: u32,       // Frames of hitstun on hit
    pub blockstun: u32,     // Frames of blockstun if blocked
    pub pushback_x: i32,    // Horizontal knockback
    pub pushback_y: i32,    // Vertical knockback (for launchers)
    pub can_block: bool,    // Is this blockable?
    pub is_overhead: bool,  // Must block standing
    pub is_low: bool,       // Must block crouching
}

impl AttackData {
    pub fn new(damage: i32) -> Self {
        Self {
            damage,
            hitstun: 12,
            blockstun: 8,
            pushback_x: 500,
            pushback_y: 0,
            can_block: true,
            is_overhead: false,
            is_low: false,
        }
    }

    pub fn with_knockback(mut self, x: i32, y: i32) -> Self {
        self.pushback_x = x;
        self.pushback_y = y;
        self
    }

    pub fn with_stun(mut self, hitstun: u32, blockstun: u32) -> Self {
        self.hitstun = hitstun;
        self.blockstun = blockstun;
        self
    }

    pub fn unblockable(mut self) -> Self {
        self.can_block = false;
        self
    }

    pub fn overhead(mut self) -> Self {
        self.is_overhead = true;
        self
    }

    pub fn low(mut self) -> Self {
        self.is_low = true;
        self
    }
}

/// A collision box with properties
#[derive(Debug, Clone, Copy)]
pub struct CollisionBox {
    pub box_type: BoxType,
    pub bounds: Rect,
    pub owner: EntityId,
    pub active: bool,
    pub attack_data: Option<AttackData>,
}

impl CollisionBox {
    pub fn hitbox(owner: EntityId, bounds: Rect, attack_data: AttackData) -> Self {
        Self {
            box_type: BoxType::Hitbox,
            bounds,
            owner,
            active: true,
            attack_data: Some(attack_data),
        }
    }

    pub fn hurtbox(owner: EntityId, bounds: Rect) -> Self {
        Self {
            box_type: BoxType::Hurtbox,
            bounds,
            owner,
            active: true,
            attack_data: None,
        }
    }

    pub fn pushbox(owner: EntityId, bounds: Rect) -> Self {
        Self {
            box_type: BoxType::Pushbox,
            bounds,
            owner,
            active: true,
            attack_data: None,
        }
    }

    /// Translate box by offset (for entity positioning)
    pub fn translate(&self, offset: Vec2) -> CollisionBox {
        let mut new_box = *self;
        new_box.bounds.x += offset.x;
        new_box.bounds.y += offset.y;
        new_box
    }
}

/// Result of a collision check
#[derive(Debug, Clone, Copy)]
pub struct CollisionResult {
    pub attacker: EntityId,
    pub defender: EntityId,
    pub attack_data: AttackData,
}

/// Collision detection system
pub struct CollisionSystem {
    hitboxes: [Option<CollisionBox>; MAX_HITBOXES],
    hurtboxes: [Option<CollisionBox>; MAX_HURTBOXES],
    hit_count: usize,
    hurt_count: usize,
}

impl CollisionSystem {
    pub fn new() -> Self {
        Self {
            hitboxes: [None; MAX_HITBOXES],
            hurtboxes: [None; MAX_HURTBOXES],
            hit_count: 0,
            hurt_count: 0,
        }
    }

    pub fn clear(&mut self) {
        self.hit_count = 0;
        self.hurt_count = 0;
        for i in 0..MAX_HITBOXES {
            self.hitboxes[i] = None;
        }
        for i in 0..MAX_HURTBOXES {
            self.hurtboxes[i] = None;
        }
    }

    pub fn add_hitbox(&mut self, hitbox: CollisionBox) {
        if self.hit_count < MAX_HITBOXES {
            self.hitboxes[self.hit_count] = Some(hitbox);
            self.hit_count += 1;
        }
    }

    pub fn add_hurtbox(&mut self, hurtbox: CollisionBox) {
        if self.hurt_count < MAX_HURTBOXES {
            self.hurtboxes[self.hurt_count] = Some(hurtbox);
            self.hurt_count += 1;
        }
    }

    /// Check all hitbox vs hurtbox collisions
    /// Returns list of collision results
    pub fn check_collisions(&self) -> [Option<CollisionResult>; MAX_COLLISIONS_PER_FRAME] {
        let mut results = [None; MAX_COLLISIONS_PER_FRAME];
        let mut result_count = 0;

        for i in 0..self.hit_count {
            if let Some(hitbox) = &self.hitboxes[i] {
                if !hitbox.active {
                    continue;
                }

                for j in 0..self.hurt_count {
                    if let Some(hurtbox) = &self.hurtboxes[j] {
                        if !hurtbox.active {
                            continue;
                        }

                        // Don't hit yourself
                        if hitbox.owner == hurtbox.owner {
                            continue;
                        }

                        // Check collision
                        if hitbox.bounds.intersects(&hurtbox.bounds) {
                            if let Some(attack_data) = hitbox.attack_data {
                                if result_count < MAX_COLLISIONS_PER_FRAME {
                                    results[result_count] = Some(CollisionResult {
                                        attacker: hitbox.owner,
                                        defender: hurtbox.owner,
                                        attack_data,
                                    });
                                    result_count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_data_builder() {
        let attack = AttackData::new(100)
            .with_knockback(1000, 500)
            .unblockable();

        assert_eq!(attack.damage, 100);
        assert_eq!(attack.pushback_x, 1000);
        assert_eq!(attack.pushback_y, 500);
        assert!(!attack.can_block);
    }

    #[test]
    fn test_collision_detection() {
        let mut system = CollisionSystem::new();

        let attacker_id = EntityId(0);
        let defender_id = EntityId(1);

        // Create overlapping boxes
        let hitbox = CollisionBox::hitbox(
            attacker_id,
            Rect::new(10, 10, 20, 20),
            AttackData::new(100),
        );

        let hurtbox = CollisionBox::hurtbox(
            defender_id,
            Rect::new(15, 15, 20, 20),
        );

        system.add_hitbox(hitbox);
        system.add_hurtbox(hurtbox);

        let results = system.check_collisions();

        assert!(results[0].is_some());
        let collision = results[0].as_ref().unwrap();
        assert_eq!(collision.attacker, attacker_id);
        assert_eq!(collision.defender, defender_id);
        assert_eq!(collision.attack_data.damage, 100);
    }

    #[test]
    fn test_no_self_collision() {
        let mut system = CollisionSystem::new();
        let entity_id = EntityId(0);

        let hitbox = CollisionBox::hitbox(
            entity_id,
            Rect::new(10, 10, 20, 20),
            AttackData::new(100),
        );

        let hurtbox = CollisionBox::hurtbox(
            entity_id,
            Rect::new(15, 15, 20, 20),
        );

        system.add_hitbox(hitbox);
        system.add_hurtbox(hurtbox);

        let results = system.check_collisions();
        assert!(results[0].is_none()); // No self-collision
    }
}
