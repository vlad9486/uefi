use ::common::Uint;
use ::common::PAGE_SIZE;
use ::interfaces::EfiResult;

use core::marker::PhantomData;
use core::ops::Index;

use ::system_table::boot_services::BootServices;
use ::system_table::boot_services::PhysicalAddress;
use ::system_table::boot_services::AllocateType;
use ::system_table::boot_services::MemoryType;

pub struct DynamicArray<'a, T> where T: Sized + Clone {
    boot_services: &'a BootServices,
    pub ptr: PhysicalAddress,
    pub pages: Uint,
    pub size_of_entry: Uint,
    pub number_of_entries: Uint,
    phantom_data: PhantomData<T>
}

impl<'a, T> DynamicArray<'a, T> where T: Sized + Clone {
    pub fn new(boot_services: &'a BootServices, size_of_entry: Uint, number_of_entries: Uint) -> EfiResult<Self> {
        let pages = (size_of_entry * number_of_entries) / PAGE_SIZE + 1;

        let ptr = boot_services.allocate_pages(AllocateType::AllocateAnyPages, MemoryType::LoaderData, pages)?;

        Ok(DynamicArray {
            boot_services: boot_services,
            ptr: ptr,
            pages: pages,
            size_of_entry: size_of_entry,
            number_of_entries: number_of_entries,
            phantom_data: PhantomData
        })
    }

    pub fn set_entry(&mut self, index: Uint, entry: T) {
        if index < self.number_of_entries {
            let PhysicalAddress(address) = self.ptr;
            let address = (address + (index * self.size_of_entry) as u64) as *mut T;
            unsafe { *address = entry }
        }
    }

    pub fn count(&self) -> Uint {
        self.number_of_entries
    }
}

impl<'a, T> Index<Uint> for DynamicArray<'a, T> where T: Sized + Clone {
    type Output = T;

    fn index(&self, index: Uint) -> &Self::Output {
        if index < self.number_of_entries {
            let PhysicalAddress(address) = self.ptr;
            let address = (address + (index * self.size_of_entry) as u64) as *const T;
            unsafe { &*address }
        } else {
            panic!()
        }
    }
}

impl<'a, T> Drop for DynamicArray<'a, T> where T: Sized + Clone {
    fn drop(&mut self) {
        match self.boot_services.free_pages(self.ptr, self.pages) {
            Ok(_) => {
                self.pages = 0;
                self.ptr = PhysicalAddress(0)
            }
            Err(_) => panic!()
        }
    }
}
