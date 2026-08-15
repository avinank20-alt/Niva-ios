// NIVA OS - Kernel Main Entry Point
// This is the first Rust code that runs after the bootloader

#![no_std]
#![no_main]

use niva_os::println;
use niva_os::arch::serial;
use niva_os::drivers::pic;
use niva_os::kernel::gdt;
use niva_os::kernel::idt;

/// Kernel main entry point (called from bootloader in 64-bit mode)
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // Initialize serial port for early debugging
    serial::init();
    
    println!("╔════════════════════════════════════════╗");
    println!("║       NIVA OS v0.1.0                   ║");
    println!("║  Desktop Operating System              ║");
    println!("╚════════════════════════════════════════╝");
    println!();
    
    println!("[*] Initializing kernel...");
    
    // Set up Global Descriptor Table
    println!("[*] Setting up GDT...");
    gdt::init();
    
    // Set up Interrupt Descriptor Table
    println!("[*] Setting up IDT...");
    idt::init();
    
    // Initialize PIC (Programmable Interrupt Controller)
    println!("[*] Initializing PIC...");
    pic::init();
    
    // Enable interrupts
    println!("[*] Enabling interrupts...");
    x86_64::instructions::interrupts::enable();
    
    println!("[✓] Kernel initialized successfully!");
    println!();
    
    // Print CPU information
    println!("System Information:");
    println!("  Architecture: x86_64");
    println!("  Mode: Long Mode (64-bit)");
    println!("  Platform: NIVA OS");
    println!();
    
    // Main kernel loop
    println!("NIVA OS is ready. Initializing desktop environment...");
    println!();
    
    // TODO: Initialize framebuffer and graphics
    // TODO: Initialize file system
    // TODO: Load desktop environment
    // TODO: Start application launcher
    
    // Idle loop
    loop {
        x86_64::instructions::hlt();
    }
}
