use crate::printk;

#[unsafe(no_mangle)]
pub extern "C" fn riscv_main() {
	printk!("Booting rusty-riscv kernel...\n");
}
