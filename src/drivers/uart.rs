use core::ptr::{read_volatile, write_volatile};

const OFFSET_UART_DATA: usize	= 0x00;
const OFFSET_UART_LSR: usize	= 0x05;

const UART_BASE: usize		= 0x1000_0000;

const UART_DATA: usize		= UART_BASE + OFFSET_UART_DATA;
const UART_LSR: usize		= UART_BASE + OFFSET_UART_LSR;

const BIT_UART_LSR_THRE: u8 = 0x20;

pub struct Uart {
}

impl Uart {
	fn send_byte(byte: u8) {
		unsafe {
			let mut line_status: u8 = read_volatile(UART_LSR as *const u8);
			while (line_status & BIT_UART_LSR_THRE) == 0 {
				line_status = read_volatile(UART_LSR as *const u8);
			}
			write_volatile(UART_DATA as *mut u8, byte);
		}
	}

	pub fn send_char(c: char) {
		let mut buf = [0; 4];
		for byte in c.encode_utf8(&mut buf).bytes() {
			Uart::send_byte(byte);
		}
	}
}

