// NIVA OS - Kernel Library
// Core kernel functionality and hardware abstractions

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![allow(dead_code)]

use core::panic::PanicInfo;

pub mod arch;
pub mod drivers;
pub mod kernel;
pub mod memory;
pub mod gui;
pub mod fs;
pub mod sys;
pub mod phase2;

// Panic handler for bare metal
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[KERNEL PANIC] {}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!($crate::arch::serial::SerialWriter, "{}", format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = writeln!($crate::arch::serial::SerialWriter, "{}", format_args!($($arg)*));
    });
}
