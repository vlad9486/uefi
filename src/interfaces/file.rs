use ::common::Status;
use ::common::Char16;
use ::common::Uint;

use super::EfiResult;

use core::convert::Into;
use core::slice;

pub const REVISION1: u32 = 0x00010000;
pub const REVISION2: u32 = 0x00020000;

pub struct I {
    revision: u64,

    open: extern "win64" fn (
        /* in */ this: *const I,
        /* out */ new_handle: *mut *const I,
        /* in */ file_name: *const Char16,
        /* in */ open_mode: u64,
        /* in */ attributes: u64
    ) -> Status,
    close: extern "win64" fn (
        /* in */ this: *const I
    ) -> Status,
    delete: extern "win64" fn (
        /* in */ this: *const I
    ) -> Status,
    read: extern "win64" fn (
        /* in */ this: *const I,
        /* in out */ buffer_size: *mut Uint,
        /* in */ buffer: *mut ()
    ) -> Status,
    write: extern "win64" fn (
        /* in */ this: *const I
    ) -> Status,
    get_position: extern "win64" fn (
        /* in */ this: *const I
    ) -> Status,
    set_position: extern "win64" fn (
        /* in */ this: *const I
    ) -> Status,
    get_info: extern "win64" fn (
        /* in */ this: *const I
    ) -> Status,
    set_info: extern "win64" fn (
        /* in */ this: *const I
    ) -> Status,
    flush: extern "win64" fn (
        /* in */ this: *const I
    ) -> Status,

    // Available only if revision == REVISION2
    open_ex: extern "win64" fn (
        /* in */ this: *const I
    ) -> Status,
    read_ex: extern "win64" fn (
        /* in */ this: *const I
    ) -> Status,
    write_ex: extern "win64" fn (
        /* in */ this: *const I
    ) -> Status,
    flush_ex: extern "win64" fn (
        /* in */ this: *const I
    ) -> Status,
}

#[derive(Copy, Clone)]
pub enum OpenMode {
    Read,
    ReadWrite,
    CreateReadWrite
}

impl Into<u64> for OpenMode {
    fn into(self) -> u64 {
        let read = 0x0000000000000001;
        let write = 0x0000000000000002;
        let create = 0x8000000000000000;
        match self {
            OpenMode::Read => read,
            OpenMode::ReadWrite => read | write,
            OpenMode::CreateReadWrite => create | read | write
        }
    }
}

bitflags! {
    pub flags Attributes: u64 {
        const READ_ONLY = 1 << 0,
        const HIDDEN    = 1 << 1,
        const SYSTEM    = 1 << 2,
        const RESERVED  = 1 << 3,
        const DIRECTORY = 1 << 4,
        const ARCHIVE   = 1 << 5
    }
}

impl I {
    pub fn open(&self, file_name: &[Char16], open_mode: OpenMode, attributes: Attributes) -> EfiResult<&'static I> {
        let open = self.open;
        let mut other: *const I = 0 as _;
        let c_str = file_name.as_ptr();
        let status = open(self as *const I, &mut other, c_str, open_mode.into(), attributes.bits());
        if status == 0 {
            let other = unsafe {
                &*other
            };
            Ok(other)
        } else {
            Err(status)
        }
    }

    pub fn read(&self, mut buffer: &mut [u8]) -> EfiResult<Uint> {
        let read = self.read;
        let mut size = buffer.len() as Uint;
        let ptr = buffer.as_mut_ptr();
        let status = read(self as *const I, &mut size, ptr as *mut ());
        if status == 0 {
            buffer = unsafe {
                slice::from_raw_parts_mut(ptr, size as usize)
            };
            Ok(size)
        } else {
            Err(status)
        }
    }
}
