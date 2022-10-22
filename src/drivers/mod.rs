//! # `psoc6-hal`
//!
//! NOTE: This crate is currently a WIP!

//! These drivers typically take ownership of peripheral registers
//! and expose associated functionality. Some knowledge of the underlying
//! register structure is required to use these drivers.


pub mod backup;
pub mod cpuss;
pub mod flashc;
pub mod prot;
pub mod system;

