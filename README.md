# Allomancy

Collection of one or more Rust crates intended to fill holes in the current `embedded-hal` traits and otherwise make embedded development, especially for more than one platform, easier.

## Goals
The stuff that makes sense to try and get included in [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) or one of its sister crates, I/we will make the attempt (and so effort should be made to follow their patterns in those cases). 
But it's likely that several things that end up getting built out here don't make sense for that, and may end up as something more like some of the [Embassy](https://github.com/embassy-rs/embassy) crates. Direct implementations of those traits for specific hardware, for example.



## Crates (current)
- [`embedded-hal-ext`](./embedded-hal-ext/README.md)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
