use core::ffi::{c_uint, c_int, c_long};
use crate::sys::krake_sleep;

#[unsafe(no_mangle)] pub unsafe extern "C" fn usleep(usec: c_uint) -> c_int {
    let ms = (usec + 999) / 1000;
    krake_sleep(ms as usize);
    0
}

#[unsafe(no_mangle)] pub unsafe extern "C" fn time(_t: *mut c_long) -> c_long { 0 }
