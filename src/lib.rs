//#![warn(missing_docs)]

//! 
extern crate rayon;

mod v2;
mod matrix;
mod bounding_box;
mod body;
mod world;
mod mpr;

// reexports
pub use v2::V2;
pub use matrix::M22;
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
