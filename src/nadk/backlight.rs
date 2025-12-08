/// Set the brightness level of the screen's backlight.
#[inline(always)]
pub fn set_brightness(brightness: u8) {
    #[cfg(feature = "epsilon")]
    unsafe {
        eadk_backlight_set_brightness(brightness);
    }
}

/// Return the brightness level of the screen's backlight.
#[inline(always)]
pub fn get_brightness() -> u8 {
    #[cfg(feature = "epsilon")]
    unsafe { eadk_backlight_brightness() }

    #[cfg(feature = "upsilon")]
    0
}

#[cfg(feature = "epsilon")]
unsafe extern "C" {
    fn eadk_backlight_set_brightness(brightness: u8);
    fn eadk_backlight_brightness() -> u8;
}
