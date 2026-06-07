use crate::printk;

#[unsafe(no_mangle)]
pub extern "C" fn riscv_main() -> ! {
	// Example usage of printk! macro - behaves like Linux printk
	printk!("Booting rusty-riscv kernel...\n");
	printk!("Kernel version: {}.{}.{}\n", 0, 1, 0);

	// Demonstrate format string capabilities
	let value = 42;
	printk!("The answer is: {}\n", value);
	printk!("Hex: 0x{:x}, Binary: 0b{:b}\n", value, value);

	// Multiple arguments
	printk!("System initialized with {} CPUs and {} MB RAM\n", 1, 128);

	printk!("Kernel ready!\n");

	loop {
	}
}
