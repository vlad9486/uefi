use ::common::Header;

pub const SIGNATURE: u64 = 0x56524553544e5552;

#[derive(Copy, Clone)]
pub struct RuntimeServices {
    header: Header,

    get_time: extern fn () -> (),
    set_time: extern fn () -> (),
    get_wakeup_time: extern fn () -> (),
    set_wakeup_time: extern fn () -> (),

    set_virtual_address_map: extern fn () -> (),
    convert_pointer: extern fn () -> (),

    get_variable: extern fn () -> (),
    get_next_variable_name: extern fn () -> (),
    set_variable: extern fn () -> (),

    get_next_high_monotonic_count: extern fn () -> (),
    reset_system: extern fn () -> (),

    update_capsule: extern fn () -> (),
    query_capsule_capabilities: extern fn () -> (),
    query_variable_info: extern fn () -> ()
}

impl RuntimeServices {
    pub fn get_header(&self) -> Header {
        self.header
    }
}
