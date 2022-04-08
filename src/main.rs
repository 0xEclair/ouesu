#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ouesu::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::mem::transmute;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

use ouesu::hlt_loop;

mod vga_buffer;
mod serial;

entry_point!(kernel_main);

#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use ouesu::memory;
    use x86_64::VirtAddr;
    use x86_64::structures::paging::{
        PageTable,
        Translate
    };

    println!("Hello ouesu{}", "!");
    ouesu::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };
    let addresses = [0xb8000, 0x201008, 0x0100_0020_1a10, boot_info.physical_memory_offset];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
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