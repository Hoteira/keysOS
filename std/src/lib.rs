#![no_std]

pub mod io;
pub mod memory;
pub mod os;
pub mod graphics;
pub mod sync;

pub use crate::io::serial::_print;

extern crate alloc;