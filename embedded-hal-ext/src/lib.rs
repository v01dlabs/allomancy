#![doc = include_str!("../README.md")]
//#![warn(missing_docs)]
#![no_std]
#![cfg_attr(nightly, allow(stable_features, unknown_lints))]
#![cfg_attr(nightly, feature(async_fn_in_trait, impl_trait_projections))]
#![allow(async_fn_in_trait)]

pub mod digital;

// needed to prevent defmt macros from breaking, since they emit code that does `defmt::blahblah`.
#[cfg(feature = "defmt")]
use defmt;


