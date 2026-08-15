// CPU-related functions and features

use x86_64::registers::control::Cr0Flags;

/// Get CPU vendor string
pub fn get_vendor_string() -> &'static str {
    "NIVA CPU"
}

/// Enable write-protect bit in CR0
pub fn enable_write_protect() {
    unsafe {
        let mut cr0 = x86_64::registers::control::Cr0::read();
        cr0.insert(Cr0Flags::WRITE_PROTECT);
        x86_64::registers::control::Cr0::write(cr0);
    }
}

/// Halt CPU
pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/// Get CPU core count
pub fn get_core_count() -> u32 {
    1 // Default to 1, will be detected from CPUID later
}
