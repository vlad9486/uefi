use ::common::Header;
use ::common::Uint;
use ::common::Status;
use ::common::Handle;
use ::common::Guid;
use ::common::Registration;

use ::interfaces::EfiResult;
use ::interfaces::ProtocolImplementation;

use ::tools::EfiObject;
use ::dynamic_array::DynamicArrayStride;
use ::dynamic_array::Array;

use core::slice;

pub const SIGNATURE: u64 = 0x56524553544f4f42;

#[repr(C)]
pub struct BootServices {
    header: Header,

    raise_tpl: extern "win64" fn (
        /* in */ new_tpl: Tpl
    ) -> Tpl,
    restore_tpl: extern "win64" fn (
        /* in */ old_tpl: Tpl
    ) -> (),

    allocate_pages: extern "win64" fn (
        /* in */ allocate_type: Uint,
        /* in */ memory_type: Uint,
        /* in */ pages: Uint,
        /* in out */ physical_address: *mut u64
    ) -> Status,
    free_pages: extern "win64" fn (
        /* in */ physical_address: u64,
        /* in */ pages: Uint
    ) -> Status,
    get_memory_map: extern "win64" fn (
        /* in out */ memory_map_size: *mut Uint,
        /* in out */ memory_map: *mut MemoryDescriptorRaw,
        /* out */ map_key: *mut Uint,
        /* out */ descriptor_size: *mut Uint,
        /* out */ descriptor_version: *mut u32
    ) -> Status,
    allocate_pool: extern "win64" fn () -> (),
    free_pool: extern "win64" fn () -> (),

    create_event: extern "win64" fn () -> (),
    set_timer: extern "win64" fn () -> (),
    wait_for_event: extern "win64" fn () -> (),
    signal_event: extern "win64" fn () -> (),
    close_event: extern "win64" fn () -> (),
    check_event: extern "win64" fn () -> (),

    install_protocol_interface: extern "win64" fn () -> (),
    reinstall_protocol_interface: extern "win64" fn () -> (),
    uninstall_protocol_interface: extern "win64" fn () -> (),
    handle_protocol: extern "win64" fn (
        /* in */ handle: Handle,
        /* in */ protocol: *const Guid,
        /* out */ interface: *mut *const ()
    ) -> Status,
    pchandle_protocol: extern "win64" fn () -> (),
    register_protocol_notify: extern "win64" fn () -> (),
    locate_handle: extern "win64" fn () -> (),
    locate_device_path: extern "win64" fn () -> (),
    install_configuration_table: extern "win64" fn () -> (),

    load_image: extern "win64" fn () -> (),
    start_image: extern "win64" fn () -> (),
    exit: extern "win64" fn () -> (),
    unload_image: extern "win64" fn () -> (),
    exit_boot_services: extern "win64" fn (
        /* in */ handle: Handle,
        /* in */ map_key: Uint
    ) -> Status,

    get_next_monotonic_count: extern "win64" fn () -> (),
    stall: extern "win64" fn () -> (),
    set_watchdog_timer: extern "win64" fn () -> (),

    connect_controller: extern "win64" fn () -> (),
    disconnect_controller: extern "win64" fn () -> (),

    open_protocol: extern "win64" fn () -> (),
    close_protocol: extern "win64" fn () -> (),
    open_protocol_information: extern "win64" fn () -> (),

    protocols_per_handle: extern "win64" fn () -> (),
    locate_handle_buffer: extern "win64" fn (
        /* in */ search_type: u32,
        /* in */ protocol: *const Guid,
        /* in */ search_key: Registration,
        /* in out */ no_handles: *mut Uint,
        /* in out */ buffer: *mut *const Handle
    ) -> Status,
    locate_protocol: extern "win64" fn () -> (),
    install_multiple_protocol_interfaces: extern "win64" fn () -> (),
    uninstall_multiple_protocol_interfaces: extern "win64" fn () -> (),

    calculate_crc32: extern "win64" fn () -> (),

    copy_mem: extern "win64" fn () -> (),
    set_mem: extern "win64" fn () -> (),
    create_event_ex: extern "win64" fn () -> ()
}

pub type Tpl = Uint;

pub enum SearchKey {
    AllHandles,
    ByRegisterNotify(Registration),
    ByProtocol(Guid)
}

pub enum AllocateType {
    AllocateAnyPages,
    AllocateMaxAddress(PhysicalAddress),
    AllocateAddress(PhysicalAddress)
}

impl AllocateType {
    fn into_raw(self) -> (Uint, PhysicalAddress) {
        match self {
            AllocateType::AllocateAnyPages => (0, PhysicalAddress(0)),
            AllocateType::AllocateMaxAddress(t) => (1, t),
            AllocateType::AllocateAddress(t) => (2, t)
        }
    }
}

#[derive(Copy, Clone)]
pub enum MemoryType {
    ReservedMemoryType,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    PersistentMemory
}

