/// Wait for the given time in micro seconds. This functions seems not implemented on the simulator so it is replaced by eadk_timing_msleep instead.
pub fn wait_microseconds(delay: u32) {
    #[cfg(target_os = "none")]
    unsafe {
        eadk_timing_usleep(delay);
    }
    #[cfg(not(target_os = "none"))]
    unsafe {
        eadk_timing_msleep(delay / 1000);
    }
}

/// Wait for the given delay in seconds. The precision is 0.001 second.
pub fn wait_seconds(delay: f32) {
    unsafe {
        eadk_timing_msleep((delay * 1000.) as u32);
    }
}

/// Wait for the given delay in milliseconds.
pub fn wait_milliseconds(delay: u32) {
    unsafe {
        eadk_timing_msleep(delay);
    }
}

/// Return the time since the startup of the calculator in milliseconds. (No warranty)
pub fn get_current_time_millis() -> u64 {
    unsafe { eadk_timing_millis() }
}

/// Return the time since the startup of the calculator in seconds. (No warranty)
pub fn get_current_time_seconds() -> f32 {
    unsafe { eadk_timing_millis() as f32 * 0.001}
}

unsafe extern "C" {
    fn eadk_timing_usleep(delay: u32);
    fn eadk_timing_msleep(delay: u32);
    fn eadk_timing_millis() -> u64;
}
