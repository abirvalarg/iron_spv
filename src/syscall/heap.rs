extern "C" {
	fn _alloc(size: usize, align: usize) -> *mut u8;
	fn _free(mem: *mut u8);
}

pub fn alloc(size: isize, align: isize, _: isize) -> isize {
	unsafe {
		_alloc(size as usize, align as usize) as isize
	}
}

pub fn free(ptr: isize, _: isize, _: isize) -> isize {
	unsafe {
		_free(ptr as *mut u8);
	}
	0
}
