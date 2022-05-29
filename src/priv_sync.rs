use core::arch::asm;

pub fn exec_sync<T: FnOnce() -> R, R>(func: T) -> R {
	unsafe {
		let masked: usize;
		asm!("mrs {}, PRIMASK", out(reg) masked);
		asm!("cpsid i");
		let res = func();
		if masked == 0 {
			asm!("cpsie i");
		}
		res
	}
}
