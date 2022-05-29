use crate::syscall::{self, syscall};
use crate::gpio::PinMode as InternalPinMode;

pub use crate::gpio::PullMode;

/// GPIO pin mode. Yes, so simple!
pub enum PinMode {
	/// Input mode, requires a pull mode
	Input(PullMode),

	/// Output mode.
	Output
}

impl Into<isize> for PinMode {
	/// Converts this enum into a bitfield for
	/// [`PinMode`](../syscall/enum.SysCall.html#variant.PinMode)
	/// system call
	fn into(self) -> isize {
		match self {
			PinMode::Input(pull) => {
				InternalPinMode::Input as isize
				| ((pull as isize) << 2)
			}
			PinMode::Output => InternalPinMode::Output as isize
		}
	}
}

/** set pin mode.
 * `controller`: 0 - GPIOA, 1 - GPIOB, etc.
 * `pin`: 0-15
 * `mode`: see [`PinMode`]

 Panics on errors.
 */
pub fn pin_mode(controller: u8, pin: u8, mode: PinMode) {
	let res = syscall(syscall::SysCall::PinMode, controller as isize, pin as isize, mode.into());
	if res != 0 {
		panic!();
	}
}

/** Write a value to the pin.
 * `a`: controller. 0 - GPIOA, 1 - GPIOB, etc.
 * `b`: pin, 0-15
 * `c`: value. Non-zero for high level

 Panics on errors.
*/
pub fn digital_write(controller: u8, pin: u8, state: bool) {
	let res = syscall(syscall::SysCall::DigitalWrite, controller as isize, pin as isize, state as isize);
	if res != 0 {
		panic!();
	}
}

/** Read a logical level from the pin
 * `controller`: 0 - GPIOA, 1 - GPIOB, etc.
 * `pin`: 0-15

 Panics on errors.
*/
pub fn digital_read(controller: u8, pin: u8) -> bool {
	let res = syscall(syscall::SysCall::DigitalRead, controller as isize, pin as isize, 0);
	if res < 0 || res > 1 {
		panic!();
	}
	res != 0
}
