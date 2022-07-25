use crate::private::Sealed;
use arrayvec::Array;
use core::fmt::Debug;
use core::marker::PhantomData;
use generic_array::typenum::{False, IsLess, True, U0, U1, U2, U256, U4294967296, U65536};
use generic_array::{ArrayLength, GenericArray};

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ArrayvecStorageRaw<T, N, I>(pub GenericArray<T, N>, PhantomData<I>)
where
    N: ArrayLength<T>;

impl<T, N, I> ArrayvecStorageRaw<T, N, I>
where
    N: ArrayLength<T>,
{
    /// Wraps the given `GenericArray` into a new `Wrapper`.
    pub fn new(arr: GenericArray<T, N>) -> Self {
        ArrayvecStorageRaw::from(arr)
    }

    /// Returns the inner `GenericArray` inside this `Wrapper`
    pub fn into_inner(self) -> GenericArray<T, N> {
        self.0
    }
}

impl<T, N, I> Copy for ArrayvecStorageRaw<T, N, I>
where
    T: Copy,
    N: ArrayLength<T>,
    N::ArrayType: Copy,
    I: Copy,
{
}

impl<T, N, I> From<GenericArray<T, N>> for ArrayvecStorageRaw<T, N, I>
where
    N: ArrayLength<T>,
{
    fn from(arr: GenericArray<T, N>) -> Self {
        ArrayvecStorageRaw(arr, PhantomData)
    }
}

impl<T, N, I> From<ArrayvecStorageRaw<T, N, I>> for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn from(wrapper: ArrayvecStorageRaw<T, N, I>) -> Self {
        wrapper.0
    }
}

unsafe impl<T> Array for ArrayvecStorageRaw<T, U0, ()> {
    type Item = T;
    type Index = ();
    const CAPACITY: usize = 0;

    fn as_slice(&self) -> &[Self::Item] {
        self.0.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self.0.as_mut_slice()
    }
}

unsafe impl<T> Array for ArrayvecStorageRaw<T, U1, bool> {
    type Item = T;
    type Index = bool;
    const CAPACITY: usize = 1;

    fn as_slice(&self) -> &[Self::Item] {
        self.0.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self.0.as_mut_slice()
    }
}

unsafe impl<T, N> Array for ArrayvecStorageRaw<T, N, u8>
where
    N: ArrayLength<T> + IsLess<U256>,
    <N as IsLess<U256>>::Output: IsTrue,
{
    type Item = T;
    type Index = u8;
    const CAPACITY: usize = N::USIZE;

    fn as_slice(&self) -> &[Self::Item] {
        self.0.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self.0.as_mut_slice()
    }
}

unsafe impl<T, N> Array for ArrayvecStorageRaw<T, N, u16>
where
    N: ArrayLength<T> + IsLess<U65536>,
    <N as IsLess<U65536>>::Output: IsTrue,
{
    type Item = T;
    type Index = u16;
    const CAPACITY: usize = N::USIZE;

    fn as_slice(&self) -> &[Self::Item] {
        self.0.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self.0.as_mut_slice()
    }
}

unsafe impl<T, N> Array for ArrayvecStorageRaw<T, N, u32>
where
    N: ArrayLength<T> + IsLess<U4294967296>,
    <N as IsLess<U4294967296>>::Output: IsTrue,
{
    type Item = T;
    type Index = u32;
    const CAPACITY: usize = N::USIZE;

    fn as_slice(&self) -> &[Self::Item] {
        self.0.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self.0.as_mut_slice()
    }
}

unsafe impl<T, N> Array for ArrayvecStorageRaw<T, N, usize>
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

pub trait IsTrue {
    fn _sealed(_: Sealed);
}

impl IsTrue for True {
    fn _sealed(_: Sealed) {}
}

pub trait PickIndex {
    type Output: Copy;
    fn _sealed(_: Sealed);
}

impl PickIndex for (True, True, True, True, True) {
    type Output = ();
    fn _sealed(_: Sealed) {}
}

impl PickIndex for (False, True, True, True, True) {
    type Output = bool;
    fn _sealed(_: Sealed) {}
}

impl PickIndex for (False, False, True, True, True) {
    type Output = u8;
    fn _sealed(_: Sealed) {}
}

impl PickIndex for (False, False, False, True, True) {
    type Output = u16;
    fn _sealed(_: Sealed) {}
}

impl PickIndex for (False, False, False, False, True) {
    type Output = u32;
    fn _sealed(_: Sealed) {}
}

impl PickIndex for (False, False, False, False, False) {
    type Output = usize;
    fn _sealed(_: Sealed) {}
}

pub type PickIndexBreakpoints<N> = (
    <N as IsLess<U1>>::Output,
    <N as IsLess<U2>>::Output,
    <N as IsLess<U256>>::Output,
    <N as IsLess<U65536>>::Output,
    <N as IsLess<U4294967296>>::Output,
);

pub type IndexForCapacity<N> =
    <<N as PickIndexBreakpointsForCapacity>::PickIndexBreakpoints as PickIndex>::Output;

pub trait PickIndexBreakpointsForCapacity {
    type PickIndexBreakpoints: PickIndex;
    fn _sealed(_: Sealed);
}

impl<N> PickIndexBreakpointsForCapacity for N
where
    N: IsLess<U1> + IsLess<U2> + IsLess<U256> + IsLess<U65536> + IsLess<U4294967296>,
    PickIndexBreakpoints<N>: PickIndex,
{
    type PickIndexBreakpoints = PickIndexBreakpoints<N>;
    fn _sealed(_: Sealed) {}
}
