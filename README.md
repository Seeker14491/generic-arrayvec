# generic_arrayvec

[![Build Status](https://travis-ci.org/Seeker14491/generic_arrayvec.svg?branch=master)](https://travis-ci.org/Seeker14491/generic_arrayvec)
[![Build status](https://ci.appveyor.com/api/projects/status/3bdv9mf81pftwkbh?svg=true)](https://ci.appveyor.com/project/Seeker14491/generic-arrayvec)

This crate provides interop between the [`arrayvec`](https://crates.io/crates/arrayvec) and [`generic_array`](https://crates.io/crates/generic-array/) crates, allowing you to use `generic_array`'s `GenericArray` datatype as the backing storage for the data structures in `arrayvec`. This lets you have vector and string types that store their contents inline, with a capacity that can be referred to in generic code.

## License

Licensed under either of

 * Apache License, Version 2.0
   (http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   (http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
