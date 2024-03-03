# `embedded-hal-ext`

## Embedded HAL Extensions

A set of useful traits to allow more sophisticated cross-platform embedded code than that supported by the embedded-hal crates.

## Design Goals

Reduce need for platform-specific code in embedded peripheral drivers, crates, and applications by abstracting additional common functionality.


### Features:
- [ ] On-the-fly GPIO configuration. Allow drivers to reconfigure GPIO pins they own (e.g. bidirectional use of interrupt pins in some I2C devices).
- [ ] Additional non-blocking traits
- [ ] interrupts/message-passing
- [ ] Analog trait?


## Optional Cargo features

- **`defmt`**: Derive `defmt::Format` from `defmt` 0.3 for enums and structs.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.75 and up. It *might*
compile with older versions but that may change in any new patch release.


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
