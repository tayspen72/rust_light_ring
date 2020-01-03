#![no_std]
#![no_main]

#[allow(dead_code)]

//declare external source files
mod stm32;
use stm32::ioport as io;

// pick a panicking behavior
// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
#[cfg(debug_assertions)]
extern crate panic_halt;

// release profile: minimize the binary size of the application
#[cfg(not(debug_assertions))]
extern crate panic_abort;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    hprintln!("Launching light ring").unwrap();

    stm32::ioport::init_leds();
    stm32::clock::init_clock();

    loop {
        // your code goes here
    }
}
