unsafe extern "C" {
    pub static mut _heap_start: u8;
    pub static mut _heap_end: u8;
}

pub static mut HEAP_START: *mut u8 = core::ptr::addr_of_mut!(_heap_start);
pub static mut HEAP_END: *mut u8 = core::ptr::addr_of_mut!(_heap_end);

/// Return the size of the heap.
pub fn heap_size() -> usize {
    100_000
    //(unsafe { HEAP_END.offset_from(HEAP_START) }) as usize
}
