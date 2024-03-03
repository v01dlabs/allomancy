# Allomancy

Collection of one or more Rust crates intended to fill holes in the current `embedded-hal` traits and otherwise make embedded development, especially for more than one platform, easier.

## Goals
The stuff that makes sense to try and get included in [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) or one of its sister crates, I/we will make the attempt (and so effort should be made to follow their patterns in those cases). 
But it's likely that several things that end up getting built out here don't make sense for that, and may end up as something more like some of the [Embassy](https://github.com/embassy-rs/embassy) crates. Direct implementations of those traits for specific hardware, for example.



## Crates (current)
- [`embedded-hal-ext`](./embedded-hal-ext/README.md)
- [`raspi-hal`](./raspi-hal/README.md)

## Building

I recommend using the [`cross`](https://github.com/cross-rs/cross) crate for compiling for other platforms (like the Pi or microcontrollers). Repository is currently configured to target the Pi 4 and 5 64-bit OS version, but you can specify the target in the `cross build` command or modify [Cross.toml](./Cross.toml) with the right target triple (`armv7-unknown-linux-gnueabihf` for 32-bit Raspberry Pi OS, `arm-unknown-linux-gnueabihf` for the Pi Zero, I believe). That works well for pretty much everything *except* Xtensa architecture ESP32s, which currently need a fork of `rustc` that supports the architecture. The `espup` crate and the docker containers Espressif provides can help with that toolchain. `cross` apparently has some issues running tests on crates, specifically anything that uses threads. Test with actual hardware if you can.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
