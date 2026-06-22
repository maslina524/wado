#![no_std]
#![no_main]

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod link;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    0
}