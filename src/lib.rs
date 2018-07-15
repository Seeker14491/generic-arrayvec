//! This crate provides interop between the [`arrayvec`](../arrayvec/index.html) and
//! [`generic_array`](../generic_array/index.html) crates, allowing you to use `generic_array`'s
//! [`GenericArray`] as the backing storage for the data structures in `arrayvec`. This lets you
//! have vector and string types that store their contents inline, with a capacity that can be
//! referred to in generic code.
//!
//! # Usage
//!
//! This crate exposes the type aliases [`GenericArrayVec`]s and [`GenericArrayString`], which are
//! aliases of datatypes in the `arrayvec` crate, so see the [`ArrayVec`] and [`ArrayString`] docs
//! to learn about their core functionality. Each one also has a corresponding extension trait
//! [`GenericArrayVecExt`] and [`GenericArrayStringExt`] that provide additional constructors and
//! conversions.
//!
//! An example of instanciating and pushing an item onto a `GenericArrayVec`:
//!
//! ```rust
//! # fn main() {
//! use generic_arrayvec::{GenericArrayVec, typenum::U5};
//!
//! // Create a new GenericArrayVec of inferred element type with a capacity of 5
//! let mut arr = GenericArrayVec::<_, U5>::new();
//!
//! arr.push(10);
//! # }
//! ```
//!
//! [`GenericArrayVec`]: type.GenericArrayVec.html
//! [`GenericArrayString`]: type.GenericArrayString.html
//! [`GenericArray`]: ../generic_array/struct.GenericArray.html
//! [`ArrayVec`]: ../arrayvec/struct.ArrayVec.html
//! [`ArrayString`]: ../arrayvec/struct.ArrayString.html
//! [`GenericArrayVecExt`]: trait.GenericArrayVecExt.html
//! [`GenericArrayStringExt`]: trait.GenericArrayStringExt.html

#![no_std]

pub extern crate arrayvec;
pub extern crate generic_array;

pub use generic_array::typenum;

use arrayvec::{Array, ArrayString, ArrayVec, CapacityError};
use generic_array::{ArrayLength, GenericArray};
use core::str::Utf8Error;

/// A [`GenericArray`]-backed [`ArrayVec`].
///
/// [`GenericArray`]: ../generic_array/struct.GenericArray.html
/// [`ArrayVec`]: ../arrayvec/struct.ArrayVec.html
pub type GenericArrayVec<T, N> = ArrayVec<Wrapper<T, N>>;

/// A [`GenericArray`]-backed [`ArrayString`].
///
/// [`GenericArray`]: ../generic_array/struct.GenericArray.html
/// [`ArrayString`]: ../arrayvec/struct.ArrayString.html
pub type GenericArrayString<N> = ArrayString<Wrapper<u8, N>>;

/// A wrapper around a [`GenericArray`] that implements the [`Array`] trait, allowing it to be used
/// as the backing store for [`ArrayVec`] and [`ArrayString`].
///
/// You don't need to use this type directly; it's used by other functions in this crate internally.
///
/// [`GenericArray`]: ../generic_array/struct.GenericArray.html
/// [`Array`]: ../arrayvec/trait.Array.html
/// [`ArrayVec`]: ../arrayvec/struct.ArrayVec.html
/// [`ArrayString`]: ../arrayvec/struct.ArrayString.html
#[derive(Debug)]
pub struct Wrapper<T, N>(pub GenericArray<T, N>)
where
    N: ArrayLength<T>;

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

/// Extension trait for [`GenericArrayString`].
///
/// See its impl on [`GenericArrayString`] for more info.
///
/// [`GenericArrayString`]: type.GenericArrayString.html
pub trait GenericArrayStringExt<N>
where
    N: ArrayLength<u8>,
{
    fn generic_from(string: &str) -> Result<GenericArrayString<N>, CapacityError<&str>>;

    fn generic_from_byte_string<A>(byte_string: &A) -> Result<GenericArrayString<N>, Utf8Error>
    where
        A: Into<GenericArray<u8, N>> + AsRef<[u8]>;
}

impl<T, N> Wrapper<T, N>
where
    N: ArrayLength<T>,
{
    /// Returns the inner `GenericArray` inside this `Wrapper`
    pub fn into_inner(self) -> GenericArray<T, N> {
        self.0
    }
}

