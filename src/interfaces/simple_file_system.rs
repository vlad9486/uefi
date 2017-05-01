use ::common::Guid;
use ::common::Status;

use super::EfiResult;
use super::ProtocolImplementation;

use super::file::I as File;

use core::marker::PhantomData;

pub const GUID: Guid =
    Guid(0x0964e5b22, 0x6459, 0x11d2, [0x8e, 0x39, 0x0, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);

pub const REVISION1: u32 = 0x00010000;

#[repr(C)]
pub struct I {
    revision: u64,

    open_volume: extern "win64" fn (
        /* in */ this: *const I,
        /* out */ root: *mut *const File
    ) -> Status
}

impl I {
    pub fn open_volume(&self) -> EfiResult<&'static File> {
        let open_volume = (*self).open_volume;
        let mut file: *const File = 0 as _;
        let status = open_volume(self as *const I, &mut file);
        if status == 0 {
            let file = unsafe {
                &*file
            };
            Ok(file)
        } else {
            Err(status)
        }
    }
}

impl ProtocolImplementation for I {
    fn get_guid() -> (Guid, PhantomData<Self>) {
        (GUID, PhantomData)
    }
}
