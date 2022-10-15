//! # `psoc6-hal`
//!
//! NOTE: This crate is currently a WIP!

#![no_std]
#![feature(try_trait_v2)]

pub use embedded_hal as ehal;
pub use psoc6_pac as pac;

pub mod delay;
pub mod gpio;
pub mod clocks;
pub mod prelude;
pub mod drivers;
pub mod psoc;
