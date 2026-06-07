fn add(a: u8, b: u8) -> u8 {
	a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn riscv_main() -> ! {
	let mut c: u8 = 1;
	loop {
		c = add(c, 1);
	}
}
