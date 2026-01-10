use crate::wasm::{Value, interpreter::Interpreter};

pub fn register(interpreter: &mut Interpreter, mod_name: &str) {
    interpreter.add_host_function(mod_name, "poll_oneoff", |_interp, _args| Some(Value::I32(0)));
    interpreter.add_host_function(mod_name, "fd_prestat_get", |interp, args| {
        let fd = match args[0] { Value::I32(v) => v, _ => return Some(Value::I32(28)) };
        let prestat_ptr = match args[1] { Value::I32(v) => v as usize, _ => return Some(Value::I32(28)) };
        if fd == 3 {
            if prestat_ptr + 8 <= interp.memory.len() { interp.memory[prestat_ptr] = 0; interp.memory[prestat_ptr+4..prestat_ptr+8].copy_from_slice(&1u32.to_le_bytes()); }
            return Some(Value::I32(0));
        }
        Some(Value::I32(8))
    });
    interpreter.add_host_function(mod_name, "fd_prestat_dir_name", |interp, args| {
        let fd = match args[0] { Value::I32(v) => v, _ => return Some(Value::I32(28)) };
        let path_ptr = match args[1] { Value::I32(v) => v as usize, _ => return Some(Value::I32(28)) };
        if fd == 3 && path_ptr < interp.memory.len() { interp.memory[path_ptr] = b'/'; return Some(Value::I32(0)); }
        Some(Value::I32(8))
    });

    interpreter.add_host_function(mod_name, "sock_recv", |_interp, _args| { crate::os::yield_task(); Some(Value::I32(58)) });
    interpreter.add_host_function(mod_name, "sock_send", |_interp, _args| { crate::os::yield_task(); Some(Value::I32(58)) });
    interpreter.add_host_function(mod_name, "sock_shutdown", |_interp, _args| { crate::os::yield_task(); Some(Value::I32(58)) });
}
