use common::Handle;
use common::Header;
use common::Char16;

use configuration_table::ConfigurationTable;

use array::Array;
use array::Pointer;

use boot_services::BootServices;
use runtime_services::RuntimeServices;

use interface::simple_input::SimpleInput;
use interface::simple_text_output::SimpleTextOutput;

#[repr(C)]
pub struct SystemTable<'a> {
    header: Header,
    firmware_vendor: Pointer<Char16>,
    firmware_revision: u32,

    stdin_handle: Handle,
    stdin: &'a SimpleInput,
    stdout_handle: Handle,
    stdout: &'a mut SimpleTextOutput,
    stderr_handle: Handle,
    stderr: &'a mut SimpleTextOutput,

    runtime_services: &'a RuntimeServices,
    boot_services: &'a BootServices,

    configuration_tables: Array<ConfigurationTable>,
}

const SIGNATURE: u64 = 0x5453595320494249;

impl<'a> SystemTable<'a> {
    pub fn check(&self) {
        assert_eq!(self.header.signature, SIGNATURE)
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn firmware_vendor(&self) -> &[Char16] {
        unimplemented!()
    }

    pub fn firmware_revision(&self) -> u32 {
        self.firmware_revision
    }

    pub fn stdin(&self) -> &SimpleInput {
        self.stdin
    }

    pub fn stdout(&mut self) -> &mut SimpleTextOutput {
        self.stdout
    }

    pub fn stderr(&mut self) -> &mut SimpleTextOutput {
        self.stderr
    }

    pub fn runtime_services(&self) -> &RuntimeServices {
        self.runtime_services
    }

    pub fn boot_services(&self) -> &BootServices {
        self.boot_services
    }

    pub fn configuration_tables(&self) -> &Array<ConfigurationTable> {
        &self.configuration_tables
    }
}
