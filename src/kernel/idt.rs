// Interrupt Descriptor Table (IDT) for x86_64

use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use crate::println;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        
        // CPU exceptions
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.general_protection_fault.set_handler_fn(gpf_handler);
        
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(0);
            idt.page_fault.set_handler_fn(page_fault_handler);
        }
        
        // Hardware interrupts will be set up by PIC handler
        idt[32].set_handler_fn(timer_interrupt_handler);      // IRQ0 - Timer
        idt[33].set_handler_fn(keyboard_interrupt_handler);   // IRQ1 - Keyboard
        
        idt
    };
}

pub fn init() {
    IDT.load();
}

// Exception handlers
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("[!] BREAKPOINT at {:?}", stack_frame.instruction_pointer);
}

extern "x86-interrupt" fn gpf_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    println!("[!] GENERAL PROTECTION FAULT");
    println!("    Error Code: {}", error_code);
    println!("    Stack Frame: {:?}", stack_frame);
    loop { x86_64::instructions::hlt(); }
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    println!("[!!!] DOUBLE FAULT");
    println!("      {:?}", stack_frame);
    loop { x86_64::instructions::hlt(); }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    println!("[!] PAGE FAULT");
    println!("    Access Address: {:?}", Cr2::read());
    println!("    Error Code: {:?}", error_code);
    println!("    Stack Frame: {:?}", stack_frame);
    loop { x86_64::instructions::hlt(); }
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Acknowledge the interrupt
    unsafe {
        crate::drivers::pic::PICS.lock().notify_end_of_interrupt(32);
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // TODO: Handle keyboard input
    unsafe {
        crate::drivers::pic::PICS.lock().notify_end_of_interrupt(33);
    }
}
