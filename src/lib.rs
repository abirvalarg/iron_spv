#![no_std]
#![no_main]
#![allow(dead_code)]

#[cfg(not(any(target_arch = "arm", feature = "i_know_i_am_not_using_arm")))]
compile_error!("are you sure you are building this for a right architecture?");

use core::panic::PanicInfo;

#[cfg(feature = "default_panic")]
#[panic_handler]
/// default panic handler, simply puts CPU into infinite loop
fn __panic(_info: &PanicInfo) -> ! {
	loop {
		unsafe {
			core::arch::asm!("wfi");
		}
	}
}

mod init;

#[cfg(feature = "heap")]
mod heap;

mod priv_sync;

#[cfg(feature = "rcc_f40_f41")]
mod rcc {
	mod rcc_f40_f41;
	pub use rcc_f40_f41::*;
}

#[cfg(feature = "gpio_f4")]
mod gpio {
	mod gpio_f4;
	pub use gpio_f4::*;
}

pub mod syscall;

pub mod prelude;
