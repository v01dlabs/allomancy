# `raspi-hal`

Concrete and friendly implementation of `embedded-hal` traits and [`embedded-hal-ext`](../embedded-hal-ext/README.md) traits for the Raspberry Pi computers.

Will likely use some of the existing [`linux-embedded-hal`](https://docs.rs/crate/linux-embedded-hal/0.4.0) crate(s), possibly some Pi-specific crates like [`rppal`](https://docs.rs/crate/rppal/latest). Possibly more from scratch.

Ideally we could ditch Linux as a dependency and have a bare-metal implementation. **LOOOOONG** term goal, that. Big project.


## Optional Cargo features

- **`async`**: async/await features

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.75 and up. It *might*
compile with older versions but that may change in any new patch release.


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
