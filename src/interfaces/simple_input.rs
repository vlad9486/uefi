use ::common::Guid;
use ::common::Status;
use ::common::Event;
use ::common::Bool;
use ::common::Char16;

use super::EfiResult;
use super::ProtocolImplementation;

use ::tools::result;

use core::marker::PhantomData;

pub const GUID: Guid =
    Guid(0x387477c1, 0x69c7, 0x11d2, [0x8e, 0x39, 0x0, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct InputKey {
    pub scan_code: u16,
    pub unicode_char: Char16
}

pub struct I {
    reset: extern "win64" fn (
        /* in */ this: *const I,
        /* in */ extended_verification: Bool
    ) -> Status,
    read_key_stroke: extern "win64" fn (
        /* in */ this: *const I,
        /* out */ key: *mut InputKey
    ) -> Status,

    wait_for_key: Event
}

impl I {
    pub fn reset(&self, extended_verification: Bool) -> EfiResult<()> {
        result(((*self).reset)(self as *const I, extended_verification), ())
    }
    pub fn read_key_stroke(&self) -> EfiResult<InputKey> {
        let mut key: InputKey = InputKey::default();
        result(((*self).read_key_stroke)(self as *const I, &mut key), key)
    }
    pub fn get_wait_for_key(&self) -> Event {
        self.wait_for_key
    }
}

impl ProtocolImplementation for I {
    fn get_guid() -> (Guid, PhantomData<Self>) {
        (GUID, PhantomData)
    }
}
