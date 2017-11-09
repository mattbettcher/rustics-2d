#![warn(missing_docs)]


//! 
mod v2;
mod bounding_box;

// reexports
pub use v2::V2;
pub use bounding_box::BoundingBox;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
