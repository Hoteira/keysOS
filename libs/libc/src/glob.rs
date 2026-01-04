use core::ffi::{c_char, c_int, c_void};
use alloc::vec::Vec;
use alloc::string::String;
use crate::string::{strlen, strdup};

#[repr(C)]
pub struct glob_t {
    pub gl_pathc: usize,
    pub gl_pathv: *mut *mut c_char,
    pub gl_offs: usize,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn glob(pattern: *const c_char, _flags: c_int, _errfunc: Option<extern "C" fn(epath: *const c_char, eerrno: c_int) -> c_int>, pglob: *mut glob_t) -> c_int {
    let pat = core::str::from_utf8_unchecked(core::slice::from_raw_parts(pattern as *const u8, strlen(pattern)));
    
    // Minimal glob: if it doesn't contain *, just return the path as is if it exists
    if !pat.contains('*') {
        let mut stats = [0u64; 2];
        let resolved = crate::misc::resolve_path_rust(pattern);
        if std::os::stat(&resolved, &mut stats) == 0 {
            (*pglob).gl_pathc = 1;
            let v = crate::stdlib::malloc(core::mem::size_of::<*mut c_char>() * 2) as *mut *mut c_char;
            *v = strdup(pattern);
            *v.add(1) = core::ptr::null_mut();
            (*pglob).gl_pathv = v;
            return 0;
        }
        return 3; // GLOB_NOMATCH
    }

    // Wildcard support would go here. For now, return NOMATCH
    3
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn globfree(pglob: *mut glob_t) {
    if pglob.is_null() { return; }
    if !(*pglob).gl_pathv.is_null() {
        let mut i = 0;
        while !(*(*pglob).gl_pathv.add(i)).is_null() {
            crate::stdlib::free(*(*pglob).gl_pathv.add(i) as *mut c_void);
            i += 1;
        }
        crate::stdlib::free((*pglob).gl_pathv as *mut c_void);
    }
    (*pglob).gl_pathc = 0;
    (*pglob).gl_pathv = core::ptr::null_mut();
}
