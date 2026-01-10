#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fluksos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;

extern crate alloc;

use fluksos::{
    allocator,
    hlt_loop,
    memory::{self, BootInfoFrameAllocator},
    println,
    task::{
        Task, executor::Executor, keyboard
    }
};

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
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap initialization failed");

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses())); // new
    executor.run();

    #[allow(unreachable_code)]
    hlt_loop()
}
