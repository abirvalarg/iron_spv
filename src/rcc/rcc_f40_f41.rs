use core::ptr::{read_volatile, write_volatile};

#[repr(C)]
#[allow(non_snake_case)]
struct RccReg {
	CR: usize,
	PLLCFGR: usize,
	CFGR: usize,
	CIR: usize,
	AHB1RSTR: usize,
	AHB2RSTR: usize,
	AHB3RSTR: usize,
	_res0: usize,
	APB1RSTR: usize,
	APB2RSTR: usize,
	_res1: usize,
	_res2: usize,
	AHB1ENR: usize,
	AHB2ENR: usize,
	AHB3ENR: usize,
	_res3: usize,
	APB1ENR: usize,
	APB2ENR: usize,
	_res4: usize,
	_res5: usize
}

#[repr(transparent)]
pub struct Rcc(*mut RccReg);

impl Rcc {
	pub const unsafe fn at(addr: usize) -> Self {
		Rcc(addr as *mut _)
	}

	pub fn switch_ahb1(&self, pos: Ahb1Module, state: bool) {
		let mask = 1 << (pos as u8);
		unsafe {
			let enr = read_volatile(&(*self.0).AHB1ENR);
			let enr = if state {
				enr | mask
			} else {
				enr & !mask
			};
			write_volatile(&mut (*self.0).AHB1ENR, enr);
		}
	}

	pub fn switch_apb1(&self, pos: Apb1Module, state: bool) {
		let mask = 1 << (pos as u8);
		unsafe {
			let enr = read_volatile(&(*self.0).APB1ENR);
			let enr = if state {
				enr | mask
			} else {
				enr & !mask
			};
			write_volatile(&mut (*self.0).APB1ENR, enr);
		}
	}

	pub fn switch_apb2(&self, pos: Apb2Module, state: bool) {
		let mask = 1 << (pos as u8);
		unsafe {
			let enr = read_volatile(&(*self.0).APB2ENR);
			let enr = if state {
				enr | mask
			} else {
				enr & !mask
			};
			write_volatile(&mut (*self.0).APB2ENR, enr);
		}
	}
}

#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum Ahb1Module {
    GPIOA = 0,
    GPIOB = 1,
    GPIOC = 2,
    DMA1 = 21,
    DMA2 = 22,
}

#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum Apb1Module {
    TIM3 = 1,
    TIM4 = 2,
    TIM6 = 4,
    TIM7 = 5,
    USART2 = 17,
    USART3 = 18
}

#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum Apb2Module {
    TIM1 = 0,
    USART1 = 4,
    ADC1 = 8,
    ADC2 = 9,
    ADC3 = 10,
    SPI1 = 12,
    SYSCFG = 14
}

pub const RCC: Rcc = unsafe { Rcc::at(0x4002_3800) };
