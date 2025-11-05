/// Core types for the Bagarre fighting game engine
/// Zero dependencies - all implementations are custom

/// A 2D vector for positions, velocities, and other 2D quantities.
///
/// Uses fixed-point integer math for deterministic gameplay. All values are in
/// "internal units" - divide by 1000 to convert to display units.
///
/// # Examples
///
/// ```
/// use bagarre::types::Vec2;
///
/// let pos = Vec2::new(5000, 3000); // 5.0, 3.0 in display units
/// let vel = Vec2::new(1000, 0);     // 1.0, 0.0 in display units
/// let new_pos = pos.add(vel);       // 6000, 3000
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    /// X component (horizontal, right is positive)
    pub x: i32,
    /// Y component (vertical, down is positive)
    pub y: i32,
}

impl Vec2 {
    /// The zero vector (0, 0)
    pub const ZERO: Vec2 = Vec2 { x: 0, y: 0 };

    /// Creates a new vector with the given x and y components.
    ///
    /// # Examples
    ///
    /// ```
    /// use bagarre::types::Vec2;
    ///
    /// let v = Vec2::new(100, 200);
    /// assert_eq!(v.x, 100);
    /// assert_eq!(v.y, 200);
    /// ```
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Adds two vectors component-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use bagarre::types::Vec2;
    ///
    /// let v1 = Vec2::new(10, 20);
    /// let v2 = Vec2::new(5, 10);
    /// assert_eq!(v1.add(v2), Vec2::new(15, 30));
    /// ```
    pub fn add(&self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    /// Subtracts two vectors component-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use bagarre::types::Vec2;
    ///
    /// let v1 = Vec2::new(10, 20);
    /// let v2 = Vec2::new(5, 10);
    /// assert_eq!(v1.sub(v2), Vec2::new(5, 10));
    /// ```
    pub fn sub(&self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    /// Scales a vector by a scalar value.
    ///
    /// # Examples
    ///
    /// ```
    /// use bagarre::types::Vec2;
    ///
    /// let v = Vec2::new(10, 20);
    /// assert_eq!(v.scale(2), Vec2::new(20, 40));
    /// ```
    pub fn scale(&self, scalar: i32) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    /// Computes the dot product of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use bagarre::types::Vec2;
    ///
    /// let v1 = Vec2::new(10, 20);
    /// let v2 = Vec2::new(5, 10);
    /// assert_eq!(v1.dot(v2), 250); // 10*5 + 20*10
    /// ```
    pub fn dot(&self, other: Vec2) -> i32 {
        self.x * other.x + self.y * other.y
    }

    /// Returns the squared length of the vector.
    ///
    /// More efficient than computing the actual length since it avoids a square root.
    /// Useful for distance comparisons.
    ///
    /// # Examples
    ///
    /// ```
    /// use bagarre::types::Vec2;
    ///
    /// let v = Vec2::new(3, 4);
    /// assert_eq!(v.length_squared(), 25); // 3² + 4² = 9 + 16
    /// ```
    pub fn length_squared(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}

/// An axis-aligned bounding box (AABB) rectangle.
///
/// Used for hitboxes, hurtboxes, pushboxes, and collision detection.
/// Origin is at the top-left corner, with positive X to the right and positive Y down.
///
/// # Examples
///
/// ```
/// use bagarre::types::Rect;
///
/// let rect = Rect::new(0, 0, 10000, 10000); // 10x10 box at origin
/// let other = Rect::new(5000, 5000, 10000, 10000); // Overlapping box
/// assert!(rect.intersects(&other));
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    /// X coordinate of the left edge
    pub x: i32,
    /// Y coordinate of the top edge
    pub y: i32,
    /// Width of the rectangle
    pub width: i32,
    /// Height of the rectangle
    pub height: i32,
}

impl Rect {
    /// Creates a new rectangle with the given position and dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use bagarre::types::Rect;
    ///
    /// let rect = Rect::new(100, 200, 50, 75);
    /// assert_eq!(rect.x, 100);
    /// assert_eq!(rect.width, 50);
    /// ```
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { x, y, width, height }
    }

    /// Creates a rectangle from a center point and dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use bagarre::types::{Rect, Vec2};
    ///
    /// let center = Vec2::new(10000, 10000);
    /// let rect = Rect::from_center(center, 2000, 3000);
    /// assert_eq!(rect.center(), center);
    /// ```
    pub fn from_center(center: Vec2, width: i32, height: i32) -> Self {
        Self {
            x: center.x - width / 2,
            y: center.y - height / 2,
            width,
            height,
        }
    }

    /// Returns the X coordinate of the left edge.
    pub fn left(&self) -> i32 {
        self.x
    }

    /// Returns the X coordinate of the right edge.
    pub fn right(&self) -> i32 {
        self.x + self.width
    }

    /// Returns the Y coordinate of the top edge.
    pub fn top(&self) -> i32 {
        self.y
    }

    /// Returns the Y coordinate of the bottom edge.
    pub fn bottom(&self) -> i32 {
        self.y + self.height
    }

    /// Returns the center point of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use bagarre::types::{Rect, Vec2};
    ///
    /// let rect = Rect::new(0, 0, 10, 20);
    /// assert_eq!(rect.center(), Vec2::new(5, 10));
    /// ```
    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.x + self.width / 2,
            self.y + self.height / 2,
        )
    }

    /// Tests if this rectangle intersects with another using AABB collision detection.
    ///
    /// # Examples
    ///
    /// ```
    /// use bagarre::types::Rect;
    ///
    /// let r1 = Rect::new(0, 0, 10, 10);
    /// let r2 = Rect::new(5, 5, 10, 10);  // Overlapping
    /// let r3 = Rect::new(20, 20, 10, 10); // Not overlapping
    ///
    /// assert!(r1.intersects(&r2));
    /// assert!(!r1.intersects(&r3));
    /// ```
    pub fn intersects(&self, other: &Rect) -> bool {
        self.left() < other.right() &&
        self.right() > other.left() &&
        self.top() < other.bottom() &&
        self.bottom() > other.top()
    }
}

