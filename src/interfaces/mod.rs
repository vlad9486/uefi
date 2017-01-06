pub mod simple_input;
pub mod simple_text_output;
pub mod simple_file_system;
pub mod file;

use ::common::Status;
use core::result::Result;

pub type EfiResult<T> = Result<T, Status>;
