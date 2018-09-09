use core::ptr::NonNull;

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
    raw: NonNull<T>,
}

impl<T> AsRef<T> for Pointer<T>
where
    T: Sized,
{
    fn as_ref(&self) -> &T {
        unsafe { self.raw.as_ref() }
    }
}

impl<T> AsMut<T> for Pointer<T>
where
    T: Sized,
{
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.raw.as_mut() }
    }
}

impl<T> Pointer<T>
where
    T: Sized,
{
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        Pointer {
            raw: NonNull::new_unchecked(raw),
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
    raw: NonNull<T>,
}

impl<T> Index<Word> for Array<T>
where
    T: Sized,
{
    type Output = T;

    fn index(&self, index: Word) -> &Self::Output {
        let index = if index < self.length { index } else { 0 };
        unsafe { &*self.raw.as_ptr().offset(index as _) }
    }
}

impl<T> IndexMut<Word> for Array<T>
where
    T: Sized,
{
    fn index_mut(&mut self, index: Word) -> &mut Self::Output {
        let index = if index < self.length { index } else { 0 };
        unsafe { &mut *self.raw.as_ptr().offset(index as _) }
    }
}

impl<T> Array<T>
where
    T: Sized,
{
    pub unsafe fn from_raw(raw: *mut T, length: Word) -> Self {
        Array {
            length: length,
            raw: NonNull::new_unchecked(raw),
        }
    }

    pub fn length(&self) -> Word {
        self.length
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.raw.as_ref(), self.length * mem::size_of::<T>()) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.raw.as_mut(), self.length * mem::size_of::<T>()) }
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
    raw: NonNull<T>,
}

impl<T> Index<Word> for ArrayStride<T>
where
    T: Sized,
{
    type Output = T;

    fn index(&self, index: Word) -> &Self::Output {
        let index = if index < self.length { index } else { 0 };
        unsafe { &*((self.raw.as_ptr() as *mut u8).offset((index * self.stride) as _) as *mut T) }
    }
}

impl<T> IndexMut<Word> for ArrayStride<T>
where
    T: Sized,
{
    fn index_mut(&mut self, index: Word) -> &mut Self::Output {
        let index = if index < self.length { index } else { 0 };
        unsafe { &mut *((self.raw.as_ptr() as *mut u8).offset((index * self.stride) as _) as *mut T) }
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
            raw: NonNull::new_unchecked(raw),
        }
    }

    pub fn length(&self) -> Word {
        self.length
    }

    pub fn stride(&self) -> Word {
        self.stride
    }

    pub fn as_ptr(&self) -> *const T {
        self.raw.as_ptr()
    }

    pub fn as_mut_ptr(&self) -> *mut T {
        self.raw.as_ptr()
    }
}
