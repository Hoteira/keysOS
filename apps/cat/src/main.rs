#![no_std]
#![no_main]

extern crate alloc;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    let args = std::args();
    if args.len() > 1 {
        for path in &args[1..] {
            if let Ok(mut file) = std::fs::File::open(path) {
                let mut buf = [0u8; 1024];
                loop {
                    match file.read(&mut buf) {
                        Ok(n) if n > 0 => {
                            std::os::file_write(1, &buf[0..n]);
                        }
                        _ => break,
                    }
                }
            } else {
                std::println!("cat: {}: No such file", path);
            }
        }
    } else {
        let mut buf = [0u8; 1024];
        loop {
            let n = std::os::file_read(0, &mut buf);
            if n == 0 { break; }
            if n == usize::MAX { break; }
            std::os::file_write(1, &buf[0..n]);
        }
    }

    0
}
