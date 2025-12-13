#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n\nKernel Panic!");
    println!("{}", info);

    #[cfg(test)] {
        serial_println!("{}", info);
        exit_qemu(QemuExitCode::Failed);
    }

    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

#[cfg(test)]
static QEMU_EXIT_PORT: u16 = 0xf4;

#[cfg(test)]
fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(QEMU_EXIT_PORT);
        port.write(exit_code as u32);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kmain();

    #[cfg(test)]
    test_main();
    
    #[allow(unreachable_code)]
    loop {}
}

static MOTD: &str = r"
  _____ _       _           ___  ____  
 |  ___| |_   _| | _____   / _ \/ ___|   (c) 2025
 | |_  | | | | | |/ / __| | | | \___ \   Kevin Wiskia
 |  _| | | |_| |   <\__ \ | |_| |___) |
 |_|   |_|\__,_|_|\_\___/  \___/|____/ 
";

fn kmain() {
    println!("{}", &MOTD);

    #[cfg(test)]
    test_main();

    panic!("Aw shit, write some runtime!");
}
