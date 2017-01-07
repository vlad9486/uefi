pub mod simple_input;
pub mod simple_text_output;
pub mod simple_file_system;
pub mod file;

use ::common::Status;
use ::common::Guid;
use core::result::Result;
use core::marker::PhantomData;

pub type EfiResult<T> = Result<T, Status>;

pub trait ProtocolImplementation {
    fn get_guid() -> (Guid, PhantomData<Self>);
}
