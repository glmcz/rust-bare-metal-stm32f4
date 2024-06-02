#![no_std]
#![no_main]

use rt::entry;

entry!(main);

fn main() {
    let x = 42;

    loop{}
}
