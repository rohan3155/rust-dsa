//! Top-level crate root for the data structures and algorithms library.
//!
//! Re-export commonly used types so clients can import them directly from
//! the crate root.

pub mod structures;
pub mod algorithms;

// re-export the most important data structures at the crate root so users can
// write `use rust_dsa::Stack` rather than `use rust_dsa::structures::stack::Stack`.
pub use structures::stack::{Stack};
