#![doc(html_root_url = "https://docs.rs/generic-arrayvec/0.3.1")]

//! This crate provides interop between the [arrayvec] and [generic_array] crates, allowing you to
//! use generic_array's [`GenericArray`] as the backing storage for the data structures in
//! arrayvec. This lets you have vector and string types that store their contents inline, with a
//! capacity that can be referred to in generic code.
//!
//! # Usage
//!
//! This crate exposes the type aliases [`GenericArrayVec`]s and [`GenericArrayString`], which are
//! aliases of datatypes in the `arrayvec` crate. See the [`ArrayVec`] and [`ArrayString`] docs
//! to learn about their functionality. Each one also has a corresponding extension trait
//! [`GenericArrayVecExt`] and [`GenericArrayStringExt`] that provide additional constructors and
//! conversions.
//!
//! ## Example
//!
//! ```rust
//! use generic_arrayvec::arrayvec::Array;
//! use generic_arrayvec::typenum::{Prod, U2, U4};
//! use generic_arrayvec::{ArrayvecStorage, Capacity, GenericArrayVec};
//! use std::ops::Mul;
//!
//! fn main() {
//!     // Create a vector of `u8`s with a capacity of 4.
//!     let mut arr = GenericArrayVec::<u8, U4>::new();
//!     assert_eq!(arr.capacity(), 4);
//!
//!     // Add some elements to it.
//!     arr.push(1);
//!     arr.push(2);
//!     arr.push(3);
//!     assert_eq!(&arr[..], &[1, 2, 3]);
//!
//!     // To demonstrate generic bounds, we call our `double()` function, which is defined below.
//!     // This function returns a new vector with double the capacity of the input vector.
//!     // The new vector contains two copies of the input vector's items.
//!     let doubled = double(&arr);
//!     assert_eq!(&doubled[..], &[1, 2, 3, 1, 2, 3]);
//!     assert_eq!(doubled.capacity(), 8);
//! }
//!
//! fn double<N>(original: &GenericArrayVec<u8, N>) -> GenericArrayVec<u8, Prod<N, U2>>
//! where
//!     // Boilerplate bounds for the input array.
//!     N: Capacity<u8>,
//!     ArrayvecStorage<u8, N>: Array<Item = u8>,
//!
//!     // Boilerplate bounds for the output array. Note it's the same as above, but
//!     // `N` -> `Prod<N, U2>`.
//!     Prod<N, U2>: Capacity<u8>,
//!     ArrayvecStorage<u8, Prod<N, U2>>: Array<Item = u8>,
//!
//!     N: Mul<U2>,
//! {
//!     let mut new = GenericArrayVec::<u8, Prod<N, U2>>::new();
//!
//!     // These `unwrap()`s can never fail.
//!     new.try_extend_from_slice(original.as_slice()).unwrap();
//!     new.try_extend_from_slice(original.as_slice()).unwrap();
//!
//!     new
//! }
//! ```
//!
//! ## `where` bound boilerplate
//!
//! When working with a [`GenericArrayVec<T, N>`] where `T` and/or `N` are not concrete types, you
//! will need to always include certain bounds in your `where` clauses, or you will get a compile
//! error. This dummy function shows how to specify them:
//!
//! ```rust
//! use generic_arrayvec::arrayvec::Array;
//! use generic_arrayvec::{ArrayvecStorage, Capacity, GenericArrayVec};
//!
//! fn f<T, N>(_arr: GenericArrayVec<T, N>)
//! where
//!     N: Capacity<T>,
//!     ArrayvecStorage<T, N>: Array<Item = T>,
//! {
//! }
//! ```
//!
//! And this is how you specify them for [`GenericArrayString<N>`]:
//!
//! ```rust
//! use generic_arrayvec::arrayvec::Array;
//! use generic_arrayvec::{ArrayvecStorage, Capacity, GenericArrayString};
//!
//! fn f<N>(_arr: GenericArrayString<N>)
//! where
//!     N: Capacity<u8>,
//!     N::ArrayType: Copy,
//!     ArrayvecStorage<u8, N>: Array<Item = u8>,
//! {
//! }
//! ```

