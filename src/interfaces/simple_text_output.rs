use ::common::Guid;
use ::common::Status;
use ::common::Bool;
use ::common::Char16;
use ::common::Uint;

use super::EfiResult;
use super::ProtocolImplementation;

use ::tools::result;

use core::marker::PhantomData;

pub const GUID: Guid =
    Guid(0x387477c2, 0x69c7, 0x11d2, [0x8e, 0x39, 0x0, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);

#[derive(Copy, Clone)]
pub struct SimpleTextOutputMode {
    pub max_mode: i32,
    pub mode: i32,
    pub attribute: i32,
    pub cursor_column: i32,
    pub cursor_row: i32,
    pub cursor_visible: Bool
}

pub struct I {
    reset: extern "win64" fn (
        /* in */ this: *const I,
        /* in */ extended_verification: Bool
    ) -> Status,

    output_string: extern "win64" fn (
        /* in */ this: *const I,
        /* in */ string: *const Char16
    ) -> Status,
    test_string: extern "win64" fn (
        /* in */ this: *const I,
        /* in */ string: *const Char16
    ) -> Status,

    query_mode: extern "win64" fn (
        /* in */ this: *const I,
        /* in */ mode_number: Uint,
        /* out */ columns: *mut Uint,
        /* out */ rows: *mut Uint
    ) -> Status,
    set_mode: extern "win64" fn (
        /* in */ this: *const I,
        /* in */ mode_number: Uint
    ) -> Status,
    set_attribute: extern "win64" fn (
        /* in */ this: *const I,
        /* in */ attribute: Uint
    ) -> Status,

    clear_screen: extern "win64" fn (
        /* in */ this: *const I
    ) -> Status,
    set_cursor_position: extern "win64" fn (
        /* in */ this: *const I,
        /* in */ column: Uint,
        /* in */ row: Uint
    ) -> Status,
    enable_cursor: extern "win64" fn (
        /* in */ this: *const I,
        /* in */ enable: Bool
    ) -> Status,

    mode: &'static SimpleTextOutputMode
}

impl I {
    pub fn reset(&self, extended_verification: bool) -> EfiResult<()> {
        let reset = (*self).reset;
        let status = reset(self as *const I, if extended_verification { 1 } else { 0 });
        result(status, ())
    }
    pub fn output_string(&self, string: &[Char16]) -> EfiResult<()> {
        let output_string = (*self).output_string;
        let c_str = string.as_ptr();
        let status = output_string(self as *const I, c_str);
        result(status, ())
    }
    pub fn test_string(&self, string: &[Char16]) -> EfiResult<()> {
        let test_string = (*self).test_string;
        let c_str = string.as_ptr();
        let status = test_string(self as *const I, c_str);
        result(status, ())
    }

    pub fn query_mode(&self, mode_number: Uint) -> EfiResult<(Uint, Uint)> {
        let query_mode = (*self).query_mode;
        let mut columns = 0;
        let mut rows = 0;
        let status = query_mode(self as *const I, mode_number, &mut columns, &mut rows);
        result(status, (columns, rows))
    }
    pub fn set_mode(&self, mode_number: Uint) -> EfiResult<()> {
        let set_mode = (*self).set_mode;
        let status = set_mode(self as *const I, mode_number);
        result(status, ())
    }
    pub fn set_attribute(&self, attribute: Uint) -> EfiResult<()> {
        let set_attribute = (*self).set_attribute;
        let status = set_attribute(self as *const I, attribute);
        result(status, ())
    }

    pub fn clear_screen(&self) -> EfiResult<()> {
        let clear_screen = (*self).clear_screen;
        let status = clear_screen(self as *const I);
        result(status, ())
    }
    pub fn set_cursor_position(&self, column: Uint, row: Uint) -> EfiResult<()> {
        let set_cursor_position = (*self).set_cursor_position;
        let status = set_cursor_position(self as *const I, column, row);
        result(status, ())
    }
    pub fn enable_cursor(&self, enable: bool) -> EfiResult<()> {
        let enable_cursor = (*self).enable_cursor;
        let status = enable_cursor(self as *const I, if enable { 1 } else { 0 });
        result(status, ())
    }

    pub fn get_mode(&self) -> SimpleTextOutputMode {
        *self.mode
    }
}

impl ProtocolImplementation for I {
    fn get_guid() -> (Guid, PhantomData<Self>) {
        (GUID, PhantomData)
    }
}
