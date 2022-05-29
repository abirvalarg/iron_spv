use core::alloc::GlobalAlloc;
use crate::syscall::{syscall, SysCall};

struct Alloc;

#[global_allocator]
static ALLOC: Alloc = Alloc;

unsafe impl Sync for Alloc {}

unsafe impl GlobalAlloc for Alloc {
	unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
		syscall(SysCall::Alloc, layout.size() as isize, layout.align() as isize, 0) as *mut u8
	}

	unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
		syscall(SysCall::Free, ptr as isize, 0, 0);
	}
}
