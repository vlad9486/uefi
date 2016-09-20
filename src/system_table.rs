use ::common::Header;
use ::common::Handle;
use ::common::Char16;
use ::common::Uint;
use ::common::Interface;

use ::interfaces::simple_input::I as SimpleInput;
use ::interfaces::simple_text_output::I as SimpleTextOutput;

use ::runtime_services::RuntimeServices;
use ::boot_services::BootServices;
use ::configuration_table::ConfigurationTable;

use core::slice::from_raw_parts;

use ::tools::utf16_to_utf8;
use ::tools::create_utf16;

pub const SIGNATURE: u64 = 0x5453595320494249;

#[derive(Copy, Clone)]
pub struct SystemTable {
    header: Header,
    firmware_vendor: *const Char16,
    firmware_revision: u32,

    stdin_handle: Handle,
    stdin: *const SimpleInput,
    stdout_handle: Handle,
    stdout: *const SimpleTextOutput,
    stderr_handle: Handle,
    stderr: *const SimpleTextOutput,

    runtime_services: *const RuntimeServices,
    boot_services: *const BootServices,

    number_of_table_entries: Uint,
    configuration_table: *const ConfigurationTable
}

impl SystemTable {
    pub fn get_header(&self) -> Header {
        self.header
    }
    pub fn get_firmware_vendor(&self) -> &'static str {
        utf16_to_utf8(create_utf16(self.firmware_vendor))
    }
    pub fn get_firmware_revision(&self) -> u32 {
        self.firmware_revision
    }

    pub fn get_stdin(&self) -> Interface<SimpleInput> {
        Interface {
            handle: self.stdin_handle,
            implementation: unsafe {
                &*self.stdin
            }
        }
    }
    pub fn get_stdout(&self) -> Interface<SimpleTextOutput> {
        Interface {
            handle: self.stdout_handle,
            implementation: unsafe {
                &*self.stdout
            }
        }
    }
    pub fn get_stderr(&self) -> Interface<SimpleTextOutput> {
        Interface {
            handle: self.stderr_handle,
            implementation: unsafe {
                &*self.stderr
            }
        }
    }

    pub fn get_runtime_services(&self) -> &'static RuntimeServices {
        unsafe { &*self.runtime_services }
    }
    pub fn get_boot_services(&self) -> &'static BootServices {
        unsafe { &*self.boot_services }
    }

    pub fn get_configuration_tables(&self) -> &[ConfigurationTable] {
        unsafe {
            from_raw_parts(self.configuration_table, self.number_of_table_entries as usize)
        }
    }
}
