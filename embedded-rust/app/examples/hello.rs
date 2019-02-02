//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

// Notes
// This program uses something called semihosting to print text 
// to the host console. When using real hardware this requires a
// debug session but when using QEMU this Just Works.

// to build:     cargo build --example hello
// to inspect:   cargo readobj --example hello -- -file-headers
// debug (QEMU): qemu-system-arm \
//               -cpu cortex-m3 \
//               -machine lm3s6965evb \
//               -nographic \
//               -semihosting-config enable=on,target=native \
//               -kernel target/thumbv7m-none-eabi/debug/examples/hello
