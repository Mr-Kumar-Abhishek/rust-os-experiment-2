#![feature(lang_items)]

use os::stream::*;

extern crate os;

pub mod unimplemented_functions;

#[no_mangle]
pub extern fn main(multiboot: os::MultibootHeader) {
    unsafe{os::init(multiboot)};

    println!("42{}", "2");
    loop{}
    println!("Hello World {:?}!{}!{}!", 42, 'a', 42);
    loop{}

    let s = SpscSender::on_value(|msg| println!("msg: {}", msg));

    s.send("test1");
    s.send("test2");
    s.close();

    print!("this is a ");
    println!("test message...");
    loop{}
}
