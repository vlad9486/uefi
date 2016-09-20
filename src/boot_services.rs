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
}

impl BootServices {
    pub fn get_header(&self) -> Header {
        self.header
    }
}
