use ::common::HasGuid;

use ::boot_services::BootServices;
use ::boot_services::SearchKey;

use ::interface::simple_file_system::SimpleFileSystem;
use ::interface::file::OpenMode;
use ::interface::file::Attributes;
use ::interface::file::File;

use ::common::Char16;

use ::common::PAGE_SIZE;
use ::common::Address;
use ::common::Status;
use ::boot_services::AllocateType;
use ::boot_services::MemoryType;

use ::array::Array;

use core::mem;
use core::ops::Try;
use core::option::NoneError;

/// Tools
trait BootServicesEx {
    fn find_file(&self, filename: &[Char16], open_mode: OpenMode, attributes: Attributes) -> Result<&File, NoneError>;
    fn alloc<T: Sized>(&self, length: usize, memory_type: MemoryType) -> Result<Array<T>, Status>;
    fn free<T: Sized>(&self, array: Array<T>) -> Result<(), Status>;
}

impl BootServicesEx for BootServices {
    fn find_file(&self, filename: &[Char16], open_mode: OpenMode, attributes: Attributes) -> Result<&File, NoneError> {
        self
            .locate_handle_buffer(SearchKey::ByProtocol(SimpleFileSystem::GUID))
            .map_err(|_| NoneError)?
            .into_iter()
            .flat_map(|handle| self.handle_protocol(*handle).ok())
            .flat_map(|fs: &SimpleFileSystem| fs.open_volume().ok())
            .flat_map(|volume| volume.open(filename, open_mode, attributes))
            .next()
            .into_result()
    }

    fn alloc<T: Sized>(&self, length: usize, memory_type: MemoryType) -> Result<Array<T>, Status> {
        let size = length * mem::size_of::<T>();
        let pages = size / PAGE_SIZE + 1;

        let address = self
            .allocate_pages(AllocateType::AllocateAnyPages, memory_type, pages)?;

        Ok(unsafe { Array::from_raw(address.cast(), length) })
    }

    fn free<T: Sized>(&self, array: Array<T>) -> Result<(), Status> {
        let size = array.length() * mem::size_of::<T>();
        let pages = size / PAGE_SIZE + 1;

        let address = unsafe {
            let mut array = array;
            Address::from_raw(array.as_slice_mut().as_mut_ptr())
        };

        self.free_pages(address, pages)
    }
}
