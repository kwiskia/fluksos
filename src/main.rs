#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fluksos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

use fluksos::{hlt_loop, memory::{self, BootInfoFrameAllocator}, println};
use x86_64::{VirtAddr, structures::paging::{Page}};

entry_point!(kernel_main);

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n\nKernel panic!");
    println!("{}", info);

    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fluksos::test_panic_handler(info)
}

static MOTD: &str = r"
  _____ _       _           ___  ____  
 |  ___| |_   _| | _____   / _ \/ ___|   (c) 2025
 | |_  | | | | | |/ / __| | | | \___ \   Kevin Wiskia
 |  _| | | |_| |   <\__ \ | |_| |___) |
 |_|   |_|\__,_|_|\_\___/  \___/|____/ 
";

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("{}", MOTD);

    fluksos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    hlt_loop()
}
