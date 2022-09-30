use crate::MemoryMapCore;
use crate::MemoryMapReadOnlyCore;
use core::ffi::c_void;
use core::fmt::Debug;
use windows::Win32::Foundation::*;
use windows::Win32::Storage::FileSystem::*;
use windows::Win32::System::Memory::*;

#[derive(Debug)]
pub struct MemoryMapped {
    pub(crate) file_handle: HANDLE,
    pub(crate) mapping_handle: HANDLE,
    pub(crate) addr: *mut c_void,
    pub(crate) len: usize,
    pub(crate) path: Option<String>,
}

impl Drop for MemoryMapped {
    fn drop(&mut self) {
        unsafe {
            // if we have modified a memory mapped file, we run a sync before
            // closing
            self.sync_flush().unwrap();

            let res = UnmapViewOfFile(self.addr);
            if !res.as_bool() {
                panic!("Cannot unmap view of file.",);
            }

            let res = CloseHandle(self.mapping_handle);
            if !res.as_bool() {
                panic!("Cannot Close the mapping handle.");
            }

            let res = CloseHandle(self.file_handle);
            if !res.as_bool() {
                panic!("Cannot Close the mapping handle.");
            }
        }
    }
}

impl MemoryMapReadOnlyCore for MemoryMapped {
    fn new<S: AsRef<str> + Debug>(path: S, offset: Option<usize>) -> Result<Self, String> {
        let path = path.as_ref();
        unsafe {
            let file_handle = CreateFileW(
                path,
                FILE_GENERIC_READ,
                FILE_SHARE_NONE, // prevent other processes to modify the file while we are reading it
                std::ptr::null() as _,
                CREATE_ALWAYS,
                FILE_FLAG_RANDOM_ACCESS,
                HANDLE(0),
            );

            if file_handle == INVALID_HANDLE_VALUE {
                return Err("Error opening file CreateFileW".into());
            }

            let mut len_higher: u32 = 0;
            let len_lower = GetFileSize(file_handle, (&mut len_higher) as *mut u32);
            let len = ((len_lower as u64) | (len_higher as u64) << 32) as usize;

            let mapping_handle = CreateFileMappingW(
                file_handle,
                std::ptr::null_mut(),
                PAGE_READWRITE, // | SEC_LARGE_PAGES,
                0,
                0,
                PWSTR(std::ptr::null_mut()),
            );

            if mapping_handle == HANDLE(0) {
                return Err("Error opening file CreateFileMappingW".into());
            }

            let addr = MapViewOfFile(
                mapping_handle,
                FILE_MAP_READ | FILE_MAP_WRITE, // | FILE_MAP_LARGE_PAGES
                0,
                0,
                len,
            );

            if addr == std::ptr::null_mut() as _ {
                return Err("Error opening file MapViewOfFile".into());
            }

            Ok(MemoryMapped {
                file_handle,
                mapping_handle,
                addr,
                len,
                path: Some(path.to_string()),
            })
        }
    }

    fn get_addr(&self) -> *mut u8 {
        self.addr as _
    }

    fn len(&self) -> usize {
        self.len
    }

    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }
}

impl MemoryMapCore for MemoryMapped {
    /// Memory map the file with mutability permissions
    fn new_mut<S: AsRef<str> + Debug>(
        path: Option<S>,
        len: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Self, String> {
        assert!(offset == None, "MMAP offsetting is TODO on windows");
        let (addr, file_handle, mapping_handle, len) = match (path.as_ref(), len) {
            // New file / expand file
            (Some(path), maybe_len) => {
                let (file_handle, len) = if let Some(len) = maybe_len {
                    let file_handle = unsafe {
                        CreateFileW(
                            path.as_ref(),
                            FILE_GENERIC_READ | FILE_GENERIC_WRITE,
                            FILE_SHARE_NONE, // prevent other processes to modify the file while we are reading it
                            0 as _,
                            CREATE_ALWAYS,
                            FILE_FLAG_RANDOM_ACCESS,
                            HANDLE(0),
                        )
                    };

                    if file_handle == INVALID_HANDLE_VALUE {
                        return Err("Error opening file CreateFileW".into());
                    }

                    // seek to the wanted position
                    let res = unsafe {
                        SetFilePointerEx(file_handle, len as _, 0 as _, FILE_BEGIN)
                    };

                    if !res.as_bool() {
                        return Err("Could not Seek to the wanted len in SetFilePointerEx".into());
                    }

                    // truncate at teel position
                    let res = unsafe { SetEndOfFile(file_handle) };

                    if !res.as_bool() {
                        return Err("Could not Truncate the file in SetEndOfFile".into());
                    }

                    (file_handle, len)
                } else {
                    // no len, so we can just load a present file
                    let file_handle = unsafe {
                        CreateFileW(
                            path.as_ref(),
                            FILE_GENERIC_READ | FILE_GENERIC_WRITE,
                            FILE_SHARE_NONE, // prevent other processes to modify the file while we are reading it
                            0 as _,
                            OPEN_EXISTING,
                            FILE_FLAG_RANDOM_ACCESS,
                            HANDLE(0),
                        )
                    };

                    if file_handle == INVALID_HANDLE_VALUE {
                        return Err("Error opening file CreateFileW".into());
                    }

                    let mut len_higher: u32 = 0;
                    let len_lower = unsafe{
                        GetFileSize(file_handle, (&mut len_higher) as *mut u32)
                    };
                    let len = ((len_lower as u64) | (len_higher as u64) << 32) as usize;

                    (file_handle, len)
                };


                let mapping_handle = unsafe{ CreateFileMappingW(
                    file_handle,
                    std::ptr::null_mut(),
                    PAGE_READWRITE, // | SEC_LARGE_PAGES,
                    0,              // max size
                    0,              // max size
                    PWSTR(std::ptr::null_mut()),
                ) };

                if mapping_handle == HANDLE(0) {
                    return Err("Error opening file CreateFileMappingW".into());
                }

                let addr = unsafe{ MapViewOfFile(
                    mapping_handle,
                    FILE_MAP_READ | FILE_MAP_WRITE, // | FILE_MAP_LARGE_PAGES
                    0,                              // offset high
                    0,                              // offset low
                    len,
                ) };
                (addr, file_handle, mapping_handle, len)
            }
            // anonymous
            (None, Some(len)) => {
                todo!();
            }
            (None, None) => {
                return Err("Cannot create an mmap without both a path and a len".to_string());
            }
        };

        if addr == std::ptr::null_mut() as _ {
            return Err("Error opening file MapViewOfFile".into());
        }

        Ok(MemoryMapped {
            file_handle,
            mapping_handle,
            addr,
            len,
            path: path.map(|x| x.as_ref().to_string()),
        })
    }

    fn sync_flush(&self) -> Result<(), String> {
        if self.path.is_some() {
            unsafe {
                let res = FlushViewOfFile(self.addr as _, self.len);
                if !res.as_bool() {
                    return Err("Error syncronously syncing the mmap ".into());
                }
                FlushFileBuffers(self.mapping_handle);
                FlushFileBuffers(self.file_handle);
            }
        }
        Ok(())
    }

    fn async_flush(&self) -> Result<(), String> {
        // No async flushes in Windows :(
        self.sync_flush()
    }
}
