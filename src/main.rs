#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle] // bootloader
pub extern "C" fn _start() -> ! {
    println!("Hello ouesu{}", "!");
    panic!("Some message.");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
