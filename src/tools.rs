use ::common::Char16;
use ::common::Status;
use ::interfaces::EfiResult;

use core::slice::from_raw_parts;

const MAX_STRING_LENGTH: usize = 1024;

pub fn create_utf16(raw: *const Char16) -> &'static [Char16] {

    let mut length = 0;
    while unsafe { *raw.offset(length) } != 0 {
        length += 1;
    }

    unsafe {
        from_raw_parts(raw, length as usize)
    }
}

// IMPLEMENT ME
pub fn utf16_to_utf8(_: &[Char16]) -> &'static str {
    "unimplemented"
}

// IMPLEMENT ME
pub fn utf8_to_utf16(_: &str) -> &'static [Char16] {
    &[]
}

pub fn result<T>(status: Status, value: T) -> EfiResult<T> {
    if status == 0 {
        Ok(value)
    } else {
        Err(status)
    }
}
