use std::fmt::{self, Write};
use stream::{Sender, Subscriber};
use self::vga_buffer::{ScreenWriter, Color};
use thread::thread_local_data;

mod vga_buffer;

pub unsafe fn init() {
    let mut screen_writer = ScreenWriter::new(Color::Black, Color::White);
    screen_writer.clear_screen();
    thread_local_data().borrow_mut().stdout = Box::new(screen_writer);
}

#[no_mangle]
pub extern fn print_to_stdout(args: fmt::Arguments) {
    thread_local_data().borrow_mut().stdout.write_fmt(args);
}

#[lang = "panic_fmt"]
extern fn panic_fmt(msg: fmt::Arguments, file: &'static str, line: u32) -> ! {
    let mut err_writer = unsafe {
        ScreenWriter::new(Color::White, Color::Red)
    };
    err_writer.write_fmt(format_args!("\nPANIC: `{}` in `{}` in line `{}`",
        msg, file, line));
    loop {}
}