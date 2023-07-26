use super::*;
use core::fmt::Debug;
use libc::*;

//const MAP_HUGE_2MB: i32 = 1_409_286_144i32;

/// A read-only memory mapped file,
/// this should be equivalent to read-only slice that
/// automatically handle the freeing.
#[derive(Debug)]
pub struct MemoryMapped {
    pub(crate) fd: Option<i32>,
    pub(crate) addr: *mut c_void,
    pub(crate) len: usize,
    pub(crate) path: Option<String>,
}

impl std::ops::Drop for MemoryMapped {
    fn drop(&mut self) {
        unsafe {
            // if we have modified a memory mapped file, we run a sync before
            // closing
            if self.fd.is_some() {
                self.sync_flush().unwrap();
            }

            // unmap the memory
            munmap(self.addr, self.len);

            if let Some(fd) = self.fd {
                // close the file descriptor
                close(fd);
            }
        }
    }
}

impl MemoryMapReadOnlyCore for MemoryMapped {
    fn new<S: AsRef<str> + Debug>(path: S, offset: Option<usize>) -> Result<Self, String> {
        let path = path.as_ref();
        // here we add a + 8 to map in an extra zero-filled word so that we can
        // do unaligned reads for bits
        let len = std::fs::metadata(path).map_err(|e| e.to_string())?.len() as usize;

        let mut c_string = path.to_string();
        c_string.push('\0');
        // Get a file descriptor to the file
        let fd = unsafe { open(c_string.as_ptr() as *const _, O_RDONLY) };

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

        Ok(MemoryMapped {
            fd: Some(fd),
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

impl MemoryMapCore for MemoryMapped {
    /// Memory map the file with mutability permissions
    fn new_mut<S: AsRef<str> + Debug>(
        path: Option<S>,
        len: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Self, String> {
        let flags = libc::MAP_SHARED;

        //if cfg!(target_os = "linux") {
        //    flags |= MAP_HUGE_2MB;
        //}

        let (addr, fd, len) = match (path.as_ref(), len) {
            // New file / expand file
            (Some(path), maybe_len) => {
                let path = path.as_ref();
                let (fd, len) = if let Some(len) = maybe_len {
                    // we have a len, so we can create the file if not present

                    // Get a file descriptor to the file
                    let fd = unsafe {
                        open(
                            std::ffi::CString::new(path.as_bytes())
                                .map_err(|e| e.to_string())?
                                .as_ptr(),
                            O_RDWR | O_CREAT,
                            (S_IRUSR | S_IWUSR) as libc::c_int,
                        )
                    };
                    // check that it was successful
                    if fd == -1 {
                        return Err(format!("Cannot open the file '{}' to mmap it.", path));
                    }
                    // allocate the memory in the file (if needed)
                    let res = unsafe { ftruncate(fd, len as i64) };
                    if res == -1 {
                        unsafe { close(fd) };
                        return Err(format!("Cannot ftruncate the file '{}'", path));
                    }
                    (fd, len)
                } else {
                    // no len, so we can just load a present file
                    let len = std::fs::metadata(path).map_err(|e| e.to_string())?.len() as usize;

                    // Get a file descriptor to the file
                    let fd = unsafe {
                        open(
                            std::ffi::CString::new(path.as_bytes())
                                .map_err(|e| e.to_string())?
                                .as_ptr(),
                            O_RDWR,
                        )
                    };
                    // check that it was successful
                    if fd == -1 {
                        return Err(format!("Cannot open the file '{}' to mmap it.", path));
                    }
                    (fd, len)
                };

                let addr = unsafe {
                    mmap(
                        // we don't want a specific address
                        core::ptr::null_mut(),
                        // the len of the file in bytes
                        len,
                        // Read only
                        PROT_READ | PROT_WRITE,
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
                (addr, Some(fd), len)
            }
            // anonymous
            (None, Some(len)) => {
                let addr = unsafe {
                    mmap(
                        // we don't want a specific address
                        core::ptr::null_mut(),
                        // the len of the file in bytes
                        len,
                        // Read only
                        PROT_READ | PROT_WRITE,
                        // We don't want the eventual modifications to get propagated
                        // to the underlying file
                        flags | libc::MAP_ANONYMOUS,
                        // the file descriptor of the file to mmap
                        0,
                        // the offset in bytes from the start of the file, we want to mmap
                        // the whole file
                        offset.unwrap_or(0) as i64,
                    )
                };
                (addr, None, len)
            }
            (None, None) => {
                return Err("Cannot create an mmap without both a path and a len".to_string());
            }
        };

        if addr == usize::MAX as *mut c_void {
            return Err(format!(
                concat!(
                    "Cannot mmap the file '{:?}' with file descriptor '{:?}' .",
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

        Ok(MemoryMapped {
            fd,
            addr,
            len,
            path: path.map(|x| x.as_ref().into()),
        })
    }

    fn sync_flush(&self) -> Result<(), String> {
        if self.fd.is_some() {
            unsafe {
                let res = msync(self.addr as _, self.len, MS_SYNC);
                if res == -1 {
                    return Err("Error syncronously syncing the mmap for spine ".into());
                }
            }
        }
        Ok(())
    }

    fn async_flush(&self) -> Result<(), String> {
        if self.fd.is_some() {
            unsafe {
                let res = msync(self.addr as _, self.len, MS_ASYNC);
                if res == -1 {
                    return Err("Error asyncronously syncing the mmap for spine ".into());
                }
            }
        }
        Ok(())
    }
}
