
// FIXME
pub type Uint = u64;
pub type Char16 = u16;
pub type Bool = u8;

pub type Status = Uint;
pub type Handle = *const ();
pub type Event = *const ();
pub type Registration = *const ();

#[derive(Copy, Clone)]
pub struct Guid(pub u32, pub u16, pub u16, pub [u8; 8]);

#[derive(Copy, Clone)]
pub struct Header {
    pub signature: u64,
    pub revision: u32,
    pub size: u32,
    pub crc32: u32,
    pub reserved: u32
}

pub struct Interface<T> where T: 'static {
    pub handle: Handle,
    pub implementation: &'static T
}
