use core::ffi::c_int;

#[unsafe(no_mangle)] pub unsafe extern "C" fn toupper(c: c_int) -> c_int { if c >= b'a' as c_int && c <= b'z' as c_int { c - 32 } else { c } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn tolower(c: c_int) -> c_int { if c >= b'A' as c_int && c <= b'Z' as c_int { c + 32 } else { c } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn isspace(c: c_int) -> c_int { if c == b' ' as c_int || c == b'\t' as c_int || c == b'\n' as c_int || c == b'\r' as c_int || c == 0x0B || c == 0x0C { 1 } else { 0 } }
#[unsafe(no_mangle)] pub unsafe extern "C" fn isdigit(c: c_int) -> c_int { if c >= b'0' as c_int && c <= b'9' as c_int { 1 } else { 0 } }
