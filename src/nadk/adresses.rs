/// Return the size of the heap.
#[inline(always)]
pub fn heap_size() -> usize {
    #[cfg(feature = "epsilon")]
    {
        100_000
    }

    #[cfg(feature = "upsilon")]
    {
        unsafe {_heap_size as usize}
    }
}

/// Return the start adress of the heap
#[inline(always)]
pub fn heap_start() -> usize {
    #[cfg(feature = "upsilon")]
    unsafe {
        _heap_base as usize
    }

    #[cfg(feature = "epsilon")]
    {
        core::ptr::addr_of_mut!(_heap_start) as usize
    }
}

#[cfg(feature = "upsilon")]
unsafe extern "C" {
    pub static mut _heap_base: *mut u8;
    pub static mut _heap_size: u32;
}

#[cfg(feature = "epsilon")]
unsafe extern "C" {
    pub static mut _heap_start: u8;
    pub static mut _heap_end: u8;
}

#[cfg(feature = "upsilon")]
#[unsafe(no_mangle)]
#[inline(always)]
pub extern "C" fn setjmp(_: u32) -> u32 {
    return 0;
}

#[cfg(feature = "upsilon")]
#[unsafe(no_mangle)]
#[inline(always)]
pub extern "C" fn longjmp(_: u32, _: u32) {}

#[cfg(feature = "upsilon")]
#[unsafe(no_mangle)]
#[inline(always)]
pub extern "C" fn extapp_main() {
    crate::main();
}
