pub mod boot_services;
pub mod runtime_services;

use ::common::Header;
use ::common::Handle;
use ::common::Char16;
use ::common::Uint;

use self::boot_services::BootServices;
use self::runtime_services::RuntimeServices;

use ::interfaces::simple_input::I as SimpleInput;
use ::interfaces::simple_text_output::I as SimpleTextOutput;

use ::configuration_table::ConfigurationTable;

use core::slice::from_raw_parts;

use ::tools::EfiObject;
use ::tools::create_utf16;

pub const SIGNATURE: u64 = 0x5453595320494249;

#[repr(C)]
pub struct SystemTable {
    header: Header,
    firmware_vendor: *const Char16,
    firmware_revision: u32,

    stdin_handle: Handle,
    stdin: &'static SimpleInput,
    stdout_handle: Handle,
    stdout: &'static SimpleTextOutput,
    stderr_handle: Handle,
    stderr: &'static SimpleTextOutput,

    runtime_services: &'static RuntimeServices,
    boot_services: &'static BootServices,

    number_of_table_entries: Uint,
    configuration_table: *const ConfigurationTable
}

impl SystemTable {
    pub fn as_object(&'static self, handle: Handle) -> EfiObject<Self> {
        EfiObject::from_parts(Some(handle), &self)
    }
    pub fn get_header(&self) -> Header {
        self.header
    }
    pub fn get_firmware_vendor(&self) -> &'static [Char16] {
        create_utf16(self.firmware_vendor)
    }
    pub fn get_firmware_revision(&self) -> u32 {
        self.firmware_revision
    }

    pub fn get_stdin(&self) -> EfiObject<SimpleInput> {
        EfiObject::from_parts(Some(self.stdin_handle), self.stdin)
    }
    pub fn get_stdout(&self) -> EfiObject<SimpleTextOutput> {
        EfiObject::from_parts(Some(self.stdout_handle), self.stdout)
    }
    pub fn get_stderr(&self) -> EfiObject<SimpleTextOutput> {
        EfiObject::from_parts(Some(self.stderr_handle), self.stderr)
    }

    pub fn get_runtime_services(&self) -> EfiObject<RuntimeServices> {
        EfiObject::from_parts(None, self.runtime_services)
    }
    pub fn get_boot_services(&self) -> EfiObject<BootServices> {
        EfiObject::from_parts(None, self.boot_services)
    }

    pub fn get_configuration_tables(&self) -> &[ConfigurationTable] {
        unsafe {
            from_raw_parts(self.configuration_table, self.number_of_table_entries as usize)
        }
    }
}
