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
//! An example of instantiating and pushing an item onto a `GenericArrayVec`:
//!
//! ```rust
//! use generic_arrayvec::{GenericArrayVec, typenum::U5};
//!
//! // Create a new GenericArrayVec of inferred element type with a capacity of 5
//! let mut arr = GenericArrayVec::<_, U5>::new();
//!
//! arr.push(10);
//! ```
//!
//! [arrayvec]: arrayvec
//! [generic_array]: generic_array
//! [`GenericArrayVec`]: type.GenericArrayVec.html
//! [`GenericArrayString`]: type.GenericArrayString.html
//! [`GenericArray`]: https://docs.rs/generic-array/0.14/generic_array/struct.GenericArray.html
//! [`ArrayVec`]: https://docs.rs/arrayvec/0.5/arrayvec/struct.ArrayVec.html
//! [`ArrayString`]: https://docs.rs/arrayvec/0.5/arrayvec/struct.ArrayString.html
//! [`GenericArrayVecExt`]: trait.GenericArrayVecExt.html
//! [`GenericArrayStringExt`]: trait.GenericArrayStringExt.html

#![no_std]
#![doc(html_root_url = "https://docs.rs/generic-arrayvec/0.3.0")]
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
use generic_array::{ArrayLength, GenericArray};

/// A [`GenericArray`]-backed [`ArrayVec`].
///
/// [`GenericArray`]: https://docs.rs/generic-array/0.14/generic_array/struct.GenericArray.html
/// [`ArrayVec`]: https://docs.rs/arrayvec/0.5/arrayvec/struct.ArrayVec.html
pub type GenericArrayVec<T, N> = ArrayVec<Wrapper<T, N>>;

/// A [`GenericArray`]-backed [`ArrayString`].
///
/// [`GenericArray`]: https://docs.rs/generic-array/0.14/generic_array/struct.GenericArray.html
/// [`ArrayString`]: https://docs.rs/arrayvec/0.5/arrayvec/struct.ArrayString.html
pub type GenericArrayString<N> = ArrayString<Wrapper<u8, N>>;

/// A wrapper around a [`GenericArray`] that implements the [`Array`] trait from the arrayvec
/// crate, allowing it to be used as the backing store for [`ArrayVec`] and [`ArrayString`].
///
/// You probably don't need to use this type directly; just use the constructors provided by the
/// [`GenericArrayVecExt`] and [`GenericArrayStringExt`] traits.
///
/// [`GenericArray`]: https://docs.rs/generic-array/0.14/generic_array/struct.GenericArray.html
/// [`Array`]: https://docs.rs/arrayvec/0.5/arrayvec/trait.Array.html
/// [`ArrayVec`]: https://docs.rs/arrayvec/0.5/arrayvec/struct.ArrayVec.html
/// [`ArrayString`]: https://docs.rs/arrayvec/0.5/arrayvec/struct.ArrayString.html
/// [`GenericArrayVecExt`]: trait.GenericArrayVecExt.html
/// [`GenericArrayStringExt`]: trait.GenericArrayStringExt.html
#[derive(Debug, Clone)]
pub struct Wrapper<T, N>(pub GenericArray<T, N>)
where
    N: ArrayLength<T>;

impl<T, N> Wrapper<T, N>
where
    N: ArrayLength<T>,
{
    /// Returns the inner `GenericArray` inside this `Wrapper`
    pub fn into_inner(self) -> GenericArray<T, N> {
        self.0
    }
}

impl<T, N> Copy for Wrapper<T, N>
where
    T: Copy,
    N: ArrayLength<T>,
    N::ArrayType: Copy,
{
}

impl<T, N> From<GenericArray<T, N>> for Wrapper<T, N>
where
    N: ArrayLength<T>,
{
    fn from(arr: GenericArray<T, N>) -> Self {
        Wrapper(arr)
    }
}

impl<T, N> Into<GenericArray<T, N>> for Wrapper<T, N>
where
    N: ArrayLength<T>,
{
    fn into(self) -> GenericArray<T, N> {
        self.0
    }
}

unsafe impl<T, N> Array for Wrapper<T, N>
where
    N: ArrayLength<T>,
{
    type Item = T;
    type Index = usize;
    const CAPACITY: usize = N::USIZE;

    fn as_slice(&self) -> &[Self::Item] {
        self.0.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self.0.as_mut_slice()
    }
}

/// Extension trait for [`GenericArrayVec`].
///
/// See its impl on [`GenericArrayVec`] for more info.
///
/// [`GenericArrayVec`]: type.GenericArrayVec.html
pub trait GenericArrayVecExt<T, N>
where
    N: ArrayLength<T>,
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
    N: ArrayLength<T>,
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
        ArrayVec::from(Wrapper::from(arr.into()))
    }

    /// Returns the inner `GenericArray`, if `self` is full to its capacity.
    ///
    /// **Errors** if `self` is not filled to capacity.
    ///
    /// ```rust
    /// use generic_arrayvec::{GenericArrayVec, GenericArrayVecExt, typenum::U5};
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
///
/// [`GenericArrayString`]: type.GenericArrayString.html
pub trait GenericArrayStringExt<N>
where
    N: ArrayLength<u8>,
    N::ArrayType: Copy,
{
    fn generic_from(string: &str) -> Result<GenericArrayString<N>, CapacityError<&str>>;

    fn generic_from_byte_string<A>(byte_string: &A) -> Result<GenericArrayString<N>, Utf8Error>
    where
        A: Into<GenericArray<u8, N>> + AsRef<[u8]>;
}

impl<N> GenericArrayStringExt<N> for GenericArrayString<N>
where
    N: ArrayLength<u8>,
    N::ArrayType: Copy,
{
    /// Creates a `GenericArrayString` from a `str`.
    ///
    /// Capacity is inferred from the type parameter.
    ///
    /// **Errors** if the capacity is not large enough to fit the string.
    ///
    /// ```rust
    /// use generic_arrayvec::{GenericArrayString, GenericArrayStringExt, typenum::U10};
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
    /// use generic_arrayvec::{
    ///     GenericArrayString, GenericArrayStringExt,
    ///     generic_array::GenericArray,
    /// };
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
        ArrayString::from_byte_string(&Wrapper::from(GenericArray::clone_from_slice(
            byte_string.as_ref(),
        )))
    }
}
