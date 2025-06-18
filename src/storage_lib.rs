use alloc::ffi;

pub fn storage_extapp_size() -> u32 {
    unsafe {
        extapp_size()
    }
}

pub fn storage_file_write(filename: &str, content: &[u8]) -> bool {
    let c_string = ffi::CString::new(filename).unwrap();
    unsafe {
        extapp_fileWrite(c_string.as_ptr(), content.as_ptr().sub(1), content.len())
    }
}

unsafe extern "C" {
    fn extapp_size() -> u32;
    fn extapp_fileWrite(filename: *const u8, content: *const u8, len: usize) -> bool;
}