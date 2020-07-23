// Crate root of the library crate

// Export for other libs to use

//! # Reference
//!
//! `reference` is a collection of basic to advanced Rust features and frameworks.
//!
 
pub use crate::{
    basics::*,
    collections::*,
    rustisms::*
};

// Import modules
pub mod basics;
pub mod collections;
pub mod rustisms;