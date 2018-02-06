use common::Header;
use common::Word;
use common::Status;
use common::Handle;
use common::Guid;
use common::Registration;
use common::Address;
use common::PAGE_SIZE;
use common::HasGuid;

use array::ArrayStride;

use core::ptr;
use core::slice;

#[repr(C)]
pub struct BootServices {
    header: Header,

    raise_tpl: extern "win64" fn(/* in */ new_tpl: Tpl) -> Tpl,
    restore_tpl: extern "win64" fn(/* in */ old_tpl: Tpl) -> (),

    allocate_pages: extern "win64" fn(
        /* in */ allocate_type: Word,
        /* in */ memory_type: Word,
        /* in */ pages: Word,
        /* in out */ physical_address: *mut Address,
    ) -> Status,
    free_pages:
        extern "win64" fn(/* in */ physical_address: Address, /* in */ pages: Word) -> Status,
    get_memory_map: extern "win64" fn(
        /* in out */ memory_map_size: *mut Word,
        /* in out */ memory_map: *mut MemoryDescriptor,
        /* out */ map_key: *mut Word,
        /* out */ descriptor_size: *mut Word,
        /* out */ descriptor_version: *mut u32,
    ) -> Status,
    allocate_pool: extern "win64" fn() -> (),
    free_pool: extern "win64" fn() -> (),

    create_event: extern "win64" fn() -> (),
    set_timer: extern "win64" fn() -> (),
    wait_for_event: extern "win64" fn() -> (),
    signal_event: extern "win64" fn() -> (),
    close_event: extern "win64" fn() -> (),
    check_event: extern "win64" fn() -> (),

    install_protocol_interface: extern "win64" fn() -> (),
    reinstall_protocol_interface: extern "win64" fn() -> (),
    uninstall_protocol_interface: extern "win64" fn() -> (),
    handle_protocol: extern "win64" fn(
        /* in */ handle: Handle,
        /* in */ protocol: *const Guid,
        /* out */ interface: *mut *const (),
    ) -> Status,
    pchandle_protocol: extern "win64" fn() -> (),
    register_protocol_notify: extern "win64" fn() -> (),
    locate_handle: extern "win64" fn() -> (),
    locate_device_path: extern "win64" fn() -> (),
    install_configuration_table: extern "win64" fn() -> (),

    load_image: extern "win64" fn() -> (),
    start_image: extern "win64" fn() -> (),
    exit: extern "win64" fn() -> (),
    unload_image: extern "win64" fn() -> (),
    exit_boot_services:
        extern "win64" fn(/* in */ handle: Handle, /* in */ map_key: Word) -> Status,

    get_next_monotonic_count: extern "win64" fn() -> (),
    stall: extern "win64" fn() -> (),
    set_watchdog_timer: extern "win64" fn() -> (),

    connect_controller: extern "win64" fn() -> (),
    disconnect_controller: extern "win64" fn() -> (),

    open_protocol: extern "win64" fn() -> (),
    close_protocol: extern "win64" fn() -> (),
    open_protocol_information: extern "win64" fn() -> (),

    protocols_per_handle: extern "win64" fn() -> (),
    locate_handle_buffer: extern "win64" fn(
        /* in */ search_type: u32,
        /* in */ protocol: *const Guid,
        /* in */ search_key: Registration,
        /* in out */ no_handles: *mut Word,
        /* in out */ buffer: *mut *const Handle,
    ) -> Status,
    locate_protocol: extern "win64" fn() -> (),
    install_multiple_protocol_interfaces: extern "win64" fn() -> (),
    uninstall_multiple_protocol_interfaces: extern "win64" fn() -> (),

    calculate_crc32: extern "win64" fn() -> (),

    copy_mem: extern "win64" fn() -> (),
    set_mem: extern "win64" fn() -> (),
    create_event_ex: extern "win64" fn() -> (),
}