unsafe impl<T, N> Array for Wrapper<T, N>
where
    N: ArrayLength<T>,
{
    type Item = T;
    type Index = usize;

    fn as_ptr(&self) -> *const Self::Item {
        self.0.as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut Self::Item {
        self.0.as_mut_ptr()
    }

    fn capacity() -> usize {
        N::to_usize()
    }
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

impl<T, N> GenericArrayVecExt<T, N> for GenericArrayVec<T, N>
where
    N: ArrayLength<T>,
{
    /// Creates a `GenericArrayVec` from an array or `GenericArray`.
    ///
    /// ```rust
    /// # fn main() {
    /// use generic_arrayvec::{GenericArrayVec, GenericArrayVecExt};
    ///
    /// let vec = GenericArrayVec::generic_from([2, 4, 6, 8]);
    ///
    /// assert_eq!(vec.len(), 4);
    /// assert_eq!(vec.capacity(), 4);
    /// # }
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
    /// # fn main() {
    /// use generic_arrayvec::{GenericArrayVec, GenericArrayVecExt, typenum::U5};
    ///
    /// let mut vec = GenericArrayVec::<i32, U5>::new();
    /// vec.push(0);
    /// vec.extend(1..5);
    ///
    /// let generic_array = vec.into_generic_array().unwrap();
    ///
    /// assert_eq!(&*generic_array, &[0, 1, 2, 3, 4][..]);
    /// # }
    /// ```
    fn into_generic_array(self) -> Result<GenericArray<T, N>, Self> {
        Ok(self.into_inner()?.into_inner())
    }
}

impl<N> GenericArrayStringExt<N> for GenericArrayString<N>
where
    N: ArrayLength<u8>,
{
    /// Creates a `GenericArrayString` from a `str`.
    ///
    /// Capacity is inferred from the type parameter.
    ///
    /// **Errors** if the capacity is not large enough to fit the string.
    ///
    /// ```rust
    /// # fn main() {
    /// use generic_arrayvec::{GenericArrayString, GenericArrayStringExt, typenum::U10};
    ///
    /// let string = GenericArrayString::<U10>::generic_from("hello").unwrap();
    ///
    /// assert_eq!(string.len(), 5);
    /// assert_eq!(string.capacity(), 10);
    /// # }
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
    /// # fn main() {
    /// use generic_arrayvec::{GenericArrayString, GenericArrayStringExt};
    ///
    /// let string = GenericArrayString::generic_from_byte_string(b"hello").unwrap();
    ///
    /// assert_eq!(string.len(), 5);
    /// assert_eq!(string.capacity(), 5);
    /// # }
    /// ```
    ///
    /// From a byte-holding `GenericArray`:
    ///
    /// ```rust
    /// # fn main() {
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
    /// # }
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

#[cfg(test)]
mod tests {
    use super::{
        generic_array::GenericArray, typenum::{U10, U41, U5}, *,
    };

    #[test]
    fn test_vec_simple() {
        let mut vec = GenericArrayVec::<i32, U41>::new();

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 41);
        vec.extend(0..20);
        assert_eq!(vec.len(), 20);
        assert_eq!(&vec[..5], &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_vec_from_array() {
        let vec: GenericArrayVec<i32, _> = GenericArrayVec::generic_from([0, 1, 2, 3, 4]);

        assert_zero_to_four(&vec);
    }

    #[test]
    fn test_vec_from_generic_array() {
        let arr: GenericArray<i32, U5> = GenericArray::clone_from_slice(&[0, 1, 2, 3, 4]);
        let vec = GenericArrayVec::generic_from(arr);

        assert_zero_to_four(&vec);
    }

    #[test]
    fn test_vec_from_iter() {
        let vec: GenericArrayVec<i32, U10> = (0..10).collect();

        assert_zero_to_four(&vec);
    }

    #[test]
    fn test_vec_into_generic_array() {
        let vec: GenericArrayVec<i32, _> = GenericArrayVec::generic_from([0, 1, 2, 3, 4]);
        let arr = vec.into_generic_array().unwrap();

        assert_zero_to_four(&arr);
    }

    #[test]
    fn test_string_from() {
        let string = GenericArrayString::<U10>::generic_from("hello").unwrap();

        assert_eq!(&string, "hello");
    }

    #[test]
    fn test_string_from_byte_string_literal() {
        let byte_string = b"hello";
        let string = GenericArrayString::<U5>::generic_from_byte_string(byte_string).unwrap();

        assert_eq!(&string, "hello");
    }

    #[test]
    fn test_string_from_byte_string_generic() {
        let byte_string = GenericArray::from(b"hello".clone());
        let string = GenericArrayString::<U5>::generic_from_byte_string(&byte_string).unwrap();

        assert_eq!(&string, "hello");
    }

    fn assert_zero_to_four<T>(vec: &T)
    where
        T: AsRef<[i32]>,
    {
        assert_eq!(&vec.as_ref()[..5], &[0, 1, 2, 3, 4][..]);
    }
}
