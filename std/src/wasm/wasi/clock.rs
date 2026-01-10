use crate::wasm::{Value, interpreter::Interpreter};

pub fn register(interpreter: &mut Interpreter, mod_name: &str) {
    interpreter.add_host_function(mod_name, "clock_res_get", |_interp, _args| Some(Value::I32(0)));
    interpreter.add_host_function(mod_name, "clock_time_get", |_interp, _args| Some(Value::I32(0)));
    interpreter.add_host_function(mod_name, "random_get", |_interp, _args| Some(Value::I32(0)));
}
