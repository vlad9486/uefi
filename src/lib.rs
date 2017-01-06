#![feature(lang_items)]
#![no_std]
#![allow(dead_code)]
#[macro_use]

extern crate bitflags;

pub mod common;
pub mod interfaces;
pub mod configuration_table;

pub mod system_table;
pub mod runtime_services;
pub mod boot_services;

mod tools;
