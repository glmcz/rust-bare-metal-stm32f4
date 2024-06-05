#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use rt::entry;

use core::intrinsics;

entry!(main);


static RODATA: &[u8] = b"Hello world";
static mut BSS: u8 = 0;
static mut DATA: u8 = 1;

fn main() -> ! {
    intrinsics::abort();
    // let _x = RODATA;
    // let _y = unsafe { BSS };
    // let _z = unsafe { DATA };
    // loop{}
}

#[no_mangle]
pub extern "C" fn HardFault() -> ! {
    loop {}
}