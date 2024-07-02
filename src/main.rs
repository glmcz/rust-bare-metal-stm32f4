#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use cortex_m::asm;

use rtt_target::{rtt_init_print, rprint};
#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprint!("Hello world");
    loop {
        asm::nop();
    }
}