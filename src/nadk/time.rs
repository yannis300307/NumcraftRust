/// Wait for the given delay in seconds. The precision is 0.001 second.
pub fn wait_seconds(delay: f32) {
    #[cfg(feature = "epsilon")]
    unsafe {
        eadk_timing_msleep((delay * 1000.) as u32);
    }

    #[cfg(feature = "upsilon")]
    unsafe {
        extapp_msleep((delay * 1000.) as u32)
    }
}

/// Wait for the given delay in milliseconds.
pub fn wait_milliseconds(delay: u32) {
    #[cfg(feature = "epsilon")]
    unsafe {
        eadk_timing_msleep(delay);
    }
    #[cfg(feature = "upsilon")]
    unsafe {
        extapp_msleep(delay);
    }
}

/// Return the time since the startup of the calculator in milliseconds. (No warranty)
pub fn get_current_time_millis() -> u64 {
    #[cfg(feature = "epsilon")]
    unsafe {
        eadk_timing_millis()
    }

    #[cfg(feature = "upsilon")]
    unsafe {
        extapp_millis()
    }
}

/// Return the time since the startup of the calculator in seconds. (No warranty)
pub fn get_current_time_seconds() -> f32 {
    #[cfg(feature = "epsilon")]
    unsafe {
        eadk_timing_millis() as f32 * 0.001
    }

    #[cfg(feature = "upsilon")]
    unsafe {
        extapp_millis() as f32 * 0.001
    }
}

#[cfg(feature = "epsilon")]
unsafe extern "C" {
    fn eadk_timing_msleep(delay: u32);
    fn eadk_timing_millis() -> u64;
}

#[cfg(feature = "upsilon")]
unsafe extern "C" {
    fn extapp_msleep(us: u32);
    fn extapp_millis() -> u64;
}
