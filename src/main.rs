#![no_std]
#![no_main]

// pick a panicking behavior
// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
#[cfg(debug_assertions)]
extern crate panic_semihosting;

// release profile: minimize the binary size of the application
#[cfg(not(debug_assertions))]
extern crate panic_abort;

use cortex_m_rt::entry;
#[allow(unused_imports)]
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    let peripheral = stm32f30x::Peripherals::take().unwrap();

    peripheral.RCC.ahbenr.write(|w| w.iopeen().set_bit());

    unsafe {peripheral.GPIOE.moder.write(|w| w.bits(0x5555_0000));}
    unsafe {peripheral.GPIOE.bsrr.write(|w| w.bits(0xFF00_0000));}

    const DELAY_MAX: u32 = 25_000;
    let mut delay: u32;
    let mut state: u8 = 0;
    let mut output: u8 = 1;

    loop {
        //arbitrary delay
        delay = DELAY_MAX;
        while delay > 0 {
            delay = delay - 1;
        }

        //update output display
        match state {
            0 => {  //PE 9, LD3, North: Red
                if output == 0 {
                    peripheral.GPIOE.bsrr.write(|w| w.br9().reset());
                }
                else {
                    peripheral.GPIOE.bsrr.write(|w| w.bs9().set());
                }
            },
            1 => {  //PE10, LD5, North-East: Orange
                if output == 0 {
                    peripheral.GPIOE.bsrr.write(|w| w.br10().reset());
                }
                else {
                    peripheral.GPIOE.bsrr.write(|w| w.bs10().set());
                }
            },
            2 => {  //PE11, LD7, East: Green
                if output == 0 {
                    peripheral.GPIOE.bsrr.write(|w| w.br11().reset());
                }
                else {
                    peripheral.GPIOE.bsrr.write(|w| w.bs11().set());
                }
            },
            3 => {  //PE12, LD9, South-East: Blue
                if output == 0 {
                    peripheral.GPIOE.bsrr.write(|w| w.br12().reset());
                }
                else {
                    peripheral.GPIOE.bsrr.write(|w| w.bs12().set());
                }
            },
            4 => {  //PE13, LD10, South: Red
                if output == 0 {
                    peripheral.GPIOE.bsrr.write(|w| w.br13().reset());
                }
                else {
                    peripheral.GPIOE.bsrr.write(|w| w.bs13().set());
                }
            },
            5 => {  //PE14. LD8, South-West: Orange
                if output == 0 {
                    peripheral.GPIOE.bsrr.write(|w| w.br14().reset());
                }
                else {
                    peripheral.GPIOE.bsrr.write(|w| w.bs14().set());
                }
            },
            6 => {  //PE15, LD6, West: Green
                if output == 0 {
                    peripheral.GPIOE.bsrr.write(|w| w.br15().reset());
                }
                else {
                    peripheral.GPIOE.bsrr.write(|w| w.bs15().set());
                }
            },
            _ => {  //PE 8, LD4, North-West: Blue
                if output == 0 {
                    peripheral.GPIOE.bsrr.write(|w| w.br8().reset());
                }
                else {
                    peripheral.GPIOE.bsrr.write(|w| w.bs8().set());
                }
            }
        }

        //set up conditions
        state = state + 1;
        if state == 8 {
            state = 0;

            if output == 0 {
                output = 1;
            }
            else {
                output = 0;
            }
        }
    }
}

//STM32F3Discovery LED Ring, defined from top going clockwise around the ring
// pub const LED0: u16 = 0x0200;		//PE 9, LD3, North: Red
// pub const LED1: u16 = 0x0400;		//PE10, LD5, North-East: Orange
// pub const LED2: u16 = 0x0800;		//PE11, LD7, East: Green
// pub const LED3: u16 = 0x1000;		//PE12, LD9, South-East: Blue
// pub const LED4: u16 = 0x2000;		//PE13, LD10, South: Red
