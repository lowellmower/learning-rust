#![no_std]  // no std lib, now links to core crate
#![no_main] // don't use standard main entrypoint interface

// pick a panicking behavior
extern crate panic_halt;           // breakpoint on `rust_begin_unwind` catch panics
// extern crate panic_abort;       // requires nightly
// extern crate panic_itm;         // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
// will be the only process running on the target hardware so we don't want it to end
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {
        // your code goes here
    }
}
