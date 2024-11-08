# `raspi-hal`

Concrete and friendly implementation of `embedded-hal` traits and [`embedded-hal-ext`](../embedded-hal-ext/README.md) traits for the Raspberry Pi computers.

Will likely use some of the existing [`linux-embedded-hal`](https://docs.rs/crate/linux-embedded-hal/0.4.0) crate(s), possibly some Pi-specific crates like [`rppal`](https://docs.rs/crate/rppal/latest). Possibly more from scratch.

Ideally we could ditch Linux as a dependency and have a bare-metal implementation. **LOOOOONG** term goal, that. Big project.


## Optional Cargo features

- **`async`**: async/await features

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.75 and up. It *might*
compile with older versions but that may change in any new patch release.

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](LICENSE)