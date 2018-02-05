use ::common::Uint;
use ::common::PAGE_SIZE;
use ::interfaces::EfiResult;

use core::ops::Index;
use core::ops::IndexMut;
use core::nonzero::NonZero;
use core::mem;

use ::system_table::boot_services::BootServices;
use ::system_table::boot_services::PhysicalAddress;
use ::system_table::boot_services::AllocateType;
use ::system_table::boot_services::MemoryType;

pub trait Array<T> {
    fn raw(&self) -> NonZero<*mut T>;
    fn stride(&self) -> Uint;
    fn count(&self) -> Uint;

    fn pages(&self) -> Uint {
        (self.stride() * self.count()) / PAGE_SIZE + 1
    }

    fn physical_address(&self) -> PhysicalAddress {
        PhysicalAddress(self.raw().get() as _)
    }
}

impl<T> Index<Uint> for Array<T> where T: Sized {
    type Output = T;

    fn index(&self, index: Uint) -> &Self::Output {
        let index = if index < self.count() { index } else { 0 };
        unsafe { &*self.raw().get().offset(index as _) }
    }
}

impl<T> IndexMut<Uint> for Array<T> where T: Sized {
    fn index_mut(&mut self, index: Uint) -> &mut Self::Output {
        let index = if index < self.count() { index } else { 0 };
        unsafe { &mut *self.raw().get().offset(index as _) }
    }
}

pub struct DynamicArray<'a, T> where T: Sized {
    boot_services: &'a BootServices,
    pointer: NonZero<*mut T>,
    count: Uint,
}

impl<'a, T> DynamicArray<'a, T> where T: Sized {
    pub fn new(boot_services: &'a BootServices, count: Uint) -> EfiResult<Self> {
        let pages = (mem::size_of::<T>() * count) / PAGE_SIZE + 1;

        let PhysicalAddress(address) = boot_services.allocate_pages(AllocateType::AllocateAnyPages, MemoryType::LoaderData, pages)?;

        match NonZero::new(address as _) {
            Some(raw_ptr) => Ok(DynamicArray {
                boot_services: boot_services,
                pointer: raw_ptr,
                count: count,
            }),
            None => Err(1),
        }
    }
}

impl<'a, T> Array<T> for DynamicArray<'a, T> where T: Sized {
    fn raw(&self) -> NonZero<*mut T> {
        self.pointer
    }

    fn stride(&self) -> Uint {
        mem::size_of::<T>()
    }

    fn count(&self) -> Uint {
        self.count
    }
}

impl<'a, T> Drop for DynamicArray<'a, T> where T: Sized {
    fn drop(&mut self) {
        match self.boot_services.free_pages(self.physical_address(), self.pages()) {
            Ok(_) => self.count = 0,
            Err(_) => panic!()
        }
    }
}

pub struct DynamicArrayStride<'a, T> where T: Sized {
    boot_services: &'a BootServices,
    pointer: NonZero<*mut T>,
    count: Uint,
    stride: Uint,
}

impl<'a, T> DynamicArrayStride<'a, T> where T: Sized {
    pub fn new(boot_services: &'a BootServices, count: Uint, stride: Uint) -> EfiResult<Self> {
        assert!(stride >= mem::size_of::<T>());

        let pages = (stride * count) / PAGE_SIZE + 1;

        let PhysicalAddress(address) = boot_services.allocate_pages(AllocateType::AllocateAnyPages, MemoryType::LoaderData, pages)?;

        match NonZero::new(address as _) {
            Some(raw_ptr) => Ok(DynamicArrayStride {
                boot_services: boot_services,
                pointer: raw_ptr,
                count: count,
                stride: stride,
            }),
            None => Err(1),
        }
    }
}

impl<'a, T> Array<T> for DynamicArrayStride<'a, T> where T: Sized {
    fn raw(&self) -> NonZero<*mut T> {
        self.pointer
    }

    fn stride(&self) -> Uint {
        self.stride
    }

    fn count(&self) -> Uint {
        self.count
    }
}

impl<'a, T> Drop for DynamicArrayStride<'a, T> where T: Sized {
    fn drop(&mut self) {
        match self.boot_services.free_pages(self.physical_address(), self.pages()) {
            Ok(_) => self.count = 0,
            Err(_) => panic!()
        }
    }
}
