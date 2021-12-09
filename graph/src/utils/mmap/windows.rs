
use core::ffi::c_void;
use windows::Win32::Foundation::*;
use windows::Win32::System::Memory::*;
use windows::Win32::Storage::FileSystem::*;

struct MemoryMappedReadOnlyFile {
    file_handle: HANDLE,
    mapping_handle: HANDLE,
    addr: *mut c_void,
    len: usize,
}

impl Drop for MemoryMappedReadOnlyFile {
    fn drop(&mut self) {
        unsafe {
            let res = UnmapViewOfFile(self.addr);
            if res == BOOL(0) {
                panic!(
                    "Cannot unmap view of file.",
                );
            }

            let res = CloseHandle(self.mapping_handle);
            if res == BOOL(0) {
                panic!(
                    "Cannot Close the mapping handle."
                );
            }

            let res = CloseHandle(self.file_handle);
            if res == BOOL(0) {
                panic!(
                    "Cannot Close the mapping handle."
                );
             }
        }
    }
}

impl MemoryMappedReadOnlyFile {
    pub fn new(path: &str) -> Result<MemoryMappedReadOnlyFile, String> {
        unsafe {
            let file_handle = CreateFileW(
                path,
                FILE_GENERIC_READ,
                FILE_SHARE_NONE,  // prevent other processes to modify the file while we are reading it
                std::ptr::null() as _,
                OPEN_EXISTING,
                FILE_FLAG_SEQUENTIAL_SCAN,
                HANDLE(0),
            );
            
            if file_handle == INVALID_HANDLE_VALUE {
                return Err(
                    "Error opening file CreateFileW".into()
                );
            }

            let mut len_higher: u32 = 0;
            let len_lower = GetFileSize(
                file_handle, 
                (&mut len_higher) as *mut u32
            );
            let len = ((len_lower as u64) | (len_higher as u64) << 32) as usize;

            let mapping_handle = CreateFileMappingW(
                file_handle,
                std::ptr::null_mut(),
                PAGE_READONLY, // | SEC_LARGE_PAGES, 
                0, 
                0, 
                PWSTR(std::ptr::null_mut()),
            );
            
            if mapping_handle == HANDLE(0) {
                return Err(
                    "Error opening file CreateFileMappingW".into()
                );
            }


            let addr = MapViewOfFile(
                mapping_handle,
                FILE_MAP_READ, // | FILE_MAP_LARGE_PAGES
                0,
                0,
                len,
            );
            
            if addr == std::ptr::null_mut() as _ {
                return Err(
                    "Error opening file MapViewOfFile".into()
                );
            }

            Ok(MMap{
                file_handle,
                mapping_handle,
                addr,
                len,
            })
        }
    }

    pub fn as_str(&self) -> &'static str {
        unsafe {
            let slice = std::slice::from_raw_parts(self.addr as *const u8, self.len);
            std::str::from_utf8_unchecked(slice)
        }
    }

    /// Return the number of `usize` words in the slice
    pub fn len(&self) -> usize {
        self.len
    }
}