#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kmain();
}

static MOTD: &[u8] = b"SchkoolOS (c) 2025";

fn kmain() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in MOTD.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // Charater
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // Color
        }
    }

    loop {}
}