#![no_std]
#![warn(
    rust_2018_idioms,
    deprecated_in_future,
    macro_use_extern_crate,
    missing_debug_implementations,
    unused_qualifications
)]

pub use arrayvec;
pub use generic_array::{self, typenum};

use arrayvec::{Array, ArrayString, ArrayVec, CapacityError};
use core::str::Utf8Error;
use generic_array::typenum::{IsLess, U1, U2, U256, U4294967296, U65536};
use generic_array::{ArrayLength, GenericArray};
use plumbing::{ArrayvecStorageRaw, IndexForCapacity, PickIndexBreakpointsForCapacity};

/// Low-level implementation details you shouldn't need to touch.
pub mod plumbing;

/// A [`GenericArray`]-backed [`ArrayVec`].
pub type GenericArrayVec<T, N> = ArrayVec<ArrayvecStorage<T, N>>;

/// A [`GenericArray`]-backed [`ArrayString`].
pub type GenericArrayString<N> = ArrayString<ArrayvecStorage<u8, N>>;

/// A wrapper around a [`GenericArray`] that implements the [`Array`] trait from the arrayvec
/// crate, allowing it to be used as the backing store for [`ArrayVec`] and [`ArrayString`].
///
/// You likely won't need to interact with this type directly, except in `where` clauses when
/// working with [`GenericArrayVec`] and [`GenericArrayString`]; see their docs for details.
pub type ArrayvecStorage<T, N> = ArrayvecStorageRaw<T, N, IndexForCapacity<N>>;

/// A trait implemented by `typenum`'s unsigned integers, which lets them be used to define the
/// capacity of [`GenericArrayVec`]/[`GenericArrayString`].
pub trait Capacity<T>:
    ArrayLength<T>
    + PickIndexBreakpointsForCapacity
    + IsLess<U1>
    + IsLess<U2>
    + IsLess<U256>
    + IsLess<U65536>
    + IsLess<U4294967296>
{
}

impl<N, T> Capacity<T> for N where
    N: ArrayLength<T>
        + PickIndexBreakpointsForCapacity
        + IsLess<U1>
        + IsLess<U2>
        + IsLess<U256>
        + IsLess<U65536>
        + IsLess<U4294967296>
{
}

/// Extension trait for [`GenericArrayVec`].
///
/// See its impl on [`GenericArrayVec`] for more info.
pub trait GenericArrayVecExt<T, N>
where
    N: Capacity<T>,
    ArrayvecStorage<T, N>: Array,
{
    fn generic_from<A>(arr: A) -> GenericArrayVec<T, N>
    where
        A: Into<GenericArray<T, N>>;

    fn into_generic_array(self) -> Result<GenericArray<T, N>, Self>
    where
        Self: Sized;
}

impl<T, N> GenericArrayVecExt<T, N> for GenericArrayVec<T, N>
where
    N: Capacity<T>,
    ArrayvecStorage<T, N>: Array,
{
    /// Creates a `GenericArrayVec` from an array or `GenericArray`.
    ///
    /// ```rust
    /// use generic_arrayvec::{GenericArrayVec, GenericArrayVecExt};
    ///
    /// let vec = GenericArrayVec::generic_from([2, 4, 6, 8]);
    ///
    /// assert_eq!(vec.len(), 4);
    /// assert_eq!(vec.capacity(), 4);
    /// ```
    fn generic_from<A>(arr: A) -> GenericArrayVec<T, N>
    where
        A: Into<GenericArray<T, N>>,
    {
        ArrayVec::from(ArrayvecStorage::from(arr.into()))
    }

    /// Returns the inner `GenericArray`, if `self` is full to its capacity.
    ///
    /// **Errors** if `self` is not filled to capacity.
    ///
    /// ```rust
    /// use generic_arrayvec::typenum::U5;
    /// use generic_arrayvec::{GenericArrayVec, GenericArrayVecExt};
    ///
    /// let mut vec = GenericArrayVec::<i32, U5>::new();
    /// vec.push(0);
    /// vec.extend(1..5);
    ///
    /// let generic_array = vec.into_generic_array().unwrap();
    ///
    /// assert_eq!(&*generic_array, &[0, 1, 2, 3, 4][..]);
    /// ```
    fn into_generic_array(self) -> Result<GenericArray<T, N>, Self> {
        Ok(self.into_inner()?.into_inner())
    }
}

