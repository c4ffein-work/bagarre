/// Core types for the Bagarre fighting game engine
/// Zero dependencies - all implementations are custom

/// 2D Vector for positions, velocities, etc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0, y: 0 };

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn scale(&self, scalar: i32) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn dot(&self, other: Vec2) -> i32 {
        self.x * other.x + self.y * other.y
    }

    pub fn length_squared(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}

/// Rectangle for hitboxes, hurtboxes, etc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: i32,      // Left edge
    pub y: i32,      // Top edge
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { x, y, width, height }
    }

    pub fn from_center(center: Vec2, width: i32, height: i32) -> Self {
        Self {
            x: center.x - width / 2,
            y: center.y - height / 2,
            width,
            height,
        }
    }

    pub fn left(&self) -> i32 {
        self.x
    }

    pub fn right(&self) -> i32 {
        self.x + self.width
    }

    pub fn top(&self) -> i32 {
        self.y
    }

    pub fn bottom(&self) -> i32 {
        self.y + self.height
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.x + self.width / 2,
            self.y + self.height / 2,
        )
    }

    /// AABB collision detection
    pub fn intersects(&self, other: &Rect) -> bool {
        self.left() < other.right() &&
        self.right() > other.left() &&
        self.top() < other.bottom() &&
        self.bottom() > other.top()
    }
}

/// Facing direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    Left = -1,
    Right = 1,
}

impl Facing {
    pub fn opposite(&self) -> Facing {
        match self {
            Facing::Left => Facing::Right,
            Facing::Right => Facing::Left,
        }
    }

    pub fn sign(&self) -> i32 {
        match self {
            Facing::Left => -1,
            Facing::Right => 1,
        }
    }
}

/// Entity ID for tracking fighters, projectiles, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u32);

impl EntityId {
    pub const INVALID: EntityId = EntityId(u32::MAX);
}

/// Player ID (0 or 1 for two-player fighting game)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerId(pub u8);

impl PlayerId {
    pub const PLAYER_1: PlayerId = PlayerId(0);
    pub const PLAYER_2: PlayerId = PlayerId(1);
}

/// Frame counter for deterministic gameplay
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame(pub u64);

impl Frame {
    pub const ZERO: Frame = Frame(0);

    pub fn next(&self) -> Frame {
        Frame(self.0 + 1)
    }

    pub fn add(&self, delta: u64) -> Frame {
        Frame(self.0 + delta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_operations() {
        let v1 = Vec2::new(10, 20);
        let v2 = Vec2::new(5, 10);

        assert_eq!(v1.add(v2), Vec2::new(15, 30));
        assert_eq!(v1.sub(v2), Vec2::new(5, 10));
        assert_eq!(v1.scale(2), Vec2::new(20, 40));
        assert_eq!(v1.dot(v2), 250); // 10*5 + 20*10
    }

    #[test]
    fn test_rect_collision() {
        let r1 = Rect::new(0, 0, 10, 10);
        let r2 = Rect::new(5, 5, 10, 10);
        let r3 = Rect::new(20, 20, 10, 10);

        assert!(r1.intersects(&r2));
        assert!(!r1.intersects(&r3));
    }

    #[test]
    fn test_facing() {
        assert_eq!(Facing::Left.opposite(), Facing::Right);
        assert_eq!(Facing::Right.sign(), 1);
        assert_eq!(Facing::Left.sign(), -1);
    }
}
