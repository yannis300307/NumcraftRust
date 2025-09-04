use core::ffi::CStr;

#[cfg(target_os = "none")]
use alloc::{ffi, string::String, vec::Vec};

#[cfg(target_os = "none")]
pub fn storage_file_write(filename: &str, content: &[u8]) -> bool {
    let c_string = ffi::CString::new(filename).unwrap();
    unsafe { extapp_fileWrite(c_string.as_ptr(), content.as_ptr(), content.len()) }
}

#[cfg(not(target_os = "none"))]
pub fn storage_file_write(filename: &str, content: &[u8]) -> bool {
    true
}

#[cfg(target_os = "none")]
pub fn storage_extapp_file_exists(filename: &str) -> bool {
    let c_string = ffi::CString::new(filename).unwrap();
    unsafe { extapp_fileExists(c_string.as_ptr()) }
}

#[cfg(not(target_os = "none"))]
pub fn storage_extapp_file_exists(filename: &str) -> bool {
    false
}

#[cfg(target_os = "none")]
pub fn storage_extapp_file_read(filename: &str) -> Option<Vec<u8>> {
    let c_string = ffi::CString::new(filename).unwrap();
    let mut lenght: usize = 0;
    let array_pointer = unsafe { extapp_fileRead(c_string.as_ptr(), &mut lenght as *mut usize) };

    if array_pointer.is_null() {
        return None;
    }

    Some(unsafe { core::slice::from_raw_parts(array_pointer, lenght).to_vec() })
}

#[cfg(not(target_os = "none"))]
pub fn storage_extapp_file_read(filename: &str) -> Option<Vec<u8>> {
    None
}

#[cfg(target_os = "none")]
pub fn storage_extapp_file_read_header(filename: &str, header_len: usize) -> Option<Vec<u8>> {
    let c_string = ffi::CString::new(filename).unwrap();
    let mut _lenght: usize = 0;
    let array_pointer = unsafe { extapp_fileRead(c_string.as_ptr(), &mut _lenght as *mut usize) };

    if array_pointer.is_null() {
        return None;
    }

    Some(unsafe { core::slice::from_raw_parts(array_pointer, header_len).to_vec() })
}

#[cfg(not(target_os = "none"))]
pub fn storage_extapp_file_read_header(filename: &str, header_len: usize) -> Option<Vec<u8>> {
    None
}

#[cfg(target_os = "none")]
pub fn storage_extapp_file_erase(filename: &str) -> bool {
    let c_string = ffi::CString::new(filename).unwrap();
    unsafe { extapp_fileErase(c_string.as_ptr()) }
}

#[cfg(not(target_os = "none"))]
pub fn storage_extapp_file_erase(filename: &str) -> bool {
    true
}

#[cfg(target_os = "none")]
pub fn storage_extapp_file_list_with_extension(max_records: usize, extension: &str) -> Vec<String> {
    let mut filenames: Vec<*mut u8> = Vec::with_capacity(max_records);
    let c_string = ffi::CString::new(extension).unwrap();

    unsafe {
        let final_len = extapp_fileListWithExtension(
            filenames.as_mut_slice().as_mut_ptr(),
            max_records as isize,
            c_string.as_ptr(),
        );
        filenames.set_len(final_len as usize);

        let mut files: Vec<String> = Vec::new();
        for name_ptr in filenames {
            if !name_ptr.is_null() {
                let name = CStr::from_ptr(name_ptr).to_string_lossy().into_owned();
                files.push(name);
            }
        }

        files
    }
}

#[cfg(not(target_os = "none"))]
pub fn storage_extapp_file_list_with_extension(max_records: usize, extension: &str) -> Vec<String> {
    Vec::new()
}

#[cfg(target_os = "none")]
unsafe extern "C" {
    fn extapp_fileWrite(filename: *const u8, content: *const u8, len: usize) -> bool;
    fn extapp_fileExists(filename: *const u8) -> bool;
    fn extapp_fileRead(filename: *const u8, len: *mut usize) -> *const u8;
    fn extapp_fileErase(filename: *const u8) -> bool;
    fn extapp_fileListWithExtension(
        filename: *mut *mut u8,
        maxrecord: isize,
        extension: *const u8,
    ) -> isize;

}
