//STM32F3Discovery LED Ring, defined from top going clockwise around the ring
const _LED0: u16 = 0x0200u16;	//PE9, LD3, North: Red
const _LED1: u16 = 0x0400u16;	//PE10, LD5, North-East: Orange
const _LED2: u16 = 0x0800u16;	//PE11, LD7, East: Green
const _LED3: u16 = 0x1000u16;	//PE12, LD9, South-East: Blue
const _LED4: u16 = 0x2000u16;	//PE13, LD10, South: Red
const _LED5: u16 = 0x4000u16;	//PE14. LD8, South-West: Orange
const _LED6: u16 = 0x8000u16;	//PE15, LD6, West: Green
const _LED7: u16 = 0x0100u16;	//PE8, LD4, North-West: Blue

enum LED_State{
	LED_On = 0b1,
	LED_Off = 0b0
}

//tell the compile to assemble this struct in c-like manner
//  ie: contiguous in memory as expected
#[repr(C)]
struct IOPort{
    pub 
}