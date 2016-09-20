use ::common::Header;

pub const SIGNATURE: u64 = 0x56524553544e5552;

#[derive(Copy, Clone)]
pub struct RuntimeServices {
    header: Header,

}

impl RuntimeServices {
    pub fn get_header(&self) -> Header {
        self.header
    }
}
