
use pic8259::ChainedPics;
use x86_64::{instructions::port::Port, structures::idt::{InterruptDescriptorTable, InterruptStackFrame}};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::{gdt, println, print};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        unsafe {
            idt.breakpoint.set_handler_fn(breakpoint_handler);

            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);

            idt[InterruptIndex::Timer.as_u8()]
                .set_handler_fn(timer_interrupt_handler);

            idt[InterruptIndex::Keyboard.as_u8()]
                .set_handler_fn(keyboard_interrupt_handler);
        }

        idt
    };
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame
) {
    println!("Exception breakpoint!\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, error_code: u64) -> !
{
    panic!("Double fault!\n{:#?}\nCode: {}", stack_frame, error_code);
}

extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(ScancodeSet1::new(),
            layouts::Us104Key, HandleControl::Ignore)
        );
}

static KEYBOARD_PORT: u16 = 0x60;

extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    let mut keyboard = KEYBOARD.lock();
    let mut port= Port::new(KEYBOARD_PORT);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

pub fn init_idt() {
    IDT.load();
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET, // Timer is the first index on PIC_1.
    Keyboard
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }
}

pub fn init_pic() {
    unsafe {
        PICS.lock().initialize();
    }
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
