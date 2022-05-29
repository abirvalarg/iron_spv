#[cfg(feature = "heap")]
use core::ptr::addr_of_mut;

#[no_mangle]
#[cold]
unsafe extern "C" fn _reset() -> ! {
	extern "C" {
		fn _init_mem();
		fn main();
	}
	#[cfg(feature = "heap")]
	extern "C" {
		fn _init_heap(start: *mut usize, end: *mut usize);

		static mut _HEAP_START: usize;
		static mut _HEAP_END: usize;
	}

	_init_mem();

	#[cfg(feature = "heap")]
	_init_heap(addr_of_mut!(_HEAP_START), addr_of_mut!(_HEAP_END));

	#[cfg(feature = "init_on_start")]
	{
		#[cfg(feature = "gpio_a")]
		crate::gpio::GPIOA.switch(true);

		#[cfg(feature = "gpio_b")]
		crate::gpio::GPIOB.switch(true);

		#[cfg(feature = "gpio_c")]
		crate::gpio::GPIOC.switch(true);
	}

	// unprivileged mode
	#[cfg(target_arch = "arm")]
	core::arch::asm!(
		"msr CONTROL, {}",
		in(reg) 1
	);

	main();
	loop {}
}

#[no_mangle]
extern "C" fn _NMI() {
	panic!();
}

#[no_mangle]
extern "C" fn _hardfault() {
	panic!();
}
