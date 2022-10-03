#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use core::intrinsics;
use core::panic::PanicInfo;
use x86_64::instructions::{hlt};

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    intrinsics::abort();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let framebuffer = 0xb800 as *mut u8;
    unsafe {
        framebuffer
            .offset(1)
            .write_volatile(0x30);
    }
    loop {
        hlt();
    }
}