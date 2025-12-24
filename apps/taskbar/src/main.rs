#![no_std]
#![no_main]

use inkui::{ Window, Widget, Color, Size };
use std::fs::File;
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let heap_size = 1024 * 1024 * 10;
    let heap_ptr = std::memory::malloc(heap_size);
    std::memory::heap::init_heap(heap_ptr as *mut u8, heap_size);

    let screen_w = std::graphics::get_screen_width();
    let screen_h = std::graphics::get_screen_height() * 6 / 100;

    let mut win = Window::new("Taskbar", screen_w, screen_h);
    win.x = 0;
    win.y = 0;

    if let Ok(mut file) = File::open("@0xE0/sys/fonts/CaskaydiaNerd.ttf") {
        let size = file.size();
        let buffer_addr = std::memory::malloc(size);
        let buffer = unsafe { core::slice::from_raw_parts_mut(buffer_addr as *mut u8, size) };
        if file.read(buffer).is_ok() {
            let static_buf = unsafe { core::slice::from_raw_parts(buffer_addr as *const u8, size) };
            win.load_font(static_buf);
        }
    }

    // --- GUI SETUP ---
    let (h, m, _s) = std::os::get_time();
    let time_str = format!("{:02}:{:02}", h, m);
    
    let font_size = 20;
    let text_width = time_str.len() * font_size * 3 / 5; // Approx width for monospaced
    let clock_x = (screen_w / 2).saturating_sub(text_width / 2);
    let clock_y = (screen_h / 2).saturating_sub(font_size / 2);

    let mut root = Widget::frame(1)
        .width(Size::Relative(100))
        .height(Size::Relative(100))
        .background_color(Color::rgba(0, 0, 0, 255));

    let clock = Widget::label(2, &time_str)
        .x(Size::Absolute(clock_x))
        .y(Size::Absolute(clock_y.saturating_sub(2)))
        .set_text_size(font_size)
        .set_text_color(Color::rgb(255, 255, 255))
        .background_color(Color::rgba(0, 0, 0, 0));

    root = root.add_child(clock);

    win.children.push(root);
    win.show();

    let mut last_m = m;
    let mut ticks = 0;

    loop {
        ticks += 1;
        // Basic throttle: update RTC check roughly once a second (depends on yield/system speed)
        if ticks % 100 == 0 {
            let (h, m, _s) = std::os::get_time();
            if m != last_m {
                if let Some(w) = win.find_widget_by_id_mut(2) {
                    if let Widget::Label { text, .. } = w {
                        text.text = format!("{:02}:{:02}", h, m);
                    }
                }
                last_m = m;
            }
            win.update();
            win.draw();
            win.show();
        }

        std::os::yield_task();
    }
}
