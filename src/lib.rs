#![feature(lang_items)]
#![no_std]
#![allow(dead_code)]

#[macro_use]
extern crate bitflags;

pub mod common;
pub mod interfaces;
pub mod configuration_table;

pub mod system_table;

mod tools;
pub use tools::EfiObject;
