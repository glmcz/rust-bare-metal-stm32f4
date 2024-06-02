#![no_std]

use core::panic::PanicInfo;

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            let f: fn() -> ! = $path;

            f()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! { //The exact location of the reset handler itself, Reset, is not important. 
    extern "Rust" {
        fn main() -> !;
    }
    main()
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}