#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ouesu::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

use ouesu::hlt_loop;

mod vga_buffer;
mod serial;

entry_point!(kernel_main);

#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use ouesu::memory::active_level_4_table;
    use x86_64::VirtAddr;
    println!("Hello ouesu{}", "!");
    ouesu::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe {
        active_level_4_table(phys_mem_offset)
    };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }

    println!("it didn't crash.");
    hlt_loop();
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