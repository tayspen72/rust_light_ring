#![no_std]
#![no_main]

// pick a panicking behavior
// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
#[cfg(debug_assertions)]
extern crate panic_semihosting;

// release profile: minimize the binary size of the application
#[cfg(not(debug_assertions))]
extern crate panic_abort;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
#[allow(unused_imports)]
use cortex_m_semihosting::{debug, hprintln};

fn init_gpio_e(p: &stm32f30x::Peripherals){
    unsafe {p.GPIOE.moder.write(|w| w.bits(0x5555_0000));}
    unsafe {p.GPIOE.bsrr.write(|w| w.bits(0xFF00_0000));}
}

fn init_rcc(p: &stm32f30x::Peripherals){ 
    //enable GPIO Port E
    p.RCC.ahbenr.write(|w| w.iopeen().set_bit());
}

fn init_systick(){
    let mut p = cortex_m::Peripherals::take().unwrap();
    p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(2_097_151); // base 8MHz, (2^22) - 1 = ~0.26 seconds
    p.SYST.enable_counter();
    p.SYST.enable_interrupt();
}

static mut UPDATE_LEDS: bool = false;

#[exception]
fn SysTick(){
    unsafe {UPDATE_LEDS = true;}
}

#[entry]
fn main() -> ! {
    let peripheral = stm32f30x::Peripherals::take().unwrap();

    init_rcc(&peripheral);
    init_gpio_e(&peripheral);
    init_systick();

    loop {
        unsafe{
            if UPDATE_LEDS == true {
                UPDATE_LEDS = false;
                update_led(&peripheral);
            }
        }
    }
}

fn update_led(p: &stm32f30x::Peripherals){
    static mut PIN: u8 = 0;
    static mut STATE: u8 = 1;

    unsafe{
        write_led(&p, PIN, STATE);

        //update conditions
        PIN = PIN + 1;
        if PIN == 8 {
            PIN = 0;

            if STATE == 0 {
                STATE = 1;
            }
            else {
                STATE = 0;
            }
        }
    }
}


fn write_led(p: &stm32f30x::Peripherals, led: u8, output_state: u8){
    //convert led to bit mask based on pin layout
    let mut mask: u32 = match led{
        0 => (1 << 9),
        1 => (1 << 10),
        2 => (1 << 11),
        3 => (1 << 12),
        4 => (1 << 13),
        5 => (1 << 14),
        6 => (1 << 15),
        _ => (1 << 8)
    };

    //if turning off, shift up  to reset bits
    if output_state == 0 {
        mask = mask << 16;
    }

    unsafe { p.GPIOE.bsrr.write(|w| w.bits(mask)); }
}