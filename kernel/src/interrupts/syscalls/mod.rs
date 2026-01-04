use core::arch::naked_asm;
use crate::interrupts::task::CPUState;
use crate::debugln;

pub mod fs;
pub use process::spawn_process;
pub mod process;
pub mod memory;
pub mod window;
pub mod misc;

pub const SYS_READ: u64 = 0;
pub const SYS_PRINT: u64 = 1;
pub const SYS_MALLOC: u64 = 5;
pub const SYS_FREE: u64 = 6;
pub const SYS_COPY_TO_DB: u64 = 8;
pub const SYS_ADD_WINDOW: u64 = 22;
pub const SYS_REMOVE_WINDOW: u64 = 23;
pub const SYS_GET_WIDTH: u64 = 44;
pub const SYS_GET_HEIGHT: u64 = 45;
pub const SYS_UPDATE_WINDOW: u64 = 51;
pub const SYS_UPDATE_WINDOW_AREA: u64 = 56;
pub const SYS_EXIT: u64 = 60;

pub const SYS_POLL: u64 = 70;
pub const SYS_CREATE_FILE: u64 = 71;
pub const SYS_CREATE_DIR: u64 = 72;
pub const SYS_REMOVE: u64 = 73;
pub const SYS_RENAME: u64 = 74;
pub const SYS_SLEEP: u64 = 76;
pub const SYS_GET_PROCESS_LIST: u64 = 77;
pub const SYS_GET_PROCESS_MEM: u64 = 79;

pub const SYS_CHDIR: u64 = 80;
pub const SYS_MKDIR: u64 = 83;
pub const SYS_RMDIR: u64 = 84;

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub extern "C" fn syscall_entry() {
    unsafe {
        naked_asm!(
            "mov [{scratch}], r15",
            "mov r15, rsp",
            "mov rsp, [{kernel_stack_ptr}]",
            "push QWORD PTR 0x23", 
            "push r15",
            "push r11",
            "push QWORD PTR 0x33", 
            "push rcx",
            "mov r15, [{scratch}]",
            "push rbp",
            "push rax",
            "push rbx",
            "push rcx",
            "push rdx",
            "push rsi",
            "push rdi",
            "push r8",
            "push r9",
            "push r10",
            "push r11",
            "push r12",
            "push r13",
            "push r14",
            "push r15",
            "cld", 
            "mov rdi, rsp",
            "call syscall_dispatcher",
            "pop r15",
            "pop r14",
            "pop r13",
            "pop r12",
            "pop r11",
            "pop r10",
            "pop r9",
            "pop r8",
            "pop rdi",
            "pop rsi",
            "pop rdx",
            "pop rcx",
            "pop rbx",
            "pop rax",
            "pop rbp",
            "iretq",
            kernel_stack_ptr = sym crate::interrupts::task::KERNEL_STACK_PTR,
            scratch = sym crate::interrupts::task::SCRATCH,
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn syscall_dispatcher(context: &mut CPUState) {
    let syscall_num = context.rax;
    
    context.rax = 0; 

    match syscall_num {
        SYS_READ => fs::handle_read(context),
        SYS_PRINT => misc::handle_print(context),
        SYS_MALLOC => memory::handle_malloc(context),
        SYS_FREE => memory::handle_free(context),
        SYS_ADD_WINDOW => window::handle_add_window(context),
        SYS_UPDATE_WINDOW => window::handle_update_window(context),
        SYS_UPDATE_WINDOW_AREA => window::handle_update_window_area(context),
        52 => window::handle_get_events(context),
        SYS_GET_WIDTH => window::handle_get_width(context),
        SYS_GET_HEIGHT => window::handle_get_height(context),
        61 => fs::handle_open(context),
        62 => fs::handle_read_file(context),
        63 => fs::handle_write_file(context),
        64 => fs::handle_read_dir(context),
        65 => fs::handle_file_size(context),
        42 => fs::handle_pipe(context),
        53 => window::handle_get_mouse(context),
        54 => misc::handle_time(context),
        55 => misc::handle_ticks(context),
        SYS_EXIT => process::handle_exit(context),
        66 => process::handle_spawn(context),
        67 => fs::handle_close(context),
        75 => fs::handle_seek(context),
        68 => process::handle_wait_pid(context),
        70 => fs::handle_poll(context),
        71 => fs::handle_create(context, 71),
        72 => fs::handle_create(context, 72),
        73 => fs::handle_remove(context),
        74 => fs::handle_rename(context),
        76 => process::handle_sleep(context),
        77 => process::handle_get_process_list(context),
        78 => process::handle_kill(context),
        79 => memory::handle_get_process_mem(context),
        SYS_CHDIR => fs::handle_chdir(context),
        SYS_MKDIR => fs::handle_create(context, 72),
        SYS_RMDIR => fs::handle_remove(context),
        _ => {
            debugln!("[Syscall] Unknown syscall #{}", syscall_num);
            context.rax = u64::MAX;
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PollFd {
    pub fd: i32,
    pub events: i16,
    pub revents: i16,
}

pub const POLLIN: i16 = 0x001;
pub const POLLOUT: i16 = 0x004;
pub const POLLERR: i16 = 0x008;
pub const POLLNVAL: i16 = 0x020;