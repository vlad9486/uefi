pub type Word = usize;
pub type Char16 = u16;
pub type Bool = u8;

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Handle {
    raw: Word,
}

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Event {
    raw: Word,
}

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Registration {
    raw: Word,
}

impl Registration {
    pub const NULL: Self = Registration { raw: 0 };
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status {
    pub raw: Word,
}

impl Status {
    pub const NULL: Self = Status { raw: 0 };
}

impl Status {
    pub fn check<T>(self, value: T) -> Result<T, Self> {
        if self.raw == 0 {
            Ok(value)
        } else {
            Err(self)
        }
    }

    pub fn check_map<T, F>(self, op: F) -> Result<T, Self>
    where
        F: FnOnce() -> T,
    {
        if self.raw == 0 {
            Ok(op())
        } else {
            Err(self)
        }
    }

    pub fn check_flat_map<T, F>(self, op: F) -> Result<T, Self>
    where
        F: FnOnce() -> Result<T, Self>,
    {
        if self.raw == 0 {
            op()
        } else {
            Err(self)
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Guid(pub u32, pub u16, pub u16, pub [u8; 8]);

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Header {
    pub signature: u64,
    pub revision: u32,
    pub size: u32,
    pub crc32: u32,
    _reserved: u32,
}

pub const PAGE_SIZE: Word = 0x1000;

pub trait HasGuid {
    const GUID: Guid;
}

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Address {
    // maybe usize
    raw: u64,
}

impl Address {
    pub const NULL: Self = Address { raw: 0 };

    pub unsafe fn cast<T>(self) -> *mut T
    where
        T: Sized,
    {
        self.raw as _
    }

    pub unsafe fn from_raw<T>(raw: *mut T) -> Self
    where
        T: Sized
    {
        Address {
            raw: raw as _,
        }
    }
}
