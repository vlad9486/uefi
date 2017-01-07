use ::common::Header;
use ::common::Uint;
use ::common::Status;
use ::common::Handle;
use ::common::Guid;
use ::common::Registration;

use ::interfaces::EfiResult;
use ::interfaces::ProtocolImplementation;

use ::core::slice;

pub type Tpl = Uint;

pub const SIGNATURE: u64 = 0x56524553544f4f42;

pub struct BootServices {
    header: Header,

    raise_tpl: extern "win64" fn (
        /* in */ new_tpl: Tpl
    ) -> Tpl,
    restore_tpl: extern "win64" fn (
        /* in */ old_tpl: Tpl
    ) -> (),

    allocate_pages: extern "win64" fn () -> (),
    free_pages: extern "win64" fn () -> (),
    get_memory_map: extern "win64" fn () -> (),
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
    exit_boot_services: extern "win64" fn () -> (),

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

pub enum SearchKey {
    AllHandles,
    ByRegisterNotify(Registration),
    ByProtocol(Guid)
}

impl BootServices {
    pub fn get_header(&self) -> Header {
        self.header
    }

    pub fn handle_protocol<T: ProtocolImplementation>(&self, handle: Handle) -> EfiResult<&'static T> {
        let handle_protocol = self.handle_protocol;
        let mut implementation: *const () = 0 as _;
        let (guid, _) = T::get_guid();
        let status = handle_protocol(handle, &guid, &mut implementation);
        if status == 0 {
            let implementation = unsafe {
                &*(implementation as *const T)
            };
            Ok(implementation)
        } else {
            Err(status)
        }
    }

    pub fn locate_handle_buffer(&self, search_key: SearchKey) -> EfiResult<&'static [Handle]> {
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
}
