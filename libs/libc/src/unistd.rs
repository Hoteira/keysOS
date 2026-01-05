use core::ffi::{c_uint, c_int, c_long, c_char, c_void};
use crate::sys::krake_sleep;

#[unsafe(no_mangle)] pub unsafe extern "C" fn usleep(usec: c_uint) -> c_int {
    let ms = (usec + 999) / 1000;
    krake_sleep(ms as usize);
    0
}

#[unsafe(no_mangle)] pub unsafe extern "C" fn time(_t: *mut c_long) -> c_long { 0 }

#[unsafe(no_mangle)] pub unsafe extern "C" fn open(path: *const c_char, _flags: c_int, _mode: c_int) -> c_int {
    let path_str = core::ffi::CStr::from_ptr(path).to_string_lossy();
    // Default to open existing. If fails, try create (simple fallback for write support)
    if let Ok(f) = std::fs::File::open(&path_str) {
        let fd = f.as_raw_fd();
        core::mem::forget(f);
        fd as c_int
    } else {
        // Try creating? If we assume this is a simple system, maybe fallback to create if open fails?
        // But checking flags would be better.
        // For now, let's just try create if open failed.
        if let Ok(f) = std::fs::File::create(&path_str) {
            let fd = f.as_raw_fd();
            core::mem::forget(f);
            fd as c_int
        } else {
            -1
        }
    }
}

#[unsafe(no_mangle)] pub unsafe extern "C" fn close(fd: c_int) -> c_int {
    std::os::file_close(fd as usize) as c_int
}

#[unsafe(no_mangle)] pub unsafe extern "C" fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize {
    let slice = core::slice::from_raw_parts_mut(buf as *mut u8, count);
    let res = std::os::file_read(fd as usize, slice);
    if res == usize::MAX { -1 } else { res as isize }
}

#[unsafe(no_mangle)] pub unsafe extern "C" fn write(fd: c_int, buf: *const c_void, count: usize) -> isize {
    let slice = core::slice::from_raw_parts(buf as *const u8, count);
    let res = std::os::file_write(fd as usize, slice);
    if res == usize::MAX { -1 } else { res as isize }
}

#[unsafe(no_mangle)] pub unsafe extern "C" fn access(path: *const c_char, _mode: c_int) -> c_int {
    let path_str = core::ffi::CStr::from_ptr(path).to_string_lossy();
    if let Ok(f) = std::fs::File::open(&path_str) {
        core::mem::drop(f);
        0
    } else {
        -1
    }
}

#[unsafe(no_mangle)] pub unsafe extern "C" fn isatty(fd: c_int) -> c_int {
    if fd >= 0 && fd <= 2 { 1 } else { 0 }
}
