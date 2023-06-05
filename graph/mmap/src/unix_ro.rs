use super::{errno, MemoryMapReadOnlyCore};
use core::fmt::Debug;
use libc::*;

//const MAP_HUGE_2MB: i32 = 1_409_286_144i32;

/// A read-only memory mapped file,
/// this should be equivalent to read-only slice that
/// automatically handle the freeing.
#[derive(Debug)]
pub struct MemoryMappedReadOnly {
    pub(crate) fd: i32,
    pub(crate) addr: *mut c_void,
    pub(crate) len: usize,
    pub(crate) path: Option<String>,
}

impl std::ops::Drop for MemoryMappedReadOnly {
    fn drop(&mut self) {
        unsafe {
            // unmap the memory
            munmap(self.addr, self.len);
            // close the file descriptor
            close(self.fd);
        }
    }
}

impl MemoryMapReadOnlyCore for MemoryMappedReadOnly {
    fn new<S: AsRef<str> + Debug>(path: S, offset: Option<usize>) -> Result<Self, String> {
        let path = path.as_ref();
        // here we add a + 8 to map in an extra zero-filled word so that we can
        // do unaligned reads for bits
        let len = std::fs::metadata(path).map_err(|e| e.to_string())?.len() as usize;

        let mut c_string = path.to_string();
        c_string.push('\0');
        // Get a file descriptor to the file
        let fd = unsafe { open(c_string.as_ptr() as *const i8, O_RDONLY) };

        // check that it was successful
        if fd == -1 {
            return Err(format!("Cannot open the file '{}' to mmap it.", path));
        }
        // Try to mmap the file into memory

        let flags = libc::MAP_PRIVATE;

        //if cfg!(target_os = "linux") {
        //    flags |= MAP_HUGE_2MB;
        //}

        let addr = unsafe {
            mmap(
                // we don't want a specific address
                core::ptr::null_mut(),
                // the len of the file in bytes
                len,
                // Read only
                PROT_READ,
                // We don't want the eventual modifications to get propagated
                // to the underlying file
                flags,
                // the file descriptor of the file to mmap
                fd,
                // the offset in bytes from the start of the file, we want to mmap
                // the whole file
                offset.unwrap_or(0) as i64,
            )
        };

        if addr == usize::MAX as *mut c_void {
            return Err(format!(
                concat!(
                    "Cannot mmap the file '{}' with file descriptor '{}'. ",
                    "The mmap was called with len '{}' and offset: '{}'",
                    "https://man7.org/linux/man-pages/man2/mmap.2.html",
                    " or the equivalent manual for your POSIX OS. ERRNO: {}",
                ),
                path,
                fd,
                len,
                offset.unwrap_or(0),
                errno(),
            ));
        }

        Ok(MemoryMappedReadOnly {
            fd,
            addr,
            len,
            path: Some(path.to_string()),
        })
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