#[derive(Copy, Clone)]
pub struct PhysicalAddress(pub u64);

#[derive(Copy, Clone)]
pub struct VirtualAddress(pub u64);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryDescriptorRaw {
    memory_type: u32,
    physical_start: u64,
    virtual_start: u64,
    number_of_pages: u64,
    attribute: u64
}

pub struct MemoryDescriptor {
    pub memory_type: MemoryType,
    pub physical_start: PhysicalAddress,
    pub virtual_start: VirtualAddress,
    pub number_of_pages: u64,
    pub attribute: MemoryAttributes
}

pub struct MemoryDescriptorArray<'a> {
    pub array: DynamicArrayStride<'a, MemoryDescriptorRaw>,
    pub key: Uint,
    pub descriptor_version: u32
}

bitflags! {
    pub struct MemoryAttributes: u64 {
        const MEMORY_UC            = 1 << 0;
        const MEMORY_WC            = 1 << 1;
        const MEMORY_WT            = 1 << 2;
        const MEMORY_WB            = 1 << 3;
        const MEMORY_UCE           = 1 << 4;
        const MEMORY_WP            = 1 << 12;
        const MEMORY_RP            = 1 << 13;
        const MEMORY_XP            = 1 << 14;
        const MEMORY_NV            = 1 << 15;
        const MEMORY_MORE_RELIABLE = 1 << 16;
        const MEMORY_RO            = 1 << 17;
        const MEMORY_RUNTIME       = 1 << 63;
    }
}

impl BootServices {
    pub fn get_header(&self) -> Header {
        self.header
    }

    pub fn allocate_pages(&self, allocate_type: AllocateType, memory_type: MemoryType, pages: Uint) -> EfiResult<PhysicalAddress> {
        let allocate_pages = self.allocate_pages;
        let (allocate_type, physical_address) = allocate_type.into_raw();
        let PhysicalAddress(mut raw) = physical_address;
        let status = allocate_pages(allocate_type, memory_type as Uint, pages, &mut raw);
        if status == 0 {
            Ok(PhysicalAddress(raw))
        } else {
            Err(status)
        }
    }

    pub fn free_pages(&self, physical_address: PhysicalAddress, pages: Uint) -> EfiResult<()> {
        let free_pages = self.free_pages;
        let status = free_pages(physical_address.0, pages);
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    pub fn get_memory_map(&self) -> EfiResult<MemoryDescriptorArray> {
        let get_memory_map = self.get_memory_map;
        let mut map_size = 0 as _;
        let mut key = 0 as _;
        let mut size = 0 as _;
        let mut version = 0 as _;
        let _ = get_memory_map(&mut map_size, 0 as *mut MemoryDescriptorRaw, &mut key, &mut size, &mut version);
        match DynamicArrayStride::new(self, size, (map_size / size)) {
            Ok(array) => {
                let PhysicalAddress(ptr) = array.physical_address();
                let status = get_memory_map(&mut map_size, ptr as *mut MemoryDescriptorRaw, &mut key, &mut size, &mut version);
                if status == 0 {
                    Ok(MemoryDescriptorArray {
                        array: array,
                        key: key,
                        descriptor_version: version
                    })
                } else {
                    Err(status)
                }
            }
            Err(e) => Err(e)
        }
    }

    pub fn handle_protocol<T: ProtocolImplementation + 'static>(&self, handle: Handle) -> EfiResult<EfiObject<T>> {
        let handle_protocol = self.handle_protocol;
        let mut implementation: *const () = 0 as _;
        let (guid, _) = T::get_guid();
        let status = handle_protocol(handle, &guid, &mut implementation);
        if status == 0 {
            let implementation = unsafe {
                &*(implementation as *const T)
            };
            Ok(EfiObject::from_parts(Some(handle), implementation))
        } else {
            Err(status)
        }
    }

    pub fn locate_handle_buffer(&self, search_key: SearchKey) -> EfiResult<&[Handle]> {
        let locate_handle_buffer = self.locate_handle_buffer;
        let mut no_handles: Uint = 0;
        let mut buffer: *const Handle = 0 as _;
        let status = match search_key {
            SearchKey::AllHandles => locate_handle_buffer(0, 0 as _, 0 as _, &mut no_handles, &mut buffer),
            SearchKey::ByRegisterNotify(registration) => locate_handle_buffer(1, 0 as _, registration, &mut no_handles, &mut buffer),
            SearchKey::ByProtocol(guid) => locate_handle_buffer(2, &guid, 0 as _, &mut no_handles, &mut buffer)
        };
        if status == 0 {
            let array = unsafe {
                slice::from_raw_parts(buffer, no_handles as usize)
            };
            Ok(array)
        } else {
            Err(status)
        }
    }

    pub fn exit_boot_services(&self, handle: Handle, map_key: Uint) -> EfiResult<()> {
        let exit_boot_services = self.exit_boot_services;
        let status = exit_boot_services(handle, map_key);
        // Boo
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }
}
