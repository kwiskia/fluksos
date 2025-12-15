#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fluksos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use fluksos::{hlt_loop, println};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kmain();

    hlt_loop();
}

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

fn kmain() {
    println!("{}", MOTD);

    fluksos::init();

    #[cfg(test)]
    test_main();

    panic!("Aw shit, write some runtime!");
}
