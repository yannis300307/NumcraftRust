/// Give a random u32.
pub fn get_random_u32() -> u32 {
    unsafe { eadk_random() }
}

/// Give a random u64.
pub fn get_random_u64() -> u64 {
    unsafe { (eadk_random() as u64) << 32 | (eadk_random() as u64) }
}

/// Give a random u16.
pub fn get_random_u16() -> u16 {
    unsafe { (eadk_random() >> 16) as u16 }
}

/// Give a random u8.
pub fn get_random_u8() -> u8 {
    unsafe { (eadk_random() >> 24) as u8 }
}

/// Give a random f32 between 0 and 1.
pub fn get_random_decimal() -> f32 {
    unsafe { eadk_random() as f32 / u32::MAX as f32 }
}

/// Give a random number between start (included) and end (excluded).
pub fn get_random_in_range(start: u32, stop: u32) -> u32 {
    assert!(start <= stop, "Start must be smaller than stop");
    let mut random_u32 = unsafe { eadk_random() };
    random_u32 %= stop - start;
    random_u32 += start;
    random_u32
}

unsafe extern "C" {
    fn eadk_random() -> u32;
}
