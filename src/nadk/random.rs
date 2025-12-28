static mut COUNTER: u64 = 0;

/// Give a random u32.
#[inline(always)]
pub fn get_random_u32() -> u32 {
    #[cfg(feature = "epsilon")]
    unsafe {
        eadk_random()
    }
    #[cfg(feature = "upsilon")]
    {
        (get_random_u64() >> 32) as u32
    }
}

/// Give a random u64.
#[inline(always)]
pub fn get_random_u64() -> u64 {
    #[cfg(feature = "epsilon")]
    unsafe {
        (eadk_random() as u64) << 32 | (eadk_random() as u64)
    }

    #[cfg(feature = "upsilon")]
    {
        // Wait! yes, I'm aware that accessing mutable static variables is unsafe BUT the Numworks calculator has a single thread so no multi threading.
        let mut nbr: u64 = unsafe { extapp_millis() + COUNTER << 8 };
        nbr ^= nbr << 13;
        nbr ^= nbr >> 7;
        nbr ^= nbr << 17;

        nbr
    }
}

/// Give a random u16.
#[inline(always)]
pub fn get_random_u16() -> u16 {
    #[cfg(feature = "epsilon")]
    unsafe {
        (eadk_random() >> 16) as u16
    }

    #[cfg(feature = "upsilon")]
    {
        (get_random_u64() >> 48) as u16
    }
}

/// Give a random u8.
#[inline(always)]
pub fn get_random_u8() -> u8 {
    #[cfg(feature = "epsilon")]
    unsafe {
        (eadk_random() >> 24) as u8
    }

    #[cfg(feature = "upsilon")]
    {
        (get_random_u64() >> 56) as u8
    }
}

/// Give a random f32 between 0 and 1.
pub fn get_random_decimal() -> f32 {
    unsafe {
        get_random_u32() as f32 / u32::MAX as f32
    }
}

/// Give a random number between start (included) and end (excluded).
pub fn get_random_in_range(start: u32, stop: u32) -> u32 {
    assert!(start < stop, "Start must be smaller than stop");
    let mut random_u32 = unsafe { get_random_u32() };
    random_u32 %= stop - start;
    random_u32 += start;
    random_u32
}

#[cfg(feature = "epsilon")]
unsafe extern "C" {
    fn eadk_random() -> u32;
}

#[cfg(feature = "upsilon")]
unsafe extern "C" {
    fn extapp_millis() -> u64;
}
