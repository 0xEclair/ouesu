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
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello ouesu{}", "!");
    ouesu::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

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