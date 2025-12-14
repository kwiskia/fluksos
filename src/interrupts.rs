
use x86_64::{set_general_handler, structures::idt::{InterruptDescriptorTable, InterruptStackFrame}};
use lazy_static::lazy_static;
use crate::println;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        set_general_handler!(&mut idt, breakpoint_handler);
        idt
    };
}

fn breakpoint_handler(
    stack_frame: InterruptStackFrame,
    index: u8,
    error_code: Option<u64>
) {
    println!(
        "Exception Breakpoint!\n{:#?}, {}, {:?}",
        stack_frame,
        index,
        error_code
    );

    match index {
        3 => println!("Breakpoint (int3)!"),
        _ => ()
    }
}

pub fn init_idt() {
    IDT.load();
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