/// The direction a character or entity is facing.
///
/// Used for relative input handling (back/forward) and sprite rendering.
/// Left faces towards negative X, Right faces towards positive X.
///
/// # Examples
///
/// ```
/// use bagarre::types::Facing;
///
/// let facing = Facing::Right;
/// assert_eq!(facing.sign(), 1);
/// assert_eq!(facing.opposite(), Facing::Left);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    /// Facing left (towards negative X)
    Left = -1,
    /// Facing right (towards positive X)
    Right = 1,
}

impl Facing {
    /// Returns the opposite facing direction.
    ///
    /// # Examples
    ///
    /// ```
    /// use bagarre::types::Facing;
    ///
    /// assert_eq!(Facing::Left.opposite(), Facing::Right);
    /// assert_eq!(Facing::Right.opposite(), Facing::Left);
    /// ```
    pub fn opposite(&self) -> Facing {
        match self {
            Facing::Left => Facing::Right,
            Facing::Right => Facing::Left,
        }
    }

    /// Returns the sign of the facing direction (-1 for left, 1 for right).
    ///
    /// Useful for multiplying with velocities or offsets to flip them based on facing.
    ///
    /// # Examples
    ///
    /// ```
    /// use bagarre::types::Facing;
    ///
    /// assert_eq!(Facing::Right.sign(), 1);
    /// assert_eq!(Facing::Left.sign(), -1);
    /// ```
    pub fn sign(&self) -> i32 {
        match self {
            Facing::Left => -1,
            Facing::Right => 1,
        }
    }
}

/// A unique identifier for entities in the game.
///
/// Used to track fighters, projectiles, and other game objects.
///
/// # Examples
///
/// ```
/// use bagarre::types::EntityId;
///
/// let id = EntityId(0);
/// assert_ne!(id, EntityId::INVALID);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u32);

impl EntityId {
    /// Invalid entity ID used to represent "no entity"
    pub const INVALID: EntityId = EntityId(u32::MAX);
}

/// Player identifier (0 or 1 for a two-player fighting game).
///
/// # Examples
///
/// ```
/// use bagarre::types::PlayerId;
///
/// let p1 = PlayerId::PLAYER_1;
/// let p2 = PlayerId::PLAYER_2;
/// assert_ne!(p1, p2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerId(pub u8);

impl PlayerId {
    /// Player 1 (index 0)
    pub const PLAYER_1: PlayerId = PlayerId(0);
    /// Player 2 (index 1)
    pub const PLAYER_2: PlayerId = PlayerId(1);
}

/// Frame counter for deterministic gameplay.
///
/// Represents the current game frame. At 60 FPS, frame 60 equals 1 second of gameplay.
///
/// # Examples
///
/// ```
/// use bagarre::types::Frame;
///
/// let frame = Frame::ZERO;
/// let next_frame = frame.next();
/// assert_eq!(next_frame, Frame(1));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame(pub u64);

impl Frame {
    /// Frame zero (start of the game)
    pub const ZERO: Frame = Frame(0);

    /// Returns the next frame.
    ///
    /// # Examples
    ///
    /// ```
    /// use bagarre::types::Frame;
    ///
    /// let frame = Frame(5);
    /// assert_eq!(frame.next(), Frame(6));
    /// ```
    pub fn next(&self) -> Frame {
        Frame(self.0 + 1)
    }

    /// Adds a delta to this frame.
    ///
    /// # Examples
    ///
    /// ```
    /// use bagarre::types::Frame;
    ///
    /// let frame = Frame(10);
    /// assert_eq!(frame.add(5), Frame(15));
    /// ```
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
