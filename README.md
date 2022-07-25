# generic-arrayvec

[![Build status](https://ci.appveyor.com/api/projects/status/3bdv9mf81pftwkbh?svg=true)](https://ci.appveyor.com/project/Seeker14491/generic-arrayvec)

This crate provides interop between the [`arrayvec`](https://crates.io/crates/arrayvec) v0.5 and
[`generic_array`](https://crates.io/crates/generic-array/) crates, allowing you to use `generic_array`'s `GenericArray`
datatype as the backing storage for the data structures in `arrayvec`. This lets you have vector and string types that
store their contents inline, with a capacity that can be referred to in generic code.

Note that as of its v0.6 release, [`arrayvec`](https://crates.io/crates/arrayvec) uses Rust's const generics feature to
implement functionality similar to this crate but with better ergonomics, so you may not need this crate anymore. Still,
const generics currently has [some limitations on stable Rust](https://github.com/rust-lang/rust/issues/76560) which
the approach this crate uses does not, so this crate can still be useful. Also, this crate can be a bit more memory
efficient due to its vectors storing their length using 1 byte when `capacity < 2^8`, 2 bytes when `capacity < 2^16`,
etc; in comparison `arrayvec` as of v0.7.2 always uses 4 bytes for this.

## License

Licensed under either of

- Apache License, Version 2.0
  (http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  (http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
