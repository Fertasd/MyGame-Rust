use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};

pub struct AtomicFlag {
	flag: AtomicBool
}

pub const ATOMIC_FLAG_INIT: AtomicFlag = AtomicFlag { flag: ATOMIC_BOOL_INIT };

impl AtomicFlag {
	pub fn test_and_set(&self) -> bool {
		self.flag.compare_and_swap(false, true, Ordering::Acquire)
	}

	pub fn clear(&self) {
		self.flag.store(false, Ordering::Release)
	}
}
