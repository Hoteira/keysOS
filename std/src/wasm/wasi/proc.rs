use crate::wasm::{Value, interpreter::Interpreter};

pub fn register(interpreter: &mut Interpreter, mod_name: &str) {
    interpreter.add_host_function(mod_name, "args_sizes_get", |_interp, _args| Some(Value::I32(0)));
    interpreter.add_host_function(mod_name, "args_get", |_interp, _args| Some(Value::I32(0)));
    interpreter.add_host_function(mod_name, "environ_sizes_get", |_interp, _args| Some(Value::I32(0)));
    interpreter.add_host_function(mod_name, "environ_get", |_interp, _args| Some(Value::I32(0)));

    interpreter.add_host_function(mod_name, "proc_exit", |_interp, args| {
        let code = match args.get(0) { Some(Value::I32(v)) => *v as u64, _ => 0 };
        crate::os::exit(code);
        None
    });

    interpreter.add_host_function(mod_name, "proc_raise", |_interp, _args| { crate::os::yield_task(); Some(Value::I32(0)) });
    interpreter.add_host_function(mod_name, "sched_yield", |_interp, _args| { crate::os::yield_task(); Some(Value::I32(0)) });
}
