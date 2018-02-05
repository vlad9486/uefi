#![no_std]
#![feature(nonzero)]

#[macro_use]
extern crate bitflags;

pub mod common;
pub mod interfaces;
pub mod configuration_table;

pub mod system_table;

pub mod dynamic_array;

mod tools;
pub use tools::EfiObject;
