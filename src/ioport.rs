use volatile_register::{RW, RO, WO};

//STM32F3Discovery LED Ring, defined from top going clockwise around the ring
pub const LED0: u16 = 0x0200;		//PE 9, LD3, North: Red
pub const LED1: u16 = 0x0400;		//PE10, LD5, North-East: Orange
pub const LED2: u16 = 0x0800;		//PE11, LD7, East: Green
pub const LED3: u16 = 0x1000;		//PE12, LD9, South-East: Blue
pub const LED4: u16 = 0x2000;		//PE13, LD10, South: Red
pub const LED5: u16 = 0x4000;		//PE14. LD8, South-West: Orange
pub const LED6: u16 = 0x8000;		//PE15, LD6, West: Green
pub const LED7: u16 = 0x0100;		//PE 8, LD4, North-West: Blue

pub const PORTA: u32 = 0x4800_0000;
pub const PORTB: u32 = 0x4800_0400;
pub const PORTC: u32 = 0x4800_0C00;
pub const PORTD: u32 = 0x4800_1000;
pub const PORTE: u32 = 0x4800_1400;
pub const PORTF: u32 = 0x4800_1800;

pub enum PinState{
	PinSet = 0b1,
	PinReset = 0b0
}

pub enum PinMode{
	ModeInput,
	ModeOutput,
	ModeAlternateFunction,
	ModeAnalog
}

pub struct IOPort{
	reg: &'static mut IOPortRegisterBlock

}

//tell the compile to assemble this struct in c-like manner
//  ie: contiguous in memory as expected
#[repr(C)]
struct IOPortRegisterBlock{
	pub moder: RW<u32>,
	pub otyper: RW<u32>, 
	pub ospeedr: RW<u32>,
	pub pupdr: RW<u32>,
	pub idr: RO<u32>,
	pub odr: RW<u32>, 
	pub bsrr: WO<u32>,
	pub lckr: RW<u32>,
	pub afrl: RW<u32>,
	pub afrh: RW<u32>,
	pub brr: WO<u32>
}

impl IOPort{
	fn get_port(port: u32) -> IOPort {
		IOPort{
			reg: unsafe { &mut *(port as *mut IOPortRegisterBlock) }
		}
	}

	fn set_reset(&mut self, pins: u16, state: PinState) {
		match state {
			PinState::PinSet => {
				//if setting pins high, simply write the pins to the register
				unsafe { self.reg.bsrr.write(pins.into()) };
			},
			PinState::PinReset => {
				//if performing a reset, shift over for u32 register size
				let reg: u32 = ((pins as u32) << 16).into();
				unsafe { self.reg.bsrr.write(reg) };
			}
		}
	}
	fn set_mode(&mut self, pins: u16, mode: PinMode){
		//build the mode value
		let mode_val = match mode{
			PinMode::ModeInput => 0x0,
			PinMode::ModeOutput => 0x1,
			PinMode::ModeAlternateFunction => 0x2,
			PinMode::ModeAnalog => 0x3,
		};
		let mut val: u32 = 0;
		for i in 0..=15 {
			if (pins & (i << i)) > 0 {
				val = val | mode_val << (2 * i);
			}
		}
		
		//perform the write
		unsafe { self.reg.moder.write(val) };	
	}
}

/** Private Functions **/


/** Public Functions **/
pub fn init_leds(){
	let leds: u16 = 
		LED0 | LED1 | LED2 | LED3 | LED4 | LED5 | LED6 | LED7;

	let mut p = IOPort::get_port(PORTE);

	p.set_mode(leds, PinMode::ModeOutput);
	p.set_reset(leds, PinState::PinReset);
}