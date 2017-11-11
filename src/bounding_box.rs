use Vector2;
use std::f32;

/// 2D Bounding Box
#[derive(Debug)]
pub struct BoundingBox {
    /// Minimum point of the bounding box.
    pub min: Vector2,
    /// Maximum point of the bounding box.
    pub max: Vector2,
}

impl BoundingBox {
    // constants

    /// Smallest possible vector.
    pub fn smallest() -> Self { 
        BoundingBox {
            min: Vector2::new(f32::MAX, f32::MAX),
            max: Vector2::new(f32::MIN, f32::MIN),
        }
    }

    /// Largest possible vector.
    pub fn largest() -> Self { 
        BoundingBox {
            min: Vector2::new(f32::MIN, f32::MIN),
            max: Vector2::new(f32::MAX, f32::MAX),
        }
    }

    // methods

    /// Returns true if the given point lies inside the bounding box.
    /// 
    pub fn contains_point(&self, point: &Vector2) -> bool {
        self.min.x <= point.x && self.min.y <= point.y && self.max.x >= point.x
            && self.max.y >= point.y
    }

    /// Returns true if the given bounding box intersects this bounding box.
    ///  
    pub fn contains_bbox(&self, other: &BoundingBox) -> bool {
        self.max.x >= other.min.x && self.min.x <= other.max.x && self.max.y >= other.min.y
            && self.min.y <= other.max.y
    }

    /// Expands the bounding box to include the given point.
    /// 
    pub fn add_point(&mut self, point: Vector2) {
        self.min.x = f32::min(self.min.x, point.x);
        self.min.y = f32::min(self.min.y, point.y);
        self.max.x = f32::max(self.min.x, point.x);
        self.max.y = f32::max(self.min.y, point.y);
    }

    /// Returns the corners of the bounding box as a `Vec` of points.
    pub fn get_corners(&self) -> Vec<Vector2> {
        vec![
            Vector2::new(self.min.x, self.max.y),
            Vector2::new(self.max.x, self.max.y),
            Vector2::new(self.max.x, self.min.y),
            Vector2::new(self.min.x, self.min.y),
        ]
    }

    /// Returns the center of the bounding box.
    pub fn center(&self) -> Vector2 {
        Vector2::new(self.min.x + self.max.x * 0.5, self.min.y + self.max.y * 0.5)
    }

    // associated functions

    /// Constructs a new bounding box.
    ///
    /// ```rust
    /// 
    /// ```
    #[inline]
    pub fn new(min: Vector2, max: Vector2) -> Self {
        BoundingBox { min, max }
    }
}


impl From<Vec<Vector2>> for BoundingBox {
    // todo- try to rewrite this as a single line iterator chain!
    fn from(points: Vec<Vector2>) -> Self {
        let mut bb = BoundingBox::smallest();
        for point in &points {
            bb.add_point(*point);
        }
        bb
    }
}
