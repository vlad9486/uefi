use common::Char16;
use common::Bool;
use common::Guid;
use common::Word;
use common::Status;

use common::HasGuid;

use array::Pointer;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SimpleTextOutputMode {
    pub max_mode: i32,
    pub mode: i32,
    pub attribute: i32,
    pub cursor_column: i32,
    pub cursor_row: i32,
    pub cursor_visible: Bool,
}

#[repr(C)]
pub struct SimpleTextOutput {
    reset: extern "win64" fn(
        /* in */ this: *const SimpleTextOutput,
        /* in */ extended_verification: Bool,
    ) -> Status,

    output_string: extern "win64" fn(
        /* in */ this: *const SimpleTextOutput,
        /* in */ string: *const Char16,
    ) -> Status,
    test_string: extern "win64" fn(
        /* in */ this: *const SimpleTextOutput,
        /* in */ string: *const Char16,
    ) -> Status,

    query_mode: extern "win64" fn(
        /* in */ this: *const SimpleTextOutput,
        /* in */ mode_number: Word,
        /* out */ columns: *mut Word,
        /* out */ rows: *mut Word,
    ) -> Status,
    set_mode: extern "win64" fn(
        /* in */ this: *const SimpleTextOutput,
        /* in */ mode_number: Word,
    ) -> Status,
    set_attribute: extern "win64" fn(
        /* in */ this: *const SimpleTextOutput,
        /* in */ attribute: Word,
    ) -> Status,

    clear_screen: extern "win64" fn(/* in */ this: *const SimpleTextOutput) -> Status,
    set_cursor_position: extern "win64" fn(
        /* in */ this: *const SimpleTextOutput,
        /* in */ column: Word,
        /* in */ row: Word,
    ) -> Status,
    enable_cursor:
        extern "win64" fn(/* in */ this: *const SimpleTextOutput, /* in */ enable: Bool) -> Status,

    mode: Pointer<SimpleTextOutputMode>,
}

impl SimpleTextOutput {
    pub fn reset(&self, extended_verification: bool) -> Result<(), Status> {
        let reset = (*self).reset;
        let status = reset(self, if extended_verification { 1 } else { 0 });
        status.check(())
    }
    pub fn output_string(&self, string: &[Char16]) -> Result<(), Status> {
        let output_string = (*self).output_string;
        let c_str = string.as_ptr();
        let status = output_string(self, c_str);
        status.check(())
    }
    pub fn test_string(&self, string: &[Char16]) -> Result<(), Status> {
        let test_string = (*self).test_string;
        let c_str = string.as_ptr();
        let status = test_string(self, c_str);
        status.check(())
    }

    pub fn query_mode(&self, mode_number: Word) -> Result<(Word, Word), Status> {
        let query_mode = (*self).query_mode;
        let mut columns = 0;
        let mut rows = 0;
        let status = query_mode(self, mode_number, &mut columns, &mut rows);
        status.check((columns, rows))
    }
    pub fn set_mode(&self, mode_number: Word) -> Result<(), Status> {
        let set_mode = (*self).set_mode;
        let status = set_mode(self, mode_number);
        status.check(())
    }
    pub fn set_attribute(&self, attribute: Word) -> Result<(), Status> {
        let set_attribute = (*self).set_attribute;
        let status = set_attribute(self, attribute);
        status.check(())
    }

    pub fn clear_screen(&self) -> Result<(), Status> {
        let clear_screen = (*self).clear_screen;
        let status = clear_screen(self);
        status.check(())
    }
    pub fn set_cursor_position(&self, column: Word, row: Word) -> Result<(), Status> {
        let set_cursor_position = (*self).set_cursor_position;
        let status = set_cursor_position(self, column, row);
        status.check(())
    }
    pub fn enable_cursor(&self, enable: bool) -> Result<(), Status> {
        let enable_cursor = (*self).enable_cursor;
        let status = enable_cursor(self, if enable { 1 } else { 0 });
        status.check(())
    }

    pub fn get_mode(&self) -> &SimpleTextOutputMode {
        self.mode.as_ref()
    }

    pub fn get_mut_mode(&mut self) -> &mut SimpleTextOutputMode {
        self.mode.as_mut()
    }
}

impl HasGuid for SimpleTextOutput {
    const GUID: Guid = Guid(
        0x387477c2,
        0x69c7,
        0x11d2,
        [0x8e, 0x39, 0x0, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}
