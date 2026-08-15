// Kernel subsystems

pub mod gdt;
pub mod idt;
pub mod syscall;
pub mod task;

pub struct KernelInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub arch: &'static str,
}

pub static KERNEL_INFO: KernelInfo = KernelInfo {
    name: "NIVA OS",
    version: "0.1.0",
    arch: "x86_64",
};
