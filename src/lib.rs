#![warn(missing_docs)]


//! 
mod v2;

// reexports
pub use v2::V2;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
