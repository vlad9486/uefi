// FIXME
pub type Uint = u64;
pub type Char16 = u16;
pub type Bool = u8;

pub type Status = Uint;
pub type Handle = *const ();
pub type Event = *const ();
pub type Registration = *const ();

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Guid(pub u32, pub u16, pub u16, pub [u8; 8]);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Header {
    pub signature: u64,
    pub revision: u32,
    pub size: u32,
    pub crc32: u32,
    pub reserved: u32
}

pub const PAGE_SIZE: Uint = 0x1000;

#[repr(C)]
#[derive(Default, Clone)]
pub struct Time {
    year: u16, // 1900 – 9999
    month: u8, // 1 – 12
    day: u8, // 1 – 31
    hour: u8, // 0 – 23
    minute: u8, // 0 – 59
    second: u8, // 0 – 59
    pad1: u8,
    nanosecond: u32, // 0 – 999,999,999
    time_zone: i16, // -1440 to 1440 or 2047
    day_light: u8,
    pad2: u8
}
