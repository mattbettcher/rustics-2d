use std::f32;
use std::ops::*;
use V2;

/// 2D Bounding Box
#[derive(Debug)]
pub struct BoundingBox {
    /// Minimum point of the bounding box.
    pub min: V2,
    /// Maximum point of the bounding box.
    pub max: V2,
}

impl BoundingBox {
    // constants

    /// Smallest possible vector.
    pub const MIN: BoundingBox = BoundingBox {
        min: V2::MAX,
        max: V2::MIN,
    };

    /// Largest possible vector.
    pub const MAX: BoundingBox = BoundingBox {
        min: V2::MIN,
        max: V2::MAX,
    };

    // methods

    /// Returns true if the given point lies inside the bounding box.
    /// 
    pub fn contains_point(&self, point: &V2) -> bool {
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
    pub fn add_point(&mut self, point: &V2) {
        self.min = V2::min(&self.min, point);
        self.max = V2::max(&self.max, point);
    }

    /// Returns the corners of the bounding box as a `Vec` of points.
    pub fn get_corners(&self) -> Vec<V2> {
        vec![
            V2::new(self.min.x, self.max.y),
            V2::new(self.max.x, self.max.y),
            V2::new(self.max.x, self.min.y),
            V2::new(self.min.x, self.min.y),
        ]
    }

    /// Returns the center of the bounding box.
    pub fn center(&self) -> V2 {
        V2 { x: self.min.x + self.max.x * 0.5, y: self.min.y + self.max.y * 0.5 }
    }

    // associated functions

    /// Constructs a new bounding box.
    ///
    /// ```rust
    /// 
    /// ```
    #[inline]
    pub fn new(min: V2, max: V2) -> Self {
        BoundingBox { min, max }
    }
}


impl From<Vec<V2>> for BoundingBox {
    // todo- try to rewrite this as a single line iterator chain!
    fn from(points: Vec<V2>) -> Self {
        let mut bb = BoundingBox::MIN;
        for point in &points {
            bb.add_point(&point);
        }
        bb
    }
}
