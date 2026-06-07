use crate::drivers::uart::Uart;

#[unsafe(no_mangle)]
pub extern "C" fn riscv_main() -> ! {
	Uart::send_char('H');
	Uart::send_char('e');
	Uart::send_char('l');
	Uart::send_char('l');
	Uart::send_char('o');
	Uart::send_char(' ');
	Uart::send_char('W');
	Uart::send_char('o');
	Uart::send_char('r');
	Uart::send_char('l');
	Uart::send_char('d');
	Uart::send_char('!');
	loop {
	}
}
