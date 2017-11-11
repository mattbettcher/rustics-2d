//#![warn(missing_docs)]

//! 
extern crate rayon;
extern crate nalgebra as na;

mod bounding_box;
mod body;
mod world;
mod mpr;

pub type Vector2 = na::Vector2<f32>;

// reexports
pub use na::*;
pub use bounding_box::BoundingBox;
pub use body::Body;
pub use world::World;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
