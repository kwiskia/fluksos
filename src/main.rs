#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n\nKernel Panic!");
    println!("{}", info);
    
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kmain();
    
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

fn kmain() -> ! {
    println!("{}\n", &MOTD);
    print!("> ");

    panic!("Aw shit!");
}
