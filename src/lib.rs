#![no_std]
#![feature(try_trait)]

#[macro_use]
extern crate bitflags;

pub mod common;
pub mod array;

pub mod interface;
pub mod configuration_table;

pub mod system_table;
pub mod boot_services;
pub mod boot_services_ex;
pub mod runtime_services;
