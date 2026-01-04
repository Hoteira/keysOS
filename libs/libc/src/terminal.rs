use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct termios {
    pub c_iflag: c_uint,
    pub c_oflag: c_uint,
    pub c_cflag: c_uint,
    pub c_lflag: c_uint,
    pub c_line: u8,
    pub c_cc: [u8; 32],
    pub c_ispeed: c_uint,
    pub c_ospeed: c_uint,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcgetattr(fd: c_int, termios_p: *mut termios) -> c_int {
    std::os::ioctl(fd as usize, 0x5401, termios_p as u64)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcsetattr(fd: c_int, _optional_actions: c_int, termios_p: *const termios) -> c_int {
    std::os::ioctl(fd as usize, 0x5402, termios_p as u64)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfgetispeed(termios_p: *const termios) -> c_uint { (*termios_p).c_ispeed }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfgetospeed(termios_p: *const termios) -> c_uint { (*termios_p).c_ospeed }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfsetispeed(termios_p: *mut termios, speed: c_uint) -> c_int {
    (*termios_p).c_ispeed = speed;
    0
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfsetospeed(termios_p: *mut termios, speed: c_uint) -> c_int {
    (*termios_p).c_ospeed = speed;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfmakeraw(termios_p: *mut termios) {
    (*termios_p).c_iflag &= !(0000001 | 0000002 | 0000010 | 0000040 | 0000100 | 0000200 | 0000400 | 0002000);
    (*termios_p).c_oflag &= !0000001;
    (*termios_p).c_lflag &= !(0000010 | 0000100 | 0000002 | 0000001 | 0100000);
    (*termios_p).c_cflag &= !(0000060 | 0000400);
    (*termios_p).c_cflag |= 0000060;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ioctl(fd: c_int, request: core::ffi::c_long, arg: u64) -> c_int {
    if request == 0x5413 { // TIOCGWINSZ
        let winsize = arg as *mut u16;
        if !winsize.is_null() {
            let cols = std::os::syscall(44, 0, 0, 0) as u16;
            let rows = std::os::syscall(45, 0, 0, 0) as u16;
            *winsize = rows;
            *winsize.add(1) = cols;
            return 0;
        }
    }
    std::os::ioctl(fd as usize, request as u64, arg)
}

#[repr(C)]
pub struct WINDOW {
    pub curr_y: c_int,
    pub curr_x: c_int,
    pub max_y: c_int,
    pub max_x: c_int,
}

#[unsafe(no_mangle)]
pub static mut stdscr: *mut WINDOW = core::ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut curscr: *mut WINDOW = core::ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut LINES: c_int = 25;
#[unsafe(no_mangle)]
pub static mut COLS: c_int = 80;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn initscr() -> *mut WINDOW {
    LINES = std::os::syscall(45, 0, 0, 0) as c_int;
    COLS = std::os::syscall(44, 0, 0, 0) as c_int;
    let win = crate::stdlib::malloc(core::mem::size_of::<WINDOW>()) as *mut WINDOW;
    (*win).curr_y = 0;
    (*win).curr_x = 0;
    (*win).max_y = LINES;
    (*win).max_x = COLS;
    stdscr = win;
    curscr = win;
    win
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn endwin() -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn isendwin() -> c_int { 0 }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wrefresh(_win: *mut WINDOW) -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn refresh() -> c_int { wrefresh(stdscr) }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wmove(win: *mut WINDOW, y: c_int, x: c_int) -> c_int {
    (*win).curr_y = y;
    (*win).curr_x = x;
    if win == stdscr {
        crate::stdio::printf(b"\x1b[%d;%dH\0".as_ptr() as *const c_char, y + 1, x + 1);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn r#move(y: c_int, x: c_int) -> c_int { wmove(stdscr, y, x) }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn waddch(win: *mut WINDOW, ch: c_int) -> c_int {
    let b = [ch as u8];
    crate::unistd::write(1, b.as_ptr() as *const core::ffi::c_void, 1);
    (*win).curr_x += 1;
    if (*win).curr_x >= (*win).max_x {
        (*win).curr_x = 0;
        (*win).curr_y += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn addch(ch: c_int) -> c_int { waddch(stdscr, ch) }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn waddstr(win: *mut WINDOW, s: *const c_char) -> c_int {
    let len = crate::string::strlen(s);
    crate::unistd::write(1, s as *const core::ffi::c_void, len);
    (*win).curr_x += len as c_int; // Simplified
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn addstr(s: *const c_char) -> c_int { waddstr(stdscr, s) }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wgetch(_win: *mut WINDOW) -> c_int {
    let mut b = 0u8;
    if crate::unistd::read(0, &mut b as *mut u8 as *mut core::ffi::c_void, 1) == 1 {
        b as c_int
    } else { -1 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn getch() -> c_int { wgetch(stdscr) }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wclear(win: *mut WINDOW) -> c_int {
    if win == stdscr {
        crate::stdio::printf(b"\x1b[2J\x1b[H\0".as_ptr() as *const c_char);
    }
    (*win).curr_x = 0;
    (*win).curr_y = 0;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clear() -> c_int { wclear(stdscr) }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn keypad(_win: *mut WINDOW, _bf: c_int) -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nodelay(_win: *mut WINDOW, _bf: c_int) -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn noecho() -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn echo() -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cbreak() -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn raw() -> c_int { 0 }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn curs_set(visibility: c_int) -> c_int {
    if visibility == 0 {
        crate::stdio::printf(b"\x1b[?25l\0".as_ptr() as *const c_char);
    } else {
        crate::stdio::printf(b"\x1b[?25h\0".as_ptr() as *const c_char);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_color() -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn has_colors() -> c_int { 1 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_pair(_pair: i16, _f: i16, _b: i16) -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn COLOR_PAIR(_n: c_int) -> c_int { 0 }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn attron(_attrs: c_int) -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn attroff(_attrs: c_int) -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wattron(_win: *mut WINDOW, _attrs: c_int) -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wattroff(_win: *mut WINDOW, _attrs: c_int) -> c_int { 0 }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn standout() -> c_int { crate::stdio::printf(b"\x1b[7m\0".as_ptr() as *const c_char) }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn standend() -> c_int { crate::stdio::printf(b"\x1b[0m\0".as_ptr() as *const c_char) }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wprintw(win: *mut WINDOW, fmt: *const c_char, mut args: ...) -> c_int {
    let mut ap = args.as_va_list();
    crate::stdio::printf_core(|b| {
        let buf = [b];
        crate::unistd::write(1, buf.as_ptr() as *const core::ffi::c_void, 1);
        (*win).curr_x += 1;
    }, fmt, &mut ap)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn printw(fmt: *const c_char, mut args: ...) -> c_int {
    let mut ap = args.as_va_list();
    crate::stdio::vfprintf(crate::stdio::STDOUT, fmt, ap)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mvwaddstr(win: *mut WINDOW, y: c_int, x: c_int, s: *const c_char) -> c_int {
    wmove(win, y, x);
    waddstr(win, s)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mvwaddch(win: *mut WINDOW, y: c_int, x: c_int, ch: c_int) -> c_int {
    wmove(win, y, x);
    waddch(win, ch)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn newwin(nlines: c_int, ncols: c_int, _begin_y: c_int, _begin_x: c_int) -> *mut WINDOW {
    let win = crate::stdlib::malloc(core::mem::size_of::<WINDOW>()) as *mut WINDOW;
    (*win).curr_y = 0;
    (*win).curr_x = 0;
    (*win).max_y = nlines;
    (*win).max_x = ncols;
    win
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn delwin(win: *mut WINDOW) -> c_int {
    crate::stdlib::free(win as *mut core::ffi::c_void);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mvwprintw(win: *mut WINDOW, y: c_int, x: c_int, fmt: *const c_char, mut args: ...) -> c_int {
    wmove(win, y, x);
    let mut ap = args.as_va_list();
    crate::stdio::vfprintf(crate::stdio::STDOUT, fmt, ap)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn waddnstr(win: *mut WINDOW, s: *const c_char, n: c_int) -> c_int {
    let len = if n < 0 { crate::string::strlen(s) } else { n as usize };
    crate::unistd::write(1, s as *const core::ffi::c_void, len);
    (*win).curr_x += len as c_int;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mvwaddnstr(win: *mut WINDOW, y: c_int, x: c_int, s: *const c_char, n: c_int) -> c_int {
    wmove(win, y, x);
    waddnstr(win, s, n)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wredrawln(_win: *mut WINDOW, _beg_line: c_int, _num_lines: c_int) -> c_int { 0 }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wclrtoeol(win: *mut WINDOW) -> c_int {
    if win == stdscr {
        crate::stdio::printf(b"\x1b[K\0".as_ptr() as *const c_char);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clrtoeol() -> c_int { wclrtoeol(stdscr) }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn scrollok(_win: *mut WINDOW, _bf: c_int) -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wscrl(_win: *mut WINDOW, _n: c_int) -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn beep() -> c_int { crate::stdio::printf(b"\x07\0".as_ptr() as *const c_char) }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ungetch(_ch: c_int) -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wnoutrefresh(_win: *mut WINDOW) -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn doupdate() -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn typeahead(_fd: c_int) -> c_int { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nonl() -> c_int { 0 }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn napms(ms: c_int) -> c_int {
    std::os::sleep(ms as u64);
    0
}
