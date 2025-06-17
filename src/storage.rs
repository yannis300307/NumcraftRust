pub fn get_extapp_size() -> u32 {
    unsafe {
        extapp_size()
    }
}

unsafe extern "C" {
    pub fn extapp_size() -> u32;
}