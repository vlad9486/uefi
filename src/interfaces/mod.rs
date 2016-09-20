pub mod simple_input;
pub mod simple_text_output;

use ::common::Status;
use core::result::Result;

pub type EfiResult<T> = Result<T, Status>;
