use ::common::Header;
use ::common::Uint;

pub type Tpl = Uint;

pub const SIGNATURE: u64 = 0x56524553544f4f42;

#[derive(Copy, Clone)]
pub struct BootServices {
    header: Header,

    raise_tpl: extern fn (
        /* in */ new_tpl: Tpl
    ) -> Tpl,
    restore_tpl: extern fn (
        /* in */ old_tpl: Tpl
    ) -> (),

    allocate_pages: extern fn () -> (),
    free_pages: extern fn () -> (),
    get_memory_map: extern fn () -> (),
    allocate_pool: extern fn () -> (),
    free_pool: extern fn () -> (),

    create_event: extern fn () -> (),
    set_timer: extern fn () -> (),
    wait_for_event: extern fn () -> (),
    signal_event: extern fn () -> (),
    close_event: extern fn () -> (),
    check_event: extern fn () -> (),

    install_protocol_interface: extern fn () -> (),
    reinstall_protocol_interface: extern fn () -> (),
    uninstall_protocol_interface: extern fn () -> (),
    handle_protocol: extern fn () -> (),
    pchandle_protocol: extern fn () -> (),
    register_protocol_notify: extern fn () -> (),
    locate_handle: extern fn () -> (),
    locate_device_path: extern fn () -> (),
    install_configuration_table: extern fn () -> (),

    load_image: extern fn () -> (),
    start_image: extern fn () -> (),
    exit: extern fn () -> (),
    unload_image: extern fn () -> (),
    exit_boot_services: extern fn () -> (),

    get_next_monotonic_count: extern fn () -> (),
    stall: extern fn () -> (),
    set_watchdog_timer: extern fn () -> (),

    connect_controller: extern fn () -> (),
    disconnect_controller: extern fn () -> (),

    open_protocol: extern fn () -> (),
    close_protocol: extern fn () -> (),
    open_protocol_information: extern fn () -> (),

    protocols_per_handle: extern fn () -> (),
    locate_handle_buffer: extern fn () -> (),
    locate_protocol: extern fn () -> (),
    install_multiple_protocol_interfaces: extern fn () -> (),
    uninstall_multiple_protocol_interfaces: extern fn () -> (),

    calculate_crc32: extern fn () -> (),

    copy_mem: extern fn () -> (),
    set_mem: extern fn () -> (),
    create_event_ex: extern fn () -> ()
}

impl BootServices {
    pub fn get_header(&self) -> Header {
        self.header
    }
}