/// Extension trait for [`GenericArrayString`].
///
/// See its impl on [`GenericArrayString`] for more info.
pub trait GenericArrayStringExt<N>
where
    N: Capacity<u8>,
    ArrayvecStorage<u8, N>: Array<Item = u8>,

    N::ArrayType: Copy,
{
    fn generic_from(string: &str) -> Result<GenericArrayString<N>, CapacityError<&str>>;

    fn generic_from_byte_string<A>(byte_string: &A) -> Result<GenericArrayString<N>, Utf8Error>
    where
        A: Into<GenericArray<u8, N>> + AsRef<[u8]>;
}

impl<N> GenericArrayStringExt<N> for GenericArrayString<N>
where
    N: Capacity<u8>,
    ArrayvecStorage<u8, N>: Array<Item = u8>,

    N::ArrayType: Copy,
{
    /// Creates a `GenericArrayString` from a `str`.
    ///
    /// Capacity is inferred from the type parameter.
    ///
    /// **Errors** if the capacity is not large enough to fit the string.
    ///
    /// ```rust
    /// use generic_arrayvec::typenum::U10;
    /// use generic_arrayvec::{GenericArrayString, GenericArrayStringExt};
    ///
    /// let string = GenericArrayString::<U10>::generic_from("hello").unwrap();
    ///
    /// assert_eq!(string.len(), 5);
    /// assert_eq!(string.capacity(), 10);
    /// ```
    fn generic_from(string: &str) -> Result<GenericArrayString<N>, CapacityError<&str>> {
        ArrayString::from(string)
    }

    /// Creates a `GenericArrayString` from a byte string.
    ///
    /// The `GenericArrayString`'s length and capacity will be equal to the input byte string.
    ///
    /// **Errors** if the byte string is not valid UTF-8.
    ///
    /// # Examples
    ///
    /// From a byte string literal:
    ///
    /// ```rust
    /// use generic_arrayvec::{GenericArrayString, GenericArrayStringExt};
    ///
    /// let string = GenericArrayString::generic_from_byte_string(b"hello").unwrap();
    ///
    /// assert_eq!(string.len(), 5);
    /// assert_eq!(string.capacity(), 5);
    /// ```
    ///
    /// From a byte-holding `GenericArray`:
    ///
    /// ```rust
    /// use generic_arrayvec::generic_array::GenericArray;
    /// use generic_arrayvec::{GenericArrayString, GenericArrayStringExt};
    ///
    /// let arr = GenericArray::from([b'h', b'i']);
    /// let string = GenericArrayString::generic_from_byte_string(&arr).unwrap();
    ///
    /// assert_eq!(string.len(), 2);
    /// assert_eq!(string.capacity(), 2);
    /// ```
    fn generic_from_byte_string<A>(byte_string: &A) -> Result<GenericArrayString<N>, Utf8Error>
    where
        A: Into<GenericArray<u8, N>> + AsRef<[u8]>,
    {
        ArrayString::from_byte_string(&ArrayvecStorage::from(GenericArray::clone_from_slice(
            byte_string.as_ref(),
        )))
    }
}

mod private {
    #[allow(missing_debug_implementations)]
    pub enum Sealed {}
}
