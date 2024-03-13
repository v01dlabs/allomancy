#![doc = include_str!("../README.md")]
//#![warn(missing_docs)]
#![cfg_attr(nightly, allow(stable_features, unknown_lints))]
#![cfg_attr(feature = "async", allow(stable_features, async_fn_in_trait))]

#![allow(async_fn_in_trait)]
// Need to redo the implementation I have of this elsewhere
// Just a stub for now
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate nix;


#[macro_use]
mod macros;
pub mod gpio;
pub mod chip;
pub mod peripheral;

pub(crate) mod private {
    pub trait Sealed {}
}

