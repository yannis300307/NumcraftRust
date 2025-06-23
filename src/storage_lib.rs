use alloc::ffi;
use alloc::vec::Vec;

pub fn storage_file_write(filename: &str, content: &[u8]) -> bool {
    let c_string = ffi::CString::new(filename).unwrap();
    unsafe { extapp_fileWrite(c_string.as_ptr(), content.as_ptr(), content.len()) }
}

pub fn storage_extapp_fileExists(filename: &str) -> bool {
    let c_string = ffi::CString::new(filename).unwrap();
    unsafe { extapp_fileExists(c_string.as_ptr()) }
}

pub fn storage_extapp_fileRead(filename: &str) -> Option<Vec<u8>> {
    let c_string = ffi::CString::new(filename).unwrap();
    let mut lenght: usize = 0;
    let array_pointer = unsafe { extapp_fileRead(c_string.as_ptr(), &mut lenght as *mut usize) };

    if array_pointer.is_null() {
        return None;
    }

    Some(unsafe { core::slice::from_raw_parts(array_pointer, lenght).to_vec() })
}

unsafe extern "C" {
    fn extapp_fileWrite(filename: *const u8, content: *const u8, len: usize) -> bool;
    fn extapp_fileExists(filename: *const u8) -> bool;
    fn extapp_fileRead(filename: *const u8, len: *mut usize) -> *const u8;

}
