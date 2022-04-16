use libc::*;

/// A read-only memory mapped file,
/// this should be equivalent to read-only slice that
/// automatically handle the freeing.
#[derive(Debug)]
pub struct MemoryMappedReadOnlyFile {
    fd: i32,
    addr: *mut c_void,
    len: usize,
}

impl std::ops::Drop for MemoryMappedReadOnlyFile {
    fn drop(&mut self) {
        unsafe {
            // unmap the memory
            munmap(self.addr, self.len);
            // close the file descriptor
            close(self.fd);
        }
    }
}

impl MemoryMappedReadOnlyFile {
    pub fn new(path: &str) -> Result<Self, String> {
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

        #[cfg(not(target_os = "windows"))]
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
                libc::MAP_PRIVATE,
                // the file descriptor of the file to mmap
                fd,
                // the offset in bytes from the start of the file, we want to mmap
                // the whole file
                0,
            )
        };

        if addr == usize::MAX as *mut c_void {
            return Err(format!(
                concat!(
                    "Cannot mmap the file '{}' with file descriptor '{}'. ",
                    "https://man7.org/linux/man-pages/man2/mmap.2.html",
                    " or the equivalent manual for your POSIX OS.",
                ),
                path, fd
            ));
        }

        Ok(MemoryMappedReadOnlyFile { fd, addr, len })
    }

    /// Return the number of `usize` words in the slice
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_str(&self) -> &'static str {
        unsafe {
            let slice = std::slice::from_raw_parts(self.addr as *const u8, self.len);
            std::str::from_utf8_unchecked(slice)
        }
    }
}
