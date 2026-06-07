#![no_std]
#![no_main]

mod boot;
mod drivers;
mod kernel;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    printk!("Kernel panic: {}\n", info);
    loop {}
}
