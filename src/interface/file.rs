use common::Char16;
use common::Status;
use common::Guid;
use common::Word;
use common::HasGuid;

use core::ptr;
use core::mem;

#[repr(C)]
pub struct File {
    revision: u64,

    open: extern "win64" fn(
        /* in */ this: *const File,
        /* out */ new_handle: *mut *const File,
        /* in */ file_name: *const Char16,
        /* in */ open_mode: u64,
        /* in */ attributes: u64,
    ) -> Status,
    close: extern "win64" fn(/* in */ this: *const File) -> Status,
    delete: extern "win64" fn(/* in */ this: *const File) -> Status,
    read: extern "win64" fn(
        /* in */ this: *const File,
        /* in out */ buffer_size: *mut Word,
        /* in */ buffer: *mut (),
    ) -> Status,
    write: extern "win64" fn(/* in */ this: *const File) -> Status,
    get_position: extern "win64" fn(/* in */ this: *const File) -> Status,
    set_position: extern "win64" fn(/* in */ this: *const File) -> Status,
    get_info: extern "win64" fn(
        /* in */ this: *const File,
        /* in */ information_type: *const Guid,
        /* in out */ buffer_size: *mut Word,
        /* out */ buffer: *mut (),
    ) -> Status,
    set_info: extern "win64" fn(/* in */ this: *const File) -> Status,
    flush: extern "win64" fn(/* in */ this: *const File) -> Status,

    // Available only if revision == REVISION2
    open_ex: extern "win64" fn(/* in */ this: *const File) -> Status,
    read_ex: extern "win64" fn(/* in */ this: *const File) -> Status,
    write_ex: extern "win64" fn(/* in */ this: *const File) -> Status,
    flush_ex: extern "win64" fn(/* in */ this: *const File) -> Status,
}

const _REVISION1: u32 = 0x00010000;
const _REVISION2: u32 = 0x00020000;

#[derive(Copy, Clone)]
#[repr(u64)]
pub enum OpenMode {
    Read,
    ReadWrite,
    CreateReadWrite,
}

bitflags! {
    pub struct Attributes: u64 {
        const NULL = 0;
        const READ_ONLY = 1 << 0;
        const HIDDEN = 1 << 1;
        const SYSTEM = 1 << 2;
        const RESERVED = 1 << 3;
        const DIRECTORY = 1 << 4;
        const ARCHIVE = 1 << 5;
    }
}

const MAX_FILE_NAME_SIZE: usize = 64;

#[repr(C)]
struct FileInfoRaw {
    size: u64,
    file_size: u64,
    physical_size: u64,
    create_time: Time,
    last_access_time: Time,
    modification_time: Time,
    attributes: Attributes,
    file_name: [Char16; MAX_FILE_NAME_SIZE],
}

impl FileInfoRaw {
    const SIZE: usize = mem::size_of::<Self>();

    fn empty() -> Self {
        FileInfoRaw {
            size: Self::SIZE as _,
            file_size: 0,
            physical_size: 0,
            create_time: Time::default(),
            last_access_time: Time::default(),
            modification_time: Time::default(),
            attributes: Attributes::NULL,
            file_name: [0; MAX_FILE_NAME_SIZE],
        }
    }
}

impl HasGuid for FileInfoRaw {
    const GUID: Guid = Guid(
        0x09576e92,
        0x6d3f,
        0x11d2,
        [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}

pub struct FileInfo {
    pub size: u64,
    pub physical_size: u64,
    pub create_time: Time,
    pub last_access_time: Time,
    pub modification_time: Time,
    pub name: [Char16; MAX_FILE_NAME_SIZE],
}

impl FileInfo {
    pub const MAX_FILE_NAME_SIZE: usize = MAX_FILE_NAME_SIZE;
}

impl From<FileInfoRaw> for FileInfo {
    fn from(raw: FileInfoRaw) -> Self {
        FileInfo {
            size: raw.file_size,
            physical_size: raw.physical_size,
            create_time: raw.create_time.clone(),
            last_access_time: raw.last_access_time.clone(),
            modification_time: raw.modification_time.clone(),
            name: raw.file_name,
        }
    }
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct Time {
    year: u16,  // 1900 – 9999
    month: u8,  // 1 – 12
    day: u8,    // 1 – 31
    hour: u8,   // 0 – 23
    minute: u8, // 0 – 59
    second: u8, // 0 – 59
    _pad1: u8,
    nanosecond: u32, // 0 – 999,999,999
    time_zone: i16,  // -1440 to 1440 or 2047
    day_light: u8,
    _pad2: u8,
}

impl File {
    pub fn open(
        &self,
        file_name: &[Char16],
        open_mode: OpenMode,
        attributes: Attributes,
    ) -> Result<&Self, Status> {
        assert!(self.revision as u32 >= _REVISION1);
        let open = self.open;
        let mut other: *const File = ptr::null_mut();
        let c_str = file_name.as_ptr();
        open(self, &mut other, c_str, open_mode as _, attributes.bits()).check(unsafe { &*other })
    }
    pub fn close(&self) -> Result<(), Status> {
        assert!(self.revision as u32 >= _REVISION1);
        let close = self.close;
        close(self).check(())
    }
    pub fn read(&self, buffer: &mut [u8]) -> Result<Word, Status> {
        assert!(self.revision as u32 >= _REVISION1);
        let read = self.read;
        let mut size = buffer.len() as Word;
        let ptr = buffer.as_mut_ptr();
        read(self, &mut size, ptr as _).check(size)
    }
    pub fn get_file_info(&self) -> Result<FileInfo, Status> {
        assert!(self.revision as u32 >= _REVISION1);
        let get_info = self.get_info;
        let mut file_info = FileInfoRaw::empty();
        let size = FileInfoRaw::SIZE;
        get_info(
            self,
            &FileInfoRaw::GUID,
            &mut (size as _),
            &mut file_info as *mut FileInfoRaw as _,
        ).check(file_info.into())
    }
}
