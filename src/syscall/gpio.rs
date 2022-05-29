use super::Error;

pub fn pin_mode(gpio: isize, pin: isize, mode: isize) -> isize {
	let gpio = gpio as usize;
	let pin = pin as u8;
	if gpio < crate::gpio::INDEXED.len() && (0..16).contains(&pin) {
		match crate::gpio::INDEXED[gpio] {
			Some(gpio) => {
				let pin_mode = mode as usize & 0b11;
				let pull = (mode >> 2) as usize & 0b11;
				let _alternative = (mode >> 4) as usize & 0xf;
				match (pin_mode.try_into(), pull.try_into()) {
					(Ok(mode), Ok(pull)) => {
						gpio.pin_mode(pin, mode);
						gpio.pin_pull(pin, pull);
						0
					}
					_ => Error::BadValue as isize
				}
			}
			None => Error::NoDevice as isize
		}
	} else {
		Error::NoDevice as isize
	}
}

pub fn digital_write(gpio: isize, pin: isize, value: isize) -> isize {
	let gpio = gpio as usize;
	if gpio < crate::gpio::INDEXED.len() && (0..16).contains(&pin) {
		match crate::gpio::INDEXED[gpio] {
			Some(gpio) => {
				gpio.write(pin as u8, value != 0);
				0
			}
			None => Error::NoDevice as isize
		}
	} else {
		Error::NoDevice as isize
	}
}

pub fn digital_read(gpio: isize, pin: isize, _: isize) -> isize {
	let gpio = gpio as usize;
	if gpio < crate::gpio::INDEXED.len() && (0..16).contains(&pin) {
		match crate::gpio::INDEXED[gpio] {
			Some(gpio) => if gpio.read(pin as u8) { 1 } else { 0 },
			None => Error::NoDevice as isize
		}
	} else {
		Error::NoDevice as isize
	}
}
