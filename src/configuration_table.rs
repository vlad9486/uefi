use array::Pointer;
use array::Array;
use common::Guid;
use common::HasGuid;

#[repr(C)]
pub struct ConfigurationTable {
    // Guid as RTTI allows to cast into appropriate pointer using Pointer::cast::<U>
    vendor_guid: Guid,
    vendor_table: *mut (),
}

pub trait FindTable {
    fn table<T>(&self) -> Option<Pointer<T>>
    where
        T: HasGuid;
}

impl FindTable for Array<ConfigurationTable> {
    fn table<T>(&self) -> Option<Pointer<T>>
    where
        T: HasGuid,
    {
        (0..self.length())
            .into_iter()
            .find(|&index| self[index].vendor_guid == T::GUID)
            .map(|index| unsafe { Pointer::from_raw(self[index].vendor_table as _) })
    }
}

pub struct MSPTable {
    // TODO:
}

impl HasGuid for MSPTable {
    const GUID: Guid = Guid(
        0xeb9d2d2f,
        0x2d88,
        0x11d3,
        [0x9a, 0x16, 0x0, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
    );
}

pub struct ACPITable {
    // TODO:
}

impl HasGuid for ACPITable {
    const GUID: Guid = Guid(
        0xeb9d2d30,
        0x2d88,
        0x11d3,
        [0x9a, 0x16, 0x0, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
    );
}

pub struct ACPI20Table {
    // TODO:
}

impl HasGuid for ACPI20Table {
    const GUID: Guid = Guid(
        0x8868e871,
        0xe4f1,
        0x11d3,
        [0xbc, 0x22, 0x0, 0x80, 0xc7, 0x3c, 0x88, 0x81],
    );
}

pub struct SMBIOSTable {
    // TODO:
}

impl HasGuid for SMBIOSTable {
    const GUID: Guid = Guid(
        0xeb9d2d31,
        0x2d88,
        0x11d3,
        [0x9a, 0x16, 0x0, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
    );
}

pub struct SALTable {
    // TODO:
}

impl HasGuid for SALTable {
    const GUID: Guid = Guid(
        0xeb9d2d32,
        0x2d88,
        0x11d3,
        [0x9a, 0x16, 0x0, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
    );
}
