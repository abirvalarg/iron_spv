use core::ptr::{read_volatile, write_volatile};
use crate::system::{self, Device, Error};
#[cfg(feature = "rcc_f40_f41")]
use crate::rcc::{RCC, Ahb1Module};
use num_enum::TryFromPrimitive;

#[repr(C)]
#[allow(non_snake_case)]
struct GpioReg {
	MODER: usize,
	OTYPER: usize,
	OSPEEDR: usize,
	PUPDR: usize,
	IDR: usize,
	ODR: usize,
	BSRR: usize,
	LCKR: usize,
	AFRL: usize,
	AFRH: usize
}

pub struct Gpio {
	hw: *mut GpioReg,
	rcc_pos: Ahb1Module
}

impl Gpio {
	pub const unsafe fn at(addr: usize, rcc_pos: Ahb1Module) -> Gpio {
		Gpio {
			hw: addr as *mut _,
			rcc_pos
		}
	}

	pub fn pin_mode(&self, pin: u8, mode: PinMode) {
		let moder = unsafe { read_volatile(&(*self.hw).MODER) };
		let moder = moder & !(0b11 << (pin * 2)) | ((mode as usize) << (pin * 2));
		unsafe {
			write_volatile(&mut (*self.hw).MODER, moder);
		}
	}

	pub fn pin_pull(&self, pin: u8, mode: PullMode) {
		let pupdr = unsafe { read_volatile(&(*self.hw).PUPDR) };
		let pupdr = pupdr & !(0b11 << (pin * 2)) | ((mode as usize) << (pin * 2));
		unsafe {
			write_volatile(&mut (*self.hw).PUPDR, pupdr);
		}
	}

	pub fn write(&self, pin: u8, state: bool) {
		unsafe {
			write_volatile(
				&mut (*self.hw).BSRR,
				(1 << pin) << if state { 0 } else { 16 }
			);
		}
	}

	pub fn read(&self, pin: u8) -> bool {
		unsafe {
			(read_volatile(&(*self.hw).IDR) >> pin) & 1 != 0
		}
	}
}

impl Device for Gpio {
	type Method = Method;

	#[cfg(feature = "rcc_f40_f41")]
	fn switch(&self, state: bool) {
		RCC.switch_ahb1(self.rcc_pos, state);
	}

	fn method(&self, method: Method, a: isize, b: isize) -> system::Result {
		use Method::*;

		if (0..16).contains(&a) {
			let pin = a as u8;
			match method {
				PinMode => {
					let pin_mode = b as usize & 0b11;
					let pull = (b >> 2) as usize & 0b11;
					let _alternative = (b >> 4) as usize & 0xf;
					self.pin_mode(pin, pin_mode.try_into().unwrap());
					self.pin_pull(pin, pull.try_into().unwrap());
					Ok(0)
				}
				DigitalWrite => {
					self.write(a as u8, b != 0);
					Ok(0)
				}
				DigitalRead => Ok(self.read(a as u8) as isize)
			}
		} else {
			Err(Error::NoDevice)
		}
	}
}

#[derive(TryFromPrimitive)]
#[repr(usize)]
pub enum Method {
	PinMode = 0,
	DigitalWrite = 1,
	DigitalRead = 2
}

#[derive(TryFromPrimitive)]
#[repr(usize)]
pub enum PinMode {
	Input = 0b00,
	Output = 0b01,
	Alternative = 0b10,
	Analog = 0b11
}

#[derive(TryFromPrimitive)]
#[repr(usize)]
/// Controls pull-up and pull-down resistors
pub enum PullMode {
	/// No pull resistors
	None = 0b00,

	/// Pull-up resistor
	PullUp = 0b01,

	/// Pull-down resistor
	PullDown = 0b10
}

#[cfg(feature = "gpio_a")]
pub const GPIOA: Gpio = unsafe { Gpio::at(0x4002_0000, Ahb1Module::GPIOA) };

#[cfg(feature = "gpio_b")]
pub const GPIOB: Gpio = unsafe { Gpio::at(0x4002_0400, Ahb1Module::GPIOB) };

#[cfg(feature = "gpio_c")]
pub const GPIOC: Gpio = unsafe { Gpio::at(0x4002_0800, Ahb1Module::GPIOC) };

pub const INDEXED: [Option<&'static Gpio>; 3] = [
	#[cfg(feature = "gpio_a")]
	Some(&GPIOA),
	#[cfg(not(feature = "gpio_a"))]
	None,

	#[cfg(feature = "gpio_b")]
	Some(&GPIOB),
	#[cfg(not(feature = "gpio_b"))]
	None,

	#[cfg(feature = "gpio_c")]
	Some(&GPIOC),
	#[cfg(not(feature = "gpio_c"))]
	None,
];
