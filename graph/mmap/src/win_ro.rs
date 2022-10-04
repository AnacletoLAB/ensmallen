use crate::MemoryMapReadOnlyCore;
use core::ffi::c_void;
use core::fmt::Debug;
use windows::Win32::Foundation::*;
use windows::Win32::Storage::FileSystem::*;
use windows::Win32::System::Memory::*;

#[derive(Debug)]
pub struct MemoryMappedReadOnly {
    pub(crate) file_handle: HANDLE,
    pub(crate) mapping_handle: HANDLE,
    pub(crate) addr: *mut c_void,
    pub(crate) len: usize,
    pub(crate) path: Option<String>,
}

impl Drop for MemoryMappedReadOnly {
    fn drop(&mut self) {
        unsafe {
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

impl MemoryMapReadOnlyCore for MemoryMappedReadOnly {
    fn new<S: AsRef<str> + Debug>(path: S, offset: Option<usize>) -> Result<Self, String> {
        let path = path.as_ref();
        assert!(offset == None, "MMAP offsetting is TODO on windows");
        unsafe {
            let file_handle = CreateFileW(
                path,
                FILE_GENERIC_READ,
                FILE_SHARE_NONE, // prevent other processes to modify the file while we are reading it
                std::ptr::null() as _,
                OPEN_EXISTING,
                FILE_FLAG_SEQUENTIAL_SCAN,
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
                PAGE_READONLY, // | SEC_LARGE_PAGES,
                0,             // max size
                0,             // max size
                PWSTR(std::ptr::null_mut()),
            );

            if mapping_handle == INVALID_HANDLE_VALUE {
                return Err("Error opening file CreateFileMappingW".into());
            }

            let addr = MapViewOfFile(
                mapping_handle,
                FILE_MAP_READ, // | FILE_MAP_LARGE_PAGES
                0,             // offset high
                0,             // offset low
                len,
            );

            if addr == std::ptr::null_mut() as _ {
                return Err("Error opening file MapViewOfFile".into());
            }

            Ok(MemoryMappedReadOnly {
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
