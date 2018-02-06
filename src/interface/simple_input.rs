use common::Char16;
use common::Bool;
use common::Event;
use common::Guid;
use common::Status;

use common::HasGuid;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InputKey {
    pub scan_code: u16,
    pub unicode_char: Char16,
}

#[repr(C)]
pub struct SimpleInput {
    reset: extern "win64" fn(
        /* in */ this: *const SimpleInput,
        /* in */ extended_verification: Bool,
    ) -> Status,
    read_key_stroke: extern "win64" fn(
        /* in */ this: *const SimpleInput,
        /* out */ key: *mut InputKey,
    ) -> Status,

    wait_for_key: Event,
}

impl SimpleInput {
    pub fn reset(&self, extended_verification: Bool) -> Result<(), Status> {
        ((*self).reset)(self, extended_verification).check(())
    }
    pub fn read_key_stroke(&self) -> Result<InputKey, Status> {
        let mut key: InputKey = InputKey {
            scan_code: 0,
            unicode_char: 0,
        };
        ((*self).read_key_stroke)(self, &mut key).check(key)
    }

    pub fn get_wait_for_key(&self) -> Event {
        self.wait_for_key
    }
}

impl HasGuid for SimpleInput {
    const GUID: Guid = Guid(
        0x387477c1,
        0x69c7,
        0x11d2,
        [0x8e, 0x39, 0x0, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}
