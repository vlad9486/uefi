use ::common::Header;

pub const SIGNATURE: u64 = 0x56524553544e5552;

pub struct RuntimeServices {
    header: Header,

    get_time: extern "win64" fn () -> (),
    set_time: extern "win64" fn () -> (),
    get_wakeup_time: extern "win64" fn () -> (),
    set_wakeup_time: extern "win64" fn () -> (),

    set_virtual_address_map: extern "win64" fn () -> (),
    convert_pointer: extern "win64" fn () -> (),

    get_variable: extern "win64" fn () -> (),
    get_next_variable_name: extern "win64" fn () -> (),
    set_variable: extern "win64" fn () -> (),

    get_next_high_monotonic_count: extern "win64" fn () -> (),
    reset_system: extern "win64" fn () -> (),

    update_capsule: extern "win64" fn () -> (),
    query_capsule_capabilities: extern "win64" fn () -> (),
    query_variable_info: extern "win64" fn () -> ()
}

impl RuntimeServices {
    pub fn get_header(&self) -> Header {
        self.header
    }
}
