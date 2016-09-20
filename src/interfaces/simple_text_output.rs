use ::common::Guid;
use ::common::Status;
use ::common::Bool;
use ::common::Char16;
use ::common::Uint;

use super::EfiResult;

use ::tools::result;
use ::tools::utf8_to_utf16;

pub const PROTOCOL: Guid =
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

#[derive(Copy, Clone)]
pub struct I {
    reset: extern fn (
        /* in */ this: *const I,
        /* in */ extended_verification: Bool
    ) -> Status,

    output_string: extern fn (
        /* in */ this: *const I,
        /* in */ string: *const Char16
    ) -> Status,
    test_string: extern fn (
        /* in */ this: *const I,
        /* in */ string: *const Char16
    ) -> Status,

    query_mode: extern fn (
        /* in */ this: *const I,
        /* in */ mode_number: Uint,
        /* out */ columns: *mut Uint,
        /* out */ rows: *mut Uint
    ) -> Status,
    set_mode: extern fn (
        /* in */ this: *const I,
        /* in */ mode_number: Uint
    ) -> Status,
    set_attribute: extern fn (
        /* in */ this: *const I,
        /* in */ attribute: Uint
    ) -> Status,

    clear_screen: extern fn (
        /* in */ this: *const I
    ) -> Status,
    set_cursor_position: extern fn (
        /* in */ this: *const I,
        /* in */ column: Uint,
        /* in */ row: Uint
    ) -> Status,
    enable_cursor: extern fn (
        /* in */ this: *const I,
        /* in */ enable: Bool
    ) -> Status,

    mode: *const SimpleTextOutputMode
}

impl I {
    pub fn reset(&self, extended_verification: Bool) -> EfiResult<()> {
        result(((*self).reset)(self as *const I, extended_verification), ())
    }
    pub fn output_string(&self, string: &str) -> EfiResult<()> {
        let utf16 = utf8_to_utf16(string);
        result(((*self).output_string)(self as *const I, &utf16[0]), ())
    }
    pub fn test_string(&self, string: &str) -> EfiResult<()> {
        let utf16 = utf8_to_utf16(string);
        result(((*self).test_string)(self as *const I, &utf16[0]), ())
    }

    pub fn query_mode(&self, mode_number: Uint) -> EfiResult<(Uint, Uint)> {
        let mut columns = 0;
        let mut rows = 0;
        result(((*self).query_mode)(self as *const I, mode_number, &mut columns, &mut rows), (columns, rows))
    }
}
