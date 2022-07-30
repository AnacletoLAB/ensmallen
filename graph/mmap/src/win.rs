use core::ffi::c_void;
use windows::Win32::Foundation::*;
use windows::Win32::Storage::FileSystem::*;
use windows::Win32::System::Memory::*;

#[derive(Debug)]
pub struct MemoryMapped {
    file_handle: HANDLE,
    mapping_handle: HANDLE,
    addr: *mut c_void,
    len: usize,
    path: Option<String>,
}

impl Drop for MemoryMapped {
    fn drop(&mut self) {
        unsafe {
            // if we have modified a memory mapped file, we run a sync before
            // closing
            if self.fd.is_some() {
                self.sync_flush().unwrap();
            }

            let res = UnmapViewOfFile(self.addr);
            if res == BOOL(0) {
                panic!("Cannot unmap view of file.",);
            }

            let res = CloseHandle(self.mapping_handle);
            if res == BOOL(0) {
                panic!("Cannot Close the mapping handle.");
            }

            let res = CloseHandle(self.file_handle);
            if res == BOOL(0) {
                panic!("Cannot Close the mapping handle.");
            }
        }
    }
}

impl MemoryMapped {
    pub fn new(path: &str) -> Result<MemoryMapped, String> {
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
            })
        }
    }

    pub fn sync_flush(&self) -> Result<(), String> {
        if self.fd.is_some() {
            unsafe {
                let res = FlushViewOfFile(self.addr as _, self.len);
                if res == -1 {
                    return Err("Error syncronously syncing the mmap ".into());
                }
                FlushFileBuffers(mapping_handle);
                FlushFileBuffers(file_handle);
            }
        }
        Ok(())
    }

    pub fn async_flush(&self) -> Result<(), String> {
        // No async flushes in Windows :(
        self.sync_flush()
    }

    /// Return the number of `usize` words in the slice
    pub fn len(&self) -> usize {
        self.len
    }
}
