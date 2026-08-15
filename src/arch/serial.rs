// Serial port I/O for debugging
// Uses UART 16550 compatible serial ports (COM1)

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;
use core::fmt;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe {
            SerialPort::new(0x3f8) // COM1 I/O port
        };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

pub fn init() {
    let _ = SERIAL1.lock();
}

/// Write a single byte to serial port
pub fn write_byte(byte: u8) {
    SERIAL1.lock().send(byte);
}

/// Write a string slice to serial port
pub fn write_str(s: &str) {
    for byte in s.bytes() {
        write_byte(byte);
    }
}

// Wrapper for Write trait implementation
pub struct SerialWriter;

impl fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            write_byte(byte);
        }
        Ok(())
    }
}
