#![no_std]
#![no_main]
#![feature(lang_items)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo ) -> ! {
    loop {}
}

#[lang = "eh_personality"]
fn eh_personality() {}

#[arduino_hal::entry]
fn main() -> ! {
    loop {}
}