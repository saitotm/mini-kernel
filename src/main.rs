#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(custom_test_frameworks)]

mod vga_buffer;

use core::panic::PanicInfo;

use x86_64::instructions::hlt;

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}

#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    vga_buffer::print_panic(info);

    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {
        hlt();
    }
}
