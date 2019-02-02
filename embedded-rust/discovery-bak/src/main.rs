#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    // exit QEMU
    // debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
