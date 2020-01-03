use volatile_register::{RW, RO};

//STM32F3Discovery LED Ring, defined from top going clockwise around the ring
enum Led{
	Led0 = 0x0200,		//PE 9, LD3, North: Red
	Led1 = 0x0400,		//PE10, LD5, North-East: Orange
	Led2 = 0x0800,		//PE11, LD7, East: Green
	Led3 = 0x1000,		//PE12, LD9, South-East: Blue
	Led4 = 0x2000,		//PE13, LD10, South: Red
	Led5 = 0x4000,		//PE14. LD8, South-West: Orange
	Led6 = 0x8000,		//PE15, LD6, West: Green
	Led7 = 0x0100,		//PE 8, LD4, North-West: Blue
}

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

pub enum PortName{
	PortA,
	PortB,
	PortC,
	PortD,
	PortE,
	PortF,
	PortG,
	PortH
}

//tell the compile to assemble this struct in c-like manner
//  ie: contiguous in memory as expected
#[repr(C)]
struct IOPort{
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

/** Private Functions **/


/** Public Functions **/
pub fn InitPins(port: PortName, pins: u16, mode: PinMode){
	let val = match mode{
		PinMode::ModeInput => 0x0
		PinMode::ModeOutput => 0x1
		PinMode::ModeAlternateFunction => initAltFuncPins(port, pins),
		PinMode::ModeAnalog => initAnalogPins(port, pins),
	}
}