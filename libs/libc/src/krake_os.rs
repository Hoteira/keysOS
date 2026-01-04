use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct Window {
    pub id: usize,
    pub buffer: usize,
    pub pid: u64,
    pub x: i64,
    pub y: i64,
    pub z: usize,
    pub width: usize,
    pub height: usize,
    pub can_move: u8,
    pub can_resize: u8,
    pub transparent: u8,
    pub treat_as_transparent: u8,
    pub min_width: usize,
    pub min_height: usize,
    pub event_handler: usize,
    pub w_type: c_int,
}

#[repr(C)]
pub struct Event {
    pub etype: u32,
    pub arg1: u32,
    pub arg2: u32,
    pub arg3: u32,
    pub arg4: u32,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn krake_window_create(width: usize, height: usize, transparent: c_int, treat_as_transparent: c_int) -> usize {
    let mut w: Window = core::mem::zeroed();
    w.width = width;
    w.height = height;
    w.transparent = transparent as u8;
    w.treat_as_transparent = treat_as_transparent as u8;
    w.can_move = 1;
    w.can_resize = 1;
    
    // Call SYS_ADD_WINDOW (22)
    std::os::syscall(22, &w as *const Window as u64, 0, 0) as usize
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn krake_window_draw(wid: usize) {
    // We need to find the window struct if we only have the ID, or we need a syscall that takes ID.
    // In our current kernel, SYS_UPDATE_WINDOW (51) takes a Window* and uses its 'id'.
    // If we only have ID, we might need a dummy window struct with just the ID.
    let mut w: Window = core::mem::zeroed();
    w.id = wid;
    std::os::syscall(51, &w as *const Window as u64, 0, 0);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn krake_window_get_buffer(wid: usize) -> *mut c_void {
    // In our current system, the buffer is mapped into user space or we need its address.
    // The Window struct in COMPOSER has a 'buffer' field which is a physical address.
    // However, the userland app should probably get a virtual address.
    // For now, let's assume it's stored in the window struct returned by some init or we have to query it.
    // Actually, userland apps in KrakeOS usually allocate their own buffer and pass it.
    // If they don't, we might need a syscall to get it.
    // Let's return a placeholder for now or implement a syscall if available.
    core::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn krake_get_event(wid: usize, out_event: *mut Event) -> c_int {
    // SYS_GET_EVENTS (52)
    std::os::syscall(52, wid as u64, out_event as u64, 1) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn krake_sleep(ms: usize) {
    std::os::sleep(ms as u64);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn krake_get_time_ms() -> usize {
    std::os::get_system_ticks() as usize
}
