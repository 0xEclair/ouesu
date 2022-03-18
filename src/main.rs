#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ouesu::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;

mod vga_buffer;
mod serial;

#[cfg(not(test))]
#[no_mangle] // bootloader
pub extern "C" fn _start() -> ! {
    println!("Hello ouesu{}", "!");
    ouesu::init();

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    #[cfg(test)]
    test_main();

    println!("it didn't crash.");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use ouesu::test_panic_handler;
    test_panic_handler(info);
}

#[test_case]
fn trival_assertion() {
    assert_eq!(1, 1);
}