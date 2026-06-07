use crate::printk;

#[unsafe(no_mangle)]
pub extern "C" fn mtrap_handler(mcause: u64, mtval: u64, mepc: u64) {
	printk!(
		"Inside mtrap handler mcause:{:x} mtval:{:x} mepc:{:x} \n",
		mcause,
		mtval,
		mepc
	);
}
