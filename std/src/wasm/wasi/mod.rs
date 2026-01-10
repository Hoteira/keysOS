use crate::wasm::interpreter::Interpreter;

pub mod file;
pub mod proc;
pub mod clock;
pub mod poll;

pub struct Wasi;

impl Wasi {
    pub fn register(interpreter: &mut Interpreter) {
        let mod_name = "wasi_snapshot_preview1";

        proc::register(interpreter, mod_name);
        clock::register(interpreter, mod_name);
        file::register(interpreter, mod_name);
        poll::register(interpreter, mod_name);
    }
}
