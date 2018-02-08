use core::nonzero::NonZero;

use core::ops::Index;
use core::ops::IndexMut;

use core::convert::AsRef;
use core::convert::AsMut;

use core::slice;
use core::mem;

use common::Word;

#[repr(C)]
#[derive(Eq, PartialEq)]
pub struct Pointer<T>
where
    T: Sized,
{
    raw: NonZero<*mut T>,
}

impl<T> AsRef<T> for Pointer<T>
where
    T: Sized,
{
    fn as_ref(&self) -> &T {
        unsafe { &*self.raw.get() }
    }
}

impl<T> AsMut<T> for Pointer<T>
where
    T: Sized,
{
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.raw.get() }
    }
}

impl<T> Pointer<T>
where
    T: Sized,
{
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        Pointer {
            raw: NonZero::new_unchecked(raw),
        }
    }
}

#[repr(C)]
#[derive(Eq, PartialEq)]
pub struct Array<T>
where
    T: Sized,
{
    length: Word,
    raw: NonZero<*mut T>,
}

impl<T> Index<Word> for Array<T>
where
    T: Sized,
{
    type Output = T;

    fn index(&self, index: Word) -> &Self::Output {
        let index = if index < self.length { index } else { 0 };
        unsafe { &*self.raw.get().offset(index as _) }
    }
}

impl<T> IndexMut<Word> for Array<T>
where
    T: Sized,
{
    fn index_mut(&mut self, index: Word) -> &mut Self::Output {
        let index = if index < self.length { index } else { 0 };
        unsafe { &mut *self.raw.get().offset(index as _) }
    }
}

impl<T> Array<T>
where
    T: Sized,
{
    pub fn length(&self) -> Word {
        self.length
    }

    pub unsafe fn from_raw(raw: *mut T, length: Word) -> Self {
        Array {
            length: length,
            raw: NonZero::new_unchecked(raw),
        }
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self.raw.get(), self.length * mem::size_of::<T>())
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.raw.get(), self.length * mem::size_of::<T>())
        }
    }
}

#[repr(C)]
#[derive(Eq, PartialEq)]
pub struct ArrayStride<T>
where
    T: Sized,
{
    length: Word,
    stride: Word,
    raw: NonZero<*mut T>,
}

impl<T> Index<Word> for ArrayStride<T>
where
    T: Sized,
{
    type Output = T;

    fn index(&self, index: Word) -> &Self::Output {
        let index = if index < self.length { index } else { 0 };
        unsafe { &*((self.raw.get() as *mut u8).offset((index * self.stride) as _) as *mut T) }
    }
}

impl<T> IndexMut<Word> for ArrayStride<T>
where
    T: Sized,
{
    fn index_mut(&mut self, index: Word) -> &mut Self::Output {
        let index = if index < self.length { index } else { 0 };
        unsafe { &mut *((self.raw.get() as *mut u8).offset((index * self.stride) as _) as *mut T) }
    }
}

impl<T> ArrayStride<T>
where
    T: Sized,
{
    pub unsafe fn from_raw(raw: *mut T, length: Word, stride: Word) -> Self {
        ArrayStride {
            length: length,
            stride: stride,
            raw: NonZero::new_unchecked(raw),
        }
    }

    pub fn length(&self) -> Word {
        self.length
    }
}
