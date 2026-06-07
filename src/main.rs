#![no_std]
#![no_main]

mod boot;
mod drivers;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