type Tpl = Word;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryDescriptor {
    // WARNING: check it
    memory_type: MemoryType,
    physical_start: Address,
    virtual_start: Address,
    number_of_pages: u64,
    attribute: MemoryAttributes,
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

pub struct MemoryDescriptorArray {
    pub array: ArrayStride<MemoryDescriptor>,
    pub key: Word,
    pub descriptor_version: u32,
}

pub enum AllocateType {
    AllocateAnyPages,
    AllocateMaxAddress(Address),
    AllocateAddress(Address),
}

impl AllocateType {
    fn into_raw(self) -> (Word, Address) {
        match self {
            AllocateType::AllocateAnyPages => (0, Address::NULL),
            AllocateType::AllocateMaxAddress(t) => (1, t),
            AllocateType::AllocateAddress(t) => (2, t),
        }
    }
}

#[repr(u32)]
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
    PersistentMemory,
}

pub enum SearchKey {
    AllHandles,
    ByRegisterNotify(Registration),
    ByProtocol(Guid),
}

const SIGNATURE: u64 = 0x56524553544f4f42;

impl BootServices {
    pub fn check(&self) {
        assert_eq!(self.header.signature, SIGNATURE)
    }

    pub fn get_header(&self) -> Header {
        self.header
    }

    pub fn allocate_pages(
        &self,
        allocate_type: AllocateType,
        memory_type: MemoryType,
        pages: Word,
    ) -> Result<Address, Status> {
        let allocate_pages = self.allocate_pages;
        let (allocate_type, mut address) = allocate_type.into_raw();
        allocate_pages(allocate_type, memory_type as _, pages, &mut address).check(address)
    }

    pub fn free_pages(&self, address: Address, pages: Word) -> Result<(), Status> {
        let free_pages = self.free_pages;
        free_pages(address, pages).check(())
    }

    pub fn get_memory_map(&self) -> Result<MemoryDescriptorArray, Status> {
        let get_memory_map = self.get_memory_map;
        let mut map_size = 0 as _;
        let mut key = 0 as _;
        let mut stride = 0 as _;
        let mut version = 0 as _;

        get_memory_map(
            &mut map_size,
            0 as *mut MemoryDescriptor,
            &mut key,
            &mut stride,
            &mut version,
        ).check_flat_map(|| {
            let length = map_size / stride;
            assert_eq!(map_size % stride, 0);
            let pages = map_size / PAGE_SIZE + 1;

            let address = self.allocate_pages(
                AllocateType::AllocateAnyPages,
                MemoryType::LoaderData,
                pages,
            )?;
            let array = unsafe { ArrayStride::from_raw(address.cast(), length, stride) };

            get_memory_map(
                &mut map_size,
                unsafe { address.cast() },
                &mut key,
                &mut stride,
                &mut version,
            ).check(MemoryDescriptorArray {
                array: array,
                key: key,
                descriptor_version: version,
            })
        })
    }

    pub fn handle_protocol<T: HasGuid>(&self, handle: Handle) -> Result<&T, Status> {
        let handle_protocol = self.handle_protocol;
        let mut implementation: *const () = ptr::null();
        let guid = T::GUID;
        handle_protocol(handle, &guid, &mut implementation)
            .check(unsafe { &*(implementation as *const T) })
    }

    pub fn locate_handle_buffer(&self, search_key: SearchKey) -> Result<&[Handle], Status> {
        let locate_handle_buffer = self.locate_handle_buffer;
        let mut no_handles: Word = 0;
        let mut buffer: *const Handle = 0 as _;
        match search_key {
            SearchKey::AllHandles => locate_handle_buffer(
                0,
                ptr::null(),
                Registration::NULL,
                &mut no_handles,
                &mut buffer,
            ),
            SearchKey::ByRegisterNotify(registration) => {
                locate_handle_buffer(1, ptr::null(), registration, &mut no_handles, &mut buffer)
            }
            SearchKey::ByProtocol(guid) => {
                locate_handle_buffer(2, &guid, Registration::NULL, &mut no_handles, &mut buffer)
            }
        }.check(unsafe { slice::from_raw_parts(buffer, no_handles as _) })
    }

    pub fn exit_boot_services(&self, handle: Handle, map_key: Word) -> Result<(), Status> {
        let exit_boot_services = self.exit_boot_services;
        exit_boot_services(handle, map_key).check(())
    }
}
