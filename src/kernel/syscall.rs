// System calls interface

pub struct SyscallHandler;

impl SyscallHandler {
    pub fn handle_syscall(number: u64, _arg1: u64, _arg2: u64, _arg3: u64) -> u64 {
        match number {
            0 => { /* exit */ 0 }
            1 => { /* write */ 0 }
            _ => 0,
        }
    }
}
