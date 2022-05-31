//! Everything about system calls

use core::sync::atomic::Ordering;
use core::sync::atomic::AtomicIsize;

pub use crate::Error;

#[cfg(feature = "gpio")]
mod gpio;
#[cfg(feature = "heap")]
mod heap;

#[cfg(target_arch = "arm")]
/// Make a system call. Parameters `a`, `b` and `c` are passed to
/// the corresponding system function. Returns the value from that
/// function. Negative values mean that an error occured unless
/// stated otherwise in call description(see [`SysCall`])
pub fn syscall(call: SysCall, a: isize, b: isize, c: isize) -> isize {
	unsafe {
		core::arch::asm!(
			"svc #0",
			in("r0") call as usize,
			in("r1") a,
			in("r2") b,
			in("r3") c,
		);
	}
	SYS_RETURN.load(Ordering::Relaxed)
}

static SYS_RETURN: AtomicIsize = AtomicIsize::new(0);

#[no_mangle]
extern "C" fn _SVCall(call: usize, a: isize, b: isize, c: isize) {
	let result = if call < SYSTEM_VECTOR.len() {
		SYSTEM_VECTOR[call](a, b, c)
	} else {
		Error::NoFunction as isize
	};
	SYS_RETURN.store(result, Ordering::Relaxed);
}

type SysFunc = fn(isize, isize, isize) -> isize;

iron_spv_macros::make_system_vector!(
	#[cfg(feature = "gpio")]
	/** set pin mode.
	 * `a`: controller. 0 - GPIOA, 1 - GPIOB, etc.
	 * `b`: pin, 0-15
	 * `c`: mode

	 `mode` is composed of multiple bit fields, values correspond
	 to hardware settings from the reference manual
	 * bits[0..=1]: I/O mode
	 * bits[2..=3]: Pull mode
	 * bits[4..=7]: Number of alternative function

	 [`iron_spv::prelude::PinMode`](../prelude/enum.PinMode.html)
	 can be converted into this bitfield with it's `into` method
	 
	 Returns `0` on success.
	 */
	PinMode => gpio::pin_mode,

	#[cfg(feature = "gpio")]
	/** Write a value to the pin.
	 * `a`: controller. 0 - GPIOA, 1 - GPIOB, etc.
	 * `b`: pin, 0-15
	 * `c`: value. Non-zero for high level

	 Returns `0` on success.
	 */
	DigitalWrite => gpio::digital_write,

	#[cfg(feature = "gpio")]
	/** Read a logical level from the pin
	 * `a`: controller. 0 - GPIOA, 1 - GPIOB, etc.
	 * `b`: pin, 0-15
	 * ~~`c`: ignored~~

	 On success returns `0` or `1` depending on logical level
	 on the pin
	*/
	DigitalRead => gpio::digital_read,

	#[cfg(feature = "heap")]
	/** Allocate a piece of memory on heap.

	 Intended for internal use, use `alloc` crare instead.
	 */
	Alloc => heap::alloc,

	#[cfg(feature = "heap")]
	/** Deallocate a piece of memory on heap
	 
	 Intended for internal use, use `alloc` crare instead.
	*/
	Free => heap::free,
);
