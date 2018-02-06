use common::Guid;
use common::Status;
use common::HasGuid;

use super::file::File;

#[repr(C)]
pub struct SimpleFileSystem {
    revision: u64,

    open_volume: extern "win64" fn(
        /* in */ this: *const SimpleFileSystem,
        /* out */ root: *mut *const File,
    ) -> Status,
}

const REVISION1: u32 = 0x00010000;

impl SimpleFileSystem {
    pub fn open_volume(&self) -> Result<&File, Status> {
        assert!(self.revision as u32 >= REVISION1);
        let open_volume = self.open_volume;
        let mut file: *const File = 0 as _;
        open_volume(self, &mut file).check(unsafe { &*file })
    }
}

impl HasGuid for SimpleFileSystem {
    const GUID: Guid = Guid(
        0x0964e5b22,
        0x6459,
        0x11d2,
        [0x8e, 0x39, 0x0, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}
