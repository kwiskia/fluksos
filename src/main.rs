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
  ____       _     _               _    ___  ____  
 / ___|  ___| |__ | | _____   ___ | |  / _ \/ ___|   Kevin Wiskia
 \___ \ / __| '_ \| |/ / _ \ / _ \| | | | | \___ \   (c) 2025
  ___) | (__| | | |   < (_) | (_) | | | |_| |___) |
 |____/ \___|_| |_|_|\_\___/ \___/|_|  \___/|____/ 
";

fn kmain() -> ! {
    println!("{}\n", &MOTD);
    print!("> ");

    panic!("Aw shit!");
}
