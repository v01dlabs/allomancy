[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](LICENSE)

# Allomancy

A collection of Rust crates intended to fill holes in the current `embedded-hal` traits 
and otherwise make embedded development, especially for more than one platform, easier.

## Goals
In general, everything in here should be compatible with a `#![no_std]` (and ideally no `alloc`) target 
unless there is a very good reason to do otherwise. This maximises compatibility with the most constrained
targets. Optional `std` features are reasonable, but all core functionality should be available for the 
minimal target.

The stuff that makes sense to try and get included in [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) 
or one of its sister crates, I/we will make the attempt (and so effort should be made to follow their patterns in those cases). 


## Crates (current)
- [`embedded-hal-ext`](./embedded-hal-ext/README.md)
- [`raspi-hal`](./raspi-hal/README.md)
- [`graphmemes`](./graphmemes.README.md)
