use std::f32;
use std::ops::{Mul, AddAssign, Neg, Sub, Add, SubAssign};

/// 2D Vector
#[derive(Debug, Clone, Copy)]
pub struct V2 {
    /// X component of the vector.
    pub x: f32,
    /// Y component of the vector.
    pub y: f32,
}

impl V2 {
    // constants

    /// Smallest possible vector.
    pub const MIN: V2 = V2 {
        x: f32::MIN,
        y: f32::MIN,
    };

    /// Largest possible vector.
    pub const MAX: V2 = V2 {
        x: f32::MAX,
        y: f32::MAX,
    };

    /// Zero vector.
    pub const ZERO: V2 = V2 { x: 0.0, y: 0.0 };

    // methods

    /// Returns the length of the vector.
    ///
    /// ```rust
    /// # use rustics_2d::V2;
    /// assert_eq!(V2::new(5.0, 5.0).len(), 7.071068);
    /// ```
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns the squared length of the vector.
    ///
    /// ```rust
    /// # use rustics_2d::V2;
    /// assert_eq!(V2::new(5.0, 5.0).len_sq(), 50.0);
    /// ```
    pub fn len_sq(&self) -> f32 {
        (self.x * self.x + self.y * self.y)
    }

    /// Normalizes the vector to a length of 0 - 1 without 
    /// changing its direction.
    ///
    /// ```rust
    /// # use rustics_2d::V2;
    /// let mut v = V2::new(5.0, 5.0);
    /// assert_eq!(v.len(), 7.071068);
    /// v.norm();
    /// assert_eq!(v.len(), 1.0);
    /// ```
    pub fn norm(&mut self) {
        let dot = V2::dot(&self, &self);
        let inv_dot = 1.0 / dot.sqrt();
        self.x *= inv_dot;
        self.y *= inv_dot;
    }

    // associated functions

    /// Constructs a new 2 dimensional vector.
    ///
    /// ```rust
    /// # use rustics_2d::V2;
    /// assert_eq!(V2::new(1.0, 2.0), V2{x: 1.0, y: 2.0});
    /// ```
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        V2 { x, y }
    }

    /// Returns the minimum x and y components from two vectors.
    ///
    /// ```rust
    /// # use rustics_2d::V2;
    /// let a = V2::new(1.0, 2.0);
    /// let b = V2::new(2.0, 1.0);
    /// assert_eq!(V2::min(&a, &b), V2{x: 1.0, y: 1.0});
    /// ```
    pub fn min(a: &V2, b: &V2) -> Self {
        V2 {
            x: if a.x < b.x { a.x } else { b.x },
            y: if a.y < b.y { a.y } else { b.y },
        }
    }

    /// Returns the maximum x and y components from two vectors.
    ///
    /// ```rust
    /// # use rustics_2d::V2;
    /// let a = V2::new(1.0, 2.0);
    /// let b = V2::new(2.0, 1.0);
    /// assert_eq!(V2::max(&a, &b), V2{x: 2.0, y: 2.0});
    /// ```
    pub fn max(a: &V2, b: &V2) -> Self {
        V2 {
            x: if a.x > b.x { a.x } else { b.x },
            y: if a.y > b.y { a.y } else { b.y },
        }
    }

    /// Calculates the dot product of the given vectors.
    ///
    /// ```rust
    /// # use rustics_2d::V2;
    /// let a = V2::new(1.0, 2.0);
    /// let b = V2::new(2.0, 1.0);
    /// assert_eq!(V2::dot(&a, &b), 4.0);
    /// ```
    #[inline]
    pub fn dot(a: &V2, b: &V2) -> f32 {
        a.x * b.x + a.y * b.y
    }

    /// Calculates the cross product of the given vectors.
    ///
    /// ```rust
    /// # use rustics_2d::V2;
    /// let a = V2::new(1.0, 2.0);
    /// let b = V2::new(2.0, 1.0);
    /// assert_eq!(V2::cross(&a, &b), 0.0);
    /// ```
    #[inline]
    pub fn cross(a: &V2, b: &V2) -> f32 {
        a.x * b.x - a.y * b.y
    }
}

impl Add for V2 {
    type Output = V2;

    fn add(self, other: V2) -> V2 {
        V2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for V2 {
    fn add_assign(&mut self, other: V2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for V2 {
    type Output = V2;

    fn sub(self, other: V2) -> V2 {
        V2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for V2 {
    fn sub_assign(&mut self, other: V2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul for V2 {
    type Output = f32;

    fn mul(self, other: V2) -> f32 {
        V2::dot(&self, &other)
    }
}

impl Mul<f32> for V2 {
    type Output = V2;

    fn mul(self, other: f32) -> V2 {
        V2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Mul<V2> for f32 {
    type Output = V2;

    fn mul(self, other: V2) -> V2 {
        V2 {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

impl Neg for V2 {
    type Output = V2;

    fn neg(self) -> V2 {
        V2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl PartialEq for V2 {
    fn eq(&self, other: &V2) -> bool {
        self.x == other.x && self.y == other.y
    }
}
