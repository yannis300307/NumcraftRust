calc_use!(core::ffi::CStr);
calc_use!(alloc::ffi);
calc_use!(alloc::string::String);
calc_use!(alloc::vec::Vec);

sim_use!(std::fs);

/// Write a binary file to the records.
pub fn file_write(filename: &str, content: &[u8]) -> Option<()> {
    #[cfg(target_os = "none")]
    {
        let c_string = ffi::CString::new(filename).unwrap();

        #[cfg(feature = "epsilon")]
        let result =
            unsafe { extapp_fileWrite(c_string.as_ptr(), content.as_ptr(), content.len()) };

        #[cfg(feature = "upsilon")]
        let result =
            unsafe { extapp_fileWrite(c_string.as_ptr(), content.as_ptr(), content.len(), 0) };

        if result { Some(()) } else { None }
    }
    #[cfg(not(target_os = "none"))]
    {
        if !fs::exists("simulator/storage").ok()? {
            fs::create_dir_all("simulator/storage").ok()?;
        }
        fs::write(format!("simulator/storage/{}", filename), content).ok()
    }
}

/// Check if a file is preset in the records. On real hardware, it is possible to have multiple files with the same name.
pub fn file_exists(filename: &str) -> bool {
    #[cfg(target_os = "none")]
    {
        let c_string = ffi::CString::new(filename).unwrap();
        #[cfg(feature = "epsilon")]
        unsafe {
            extapp_fileExists(c_string.as_ptr())
        }

        #[cfg(feature = "upsilon")]
        unsafe {
            extapp_fileExists(c_string.as_ptr(), 0)
        }
    }
    #[cfg(not(target_os = "none"))]
    {
        fs::exists(format!("simulator/storage/{}", filename)).unwrap()
    }
}

/// Read a file and return its content. Needs an allocator to function properly.
pub fn file_read(filename: &str) -> Option<Vec<u8>> {
    #[cfg(target_os = "none")]
    {
        let c_string = ffi::CString::new(filename).unwrap();
        let mut lenght: usize = 0;

        #[cfg(feature = "epsilon")]
        let array_pointer =
            unsafe { extapp_fileRead(c_string.as_ptr(), &mut lenght as *mut usize) };

        #[cfg(feature = "upsilon")]
        let array_pointer =
            unsafe { extapp_fileRead(c_string.as_ptr(), &mut lenght as *mut usize, 0) };

        if array_pointer.is_null() {
            return None;
        }

        Some(unsafe { core::slice::from_raw_parts(array_pointer, lenght).to_vec() })
    }
    #[cfg(not(target_os = "none"))]
    {
        fs::read(format!("simulator/storage/{}", filename)).ok()
    }
}

/// Read a part of a file. Needs an allocator to function properly.
pub fn file_read_slice(filename: &str, start: usize, mut slice_lenght: usize) -> Option<Vec<u8>> {
    #[cfg(target_os = "none")]
    {
        let c_string = ffi::CString::new(filename).unwrap();
        let mut lenght: usize = 0;

        #[cfg(feature = "epsilon")]
        let array_pointer = unsafe {
            extapp_fileRead(c_string.as_ptr(), &mut lenght as *mut usize).offset(start as isize)
        };

        #[cfg(feature = "upsilon")]
        let array_pointer = unsafe {
            extapp_fileRead(c_string.as_ptr(), &mut lenght as *mut usize, 0).offset(start as isize)
        };

        if array_pointer.is_null() {
            return None;
        }

        if lenght < slice_lenght {
            slice_lenght = lenght;
        }

        Some(unsafe { core::slice::from_raw_parts(array_pointer, slice_lenght).to_vec() })
    }
    #[cfg(not(target_os = "none"))]
    {
        fs::read(format!("simulator/storage/{}", filename))
            .map(|v| v[start..(start + slice_lenght)].to_vec())
            .ok()
    }
}

/// Remove a file from the records if it exists.
pub fn file_erase(filename: &str) -> Option<()> {
    #[cfg(target_os = "none")]
    {
        let c_string = ffi::CString::new(filename).unwrap();

        #[cfg(feature = "epsilon")]
        let result = unsafe { extapp_fileErase(c_string.as_ptr()) };

        #[cfg(feature = "upsilon")]
        let result = unsafe { extapp_fileErase(c_string.as_ptr(), 0) };

        if result { Some(()) } else { None }
    }
    #[cfg(not(target_os = "none"))]
    {
        fs::remove_file(format!("simulator/storage/{}", filename)).ok()
    }
}

/// Return an array containing the name of all the records.
pub fn file_list_with_extension(max_records: usize, extension: &str) -> Vec<String> {
    #[cfg(target_os = "none")]
    {
        let mut filenames: Vec<*mut u8> = Vec::with_capacity(max_records);
        let c_string = ffi::CString::new(extension).unwrap();

        unsafe {
            #[cfg(feature = "epsilon")]
            let final_len = extapp_fileListWithExtension(
                filenames.as_mut_slice().as_mut_ptr(),
                max_records as isize,
                c_string.as_ptr(),
            );
            #[cfg(feature = "upsilon")]
            let final_len = extapp_fileListWithExtension(
                filenames.as_mut_slice().as_mut_ptr(),
                max_records as isize,
                c_string.as_ptr(),
                0,
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
    {
        let mut files: Vec<String> = Vec::new();
        for entry in fs::read_dir("simulator/storage").unwrap() {
            if files.len() < max_records {
                let entry = entry.unwrap();
                let name = entry.file_name().into_string().unwrap();
                if name.ends_with(extension) {
                    files.push(name);
                }
            }
        }

        files
    }
}

pub enum CalculatorModel {
    Unknown,
    EpsilonN0110N0115,
    EpsilonN0120,
    Upsilon,
    Simulator,
}

/// Return the model name of the calculator or Simulator on the Simulator.
pub fn get_calculator_model() -> CalculatorModel {
    #[cfg(target_os = "none")]
    #[cfg(feature = "epsilon")]
    {
        match unsafe { extapp_calculatorModel() } {
            _ => CalculatorModel::Unknown,
            1 => CalculatorModel::EpsilonN0110N0115,
            2 => CalculatorModel::EpsilonN0120,
        }
    }
    #[cfg(target_os = "none")]
    #[cfg(feature = "upsilon")]
    {
        CalculatorModel::Upsilon
    }
    #[cfg(not(target_os = "none"))]
    {
        CalculatorModel::Simulator
    }
}

#[cfg(target_os = "none")]
#[cfg(feature = "epsilon")]
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
    fn extapp_calculatorModel() -> u8;
}

#[cfg(target_os = "none")]
#[cfg(feature = "upsilon")]
unsafe extern "C" {
    fn extapp_fileWrite(
        filename: *const u8,
        content: *const u8,
        len: usize,
        storage: isize,
    ) -> bool;
    fn extapp_fileExists(filename: *const u8, storage: isize) -> bool;
    fn extapp_fileRead(filename: *const u8, len: *mut usize, storage: isize) -> *const u8;
    fn extapp_fileErase(filename: *const u8, storage: isize) -> bool;
    fn extapp_fileListWithExtension(
        filename: *mut *mut u8,
        maxrecord: isize,
        extension: *const u8,
        storage: isize,
    ) -> isize;
}
