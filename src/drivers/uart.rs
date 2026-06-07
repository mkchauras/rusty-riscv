use core::ptr::{read_volatile, write_volatile};
use core::fmt::{self, Write};

const OFFSET_UART_DATA: usize	= 0x00;
const OFFSET_UART_LSR: usize	= 0x05;

const UART_BASE: usize		= 0x1000_0000;

const UART_DATA: usize		= UART_BASE + OFFSET_UART_DATA;
const UART_LSR: usize		= UART_BASE + OFFSET_UART_LSR;

const BIT_UART_LSR_THRE: u8 = 0x20;

pub struct Console;

impl Console {
	fn send_byte(byte: u8) {
		unsafe {
			let mut line_status: u8 = read_volatile(UART_LSR as *const u8);
			while (line_status & BIT_UART_LSR_THRE) == 0 {
				line_status = read_volatile(UART_LSR as *const u8);
			}
			write_volatile(UART_DATA as *mut u8, byte);
		}
	}

	fn send_char(c: char) {
		let mut buf = [0; 4];
		for byte in c.encode_utf8(&mut buf).bytes() {
			Console::send_byte(byte);
		}
	}

	pub fn print_str(s: &str) {
		for c in s.chars() {
			Self::send_char(c);
		}
	}
}

// Implement core::fmt::Write for Console to enable format! macro support
impl Write for Console {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		Console::print_str(s);
		Ok(())
	}
}

#[macro_export]
macro_rules! printk {
	($($arg:tt)*) => {{
		use core::fmt::Write;
		let _ = write!($crate::drivers::uart::Console, $($arg)*);
	}};
}

