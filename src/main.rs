#![no_std]
#![no_main]

// pick a panicking behavior
// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
#[cfg(debug_assertions)]
extern crate panic_semihosting;

// release profile: minimize the binary size of the application
#[cfg(not(debug_assertions))]
extern crate panic_abort;

use cortex_m_rt::{entry, exception};
#[allow(unused_imports)]
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    let mut peripheral = stm32f30x::Peripherals::take().unwrap();

    peripheral.RCC.ahbenr.write(|w| w.iopeen().set_bit());

    unsafe {peripheral.GPIOE.moder.write(|w| w.bits(0x5555_0000));}
    unsafe {peripheral.GPIOE.bsrr.write(|w| w.bits(0xFF00_0000));}

    const delay_max: u32 = 200_000;
    let mut delay: u32;

    loop {
        //arbitrary delay
        delay = delay_max;
        while delay > 0 {
            delay = delay - 1;
        }

        //set the pins
        unsafe {peripheral.GPIOE.bsrr.write(|w| w.bits(0xFF00_0000));}

        //arbitrary delay
        delay = delay_max;
        while delay > 0 {
            delay = delay - 1;
        }

        //clear the pins
        unsafe {peripheral.GPIOE.bsrr.write(|w| w.bits(0x0000_FF00));}
    }
}
