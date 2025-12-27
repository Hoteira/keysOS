#![feature(c_variadic)]
#![no_std]

#[macro_use]
extern crate std;
// --- ENTRY POINT ---

extern crate alloc;

unsafe extern "C" {
    fn main() -> c_int;
}

// 32MB Static Heap for C programs
static mut HEAP_MEM: [u8; 1024 * 1024 * 32] = [0; 1024 * 1024 * 32];

#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        "mov rdi, rsp",
        "call rust_start",
        "hlt",
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_start(_stack: *const u8) -> ! {
    std::os::print("CRT: rust_start reached\n");

    // 1. Initialize Heap
    std::os::print("CRT: Initializing heap...\n");
    unsafe {
        //std::memory::heap::init_heap((*(&raw mut HEAP_MEM)).as_mut_ptr(), (*(&raw mut HEAP_MEM)).len());
    }

    // 2. Call C main
    std::os::print("CRT: Calling main()...\n");
    let result = main();

    // 3. Exit
    std::os::print("CRT: main() returned, exiting.\n");
    exit(result);
}

use alloc::boxed::Box;
use alloc::string::String;
use core::ffi::{c_void, c_char, c_int, c_long};
use core::alloc::{Layout};


#[repr(C)]
struct Header {
    size: usize,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc(size: usize) -> *mut c_void {
    let total_size = size + core::mem::size_of::<Header>();
    let layout = Layout::from_size_align(total_size, 8).unwrap();
    
    let ptr = alloc::alloc::alloc(layout);
    if ptr.is_null() { return core::ptr::null_mut(); }
    
    let header = ptr as *mut Header;
    (*header).size = size;
    
    ptr.add(core::mem::size_of::<Header>()) as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free(ptr: *mut c_void) {
    if ptr.is_null() { return; }
    
    let real_ptr = (ptr as *mut u8).sub(core::mem::size_of::<Header>());
    let header = real_ptr as *mut Header;
    let size = (*header).size;
    
    let total_size = size + core::mem::size_of::<Header>();
    let layout = Layout::from_size_align(total_size, 8).unwrap();
    
    alloc::alloc::dealloc(real_ptr, layout);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn calloc(nmemb: usize, size: usize) -> *mut c_void {
    let total = nmemb * size;
    let ptr = malloc(total);
    if !ptr.is_null() {
        core::ptr::write_bytes(ptr as *mut u8, 0, total);
    }
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    if ptr.is_null() { return malloc(size); }
    if size == 0 { free(ptr); return core::ptr::null_mut(); }
    
    let new_ptr = malloc(size);
    if new_ptr.is_null() { return core::ptr::null_mut(); }
    
    let real_ptr = (ptr as *mut u8).sub(core::mem::size_of::<Header>());
    let header = real_ptr as *mut Header;
    let old_size = (*header).size;
    
    let copy_size = if size < old_size { size } else { old_size };
    core::ptr::copy_nonoverlapping(ptr as *const u8, new_ptr as *mut u8, copy_size);
    
    free(ptr);
    new_ptr
}

// --- MEMORY OPS ---

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void {
    core::ptr::write_bytes(s as *mut u8, c as u8, n);
    s
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    core::ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, n);
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    core::ptr::copy(src as *const u8, dest as *mut u8, n);
    dest
}

// --- STRING OPS ---

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlen(s: *const c_char) -> usize {
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    len
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int {
    let mut i = 0;
    loop {
        let c1 = *s1.add(i) as u8;
        let c2 = *s2.add(i) as u8;
        if c1 != c2 {
            return (c1 as c_int) - (c2 as c_int);
        }
        if c1 == 0 { return 0; }
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int {
    let mut i = 0;
    while i < n {
        let c1 = *s1.add(i) as u8;
        let c2 = *s2.add(i) as u8;
        if c1 != c2 {
            return (c1 as c_int) - (c2 as c_int);
        }
        if c1 == 0 { return 0; }
        i += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char {
    let mut i = 0;
    loop {
        let c = *src.add(i);
        *dest.add(i) = c;
        if c == 0 { break; }
        i += 1;
    }
    dest
}

// --- IO OPS ---

#[unsafe(no_mangle)]
pub unsafe extern "C" fn putchar(c: c_int) -> c_int {
    let buf = [c as u8];
    // Write to stdout (fd 1) or serial?
    // Let's use std::print which calls syscall 1 (PRINT)
    let s = core::str::from_utf8_unchecked(&buf);
    std::os::print(s);
    c
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn puts(s: *const c_char) -> c_int {
    let len = strlen(s);
    let slice = core::slice::from_raw_parts(s as *const u8, len);
    let str_val = core::str::from_utf8_unchecked(slice);
    std::print!("{}\n", str_val);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn printf(fmt: *const c_char, ...) -> c_int {
    // STUB: Varargs are hard in Rust FFI without crates. 
    // We just print the format string for now to debug.
    let len = strlen(fmt);
    let slice = core::slice::from_raw_parts(fmt as *const u8, len);
    let str_val = String::from_utf8_lossy(slice);
    std::print!("[printf] {}", str_val);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fprintf(_stream: *mut c_void, fmt: *const c_char, ...) -> c_int {
    // Treat all as stdout for now
    let len = strlen(fmt);
    let slice = core::slice::from_raw_parts(fmt as *const u8, len);
    let str_val = String::from_utf8_lossy(slice);
    std::print!("[fprintf] {}", str_val);
    0
}

// --- FILE IO ---
// This is hard. C returns FILE*. We need to return a pointer to something we manage.
// We can use a global table or heap allocate a struct.
// For KrakeOS, `std::fs::File` is a struct wrapping an FD.
// We can Box<std::fs::File> and return the raw pointer.

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fopen(filename: *const c_char, _mode: *const c_char) -> *mut c_void {
    let len = strlen(filename);
    let slice = core::slice::from_raw_parts(filename as *const u8, len);
    let path = core::str::from_utf8_unchecked(slice);
    
    // Convert path to internal format?
    // std::fs::File::open returns Result.
    if let Ok(file) = std::fs::File::open(path) {
        let boxed = Box::new(file);
        Box::into_raw(boxed) as *mut c_void
    } else {
        core::ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fclose(stream: *mut c_void) -> c_int {
    if stream.is_null() { return -1; }
    let _file = Box::from_raw(stream as *mut std::fs::File);
    // Drop handles close
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut c_void) -> usize {
    if stream.is_null() { return 0; }
    let file = &mut *(stream as *mut std::fs::File);
    let total = size * nmemb;
    let buf = core::slice::from_raw_parts_mut(ptr as *mut u8, total);
    
    if let Ok(n) = file.read(buf) {
        n / size // Return count of items
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fwrite(ptr: *const c_void, size: usize, nmemb: usize, stream: *mut c_void) -> usize {
    if stream.is_null() { return 0; }
    let file = &mut *(stream as *mut std::fs::File);
    let total = size * nmemb;
    let buf = core::slice::from_raw_parts(ptr as *const u8, total);
    
    if let Ok(n) = file.write(buf) {
        n / size
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fseek(stream: *mut c_void, offset: c_long, whence: c_int) -> c_int {
    if stream.is_null() { return -1; }
    let file = &mut *(stream as *mut std::fs::File);
    
    // std::fs::File doesn't expose seek yet?
    // Check std::os::file_seek
    let res = std::os::file_seek(file.as_raw_fd(), offset as i64, whence as usize);
    if res == u64::MAX { -1 } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ftell(stream: *mut c_void) -> c_long {
    if stream.is_null() { return -1; }
    let file = &mut *(stream as *mut std::fs::File);
    
    // Seek 0 from current to get pos
    let res = std::os::file_seek(file.as_raw_fd(), 0, 1); // SEEK_CUR = 1
    if res == u64::MAX { -1 } else { res as c_long }
}

// --- SYSTEM ---

#[unsafe(no_mangle)]
pub unsafe extern "C" fn exit(status: c_int) -> ! {
    std::os::exit(status as u64)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn getenv(_name: *const c_char) -> *mut c_char {
    core::ptr::null_mut() // No env vars yet
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn time(_t: *mut c_long) -> c_long {
    // Return seconds?
    // rtc::get_time returns H M S. Not unix timestamp.
    // Fake it or implement conversion. 
    0 
}

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("[USER PANIC] {}", _info);
    loop {}
}
