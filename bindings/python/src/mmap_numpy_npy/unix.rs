use libc::*;

/// A read-only memory mapped file,
/// this should be equivalent to read-only slice that
/// automatically handle the freeing.
#[derive(Debug)]
pub struct MemoryMapped {
    fd: Option<i32>,
    addr: *mut c_void,
    len: usize,
    path: Option<String>,
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

impl MemoryMapped {
    /// Memory map the file with mutability permissions
    pub fn new(path: Option<&str>, len: Option<usize>) -> Result<Self, String> {
        let (addr, fd, len) = match (path, len) {
            // New file / expand file
            (Some(path), maybe_len) => {
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
                        libc::MAP_SHARED
                            | libc::MAP_POPULATE
                            | libc::MAP_HUGETLB
                            | libc::MAP_HUGE_1GB,
                        // the file descriptor of the file to mmap
                        fd,
                        // the offset in bytes from the start of the file, we want to mmap
                        // the whole file
                        0,
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
                        libc::MAP_SHARED | libc::MAP_ANONYMOUS,
                        // the file descriptor of the file to mmap
                        0,
                        // the offset in bytes from the start of the file, we want to mmap
                        // the whole file
                        0,
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
                    "Cannot mmap the file '{:?}' with file descriptor '{:?}'. ",
                    "https://man7.org/linux/man-pages/man2/mmap.2.html",
                    " or the equivalent manual for your POSIX OS.",
                ),
                path, fd
            ));
        }

        Ok(MemoryMapped {
            fd,
            addr,
            len,
            path: path.map(|x| x.into()),
        })
    }

    pub fn sync_flush(&self) -> Result<(), String> {
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

    pub fn async_flush(&self) -> Result<(), String> {
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

    /// Return the number of `usize` words in the slice
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    pub fn get<T>(&self, offset: usize) -> Result<&T, String> {
        if offset + std::mem::size_of::<T>() > self.len {
            return Err(format!(
                concat!(
                    "Could not create a slice on the MMapped memory because the ",
                    "offset of `{}` bytes is bigger than the len of the mmap `{}`."
                ),
                offset, self.len,
            ));
        }

        Ok(unsafe { self.get_unchecked(offset) })
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    pub unsafe fn get_unchecked<T>(&self, offset: usize) -> &T {
        // get a ptr to the start of the requested slice taking in
        // consideration offset
        let ptr = (self.addr as *const u8).add(offset);

        // Create the actual slice
        &*(ptr as *const T)
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    pub fn get_mut<T>(&mut self, offset: usize) -> Result<&mut T, String> {
        if offset + std::mem::size_of::<T>() > self.len {
            return Err(format!(
                concat!(
                    "Could not create a slice on the MMapped memory because the ",
                    "offset of `{}` bytes is bigger than the len of the mmap `{}`."
                ),
                offset, self.len,
            ));
        }

        Ok(unsafe { self.get_mut_unchecked(offset) })
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    pub unsafe fn get_mut_unchecked<T>(&mut self, offset: usize) -> &mut T {
        // get a ptr to the start of the requested slice taking in
        // consideration offset
        let ptr = (self.addr as *mut u8).add(offset);

        // Create the actual slice
        &mut *(ptr as *mut T)
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    pub fn get_slice<T>(&self, offset: usize, elements_len: Option<usize>) -> Result<&[T], String> {
        let elements_len =
            elements_len.unwrap_or(self.len().saturating_sub(offset) / std::mem::size_of::<T>());
        // Convert from number of elements to number of bytes
        let bytes_len = elements_len * std::mem::size_of::<T>();

        if offset > self.len {
            return Err(format!(
                concat!(
                    "Could not create a slice on the MMapped memory because the ",
                    "offset of `{}` bytes is bigger than the len of the mmap `{}`."
                ),
                offset, self.len,
            ));
        }

        if bytes_len > self.len - offset {
            return Err(format!(
                concat!(
                    "The current MMap has size of {} bytes, you are asking to ",
                    "skip {} bytes leaving {} bytes available. You asked for `{}` ",
                    "elements of `{}` bytes, for a total of `{}` bytes. ",
                    "Therefore, you asked `{}` too many bytes."
                ),
                self.len,
                offset,
                self.len - offset,
                elements_len,
                std::mem::size_of::<T>(),
                bytes_len,
                (self.len - offset) - bytes_len,
            ));
        }

        Ok(unsafe { self.get_slice_unchecked(offset, Some(elements_len)) })
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    pub unsafe fn get_slice_unchecked<T>(
        &self,
        offset: usize,
        elements_len: Option<usize>,
    ) -> &[T] {
        let elements_len =
            elements_len.unwrap_or(self.len().saturating_sub(offset) / std::mem::size_of::<T>());
        // get a ptr to the start of the requested slice taking in
        // consideration offset
        let ptr = (self.addr as *const u8).add(offset);

        // Create the actual slice
        std::slice::from_raw_parts(ptr as *const T, elements_len)
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    pub fn get_slice_mut<T>(
        &mut self,
        offset: usize,
        elements_len: Option<usize>,
    ) -> Result<&mut [T], String> {
        let elements_len =
            elements_len.unwrap_or(self.len().saturating_sub(offset) / std::mem::size_of::<T>());
        // Convert from number of elements to number of bytes
        let bytes_len = elements_len * std::mem::size_of::<T>();

        if offset >= self.len {
            return Err(format!(
                concat!(
                    "Could not create a slice on the MMapped memory because the ",
                    "offset of `{}` bytes is bigger than the len of the mmap `{}`."
                ),
                offset, self.len,
            ));
        }

        if bytes_len > self.len - offset {
            return Err(format!(
                concat!(
                    "The current MMap has size of {} bytes, you are asking to ",
                    "skip {} bytes leaving {} bytes available. You asked for `{}` ",
                    "elements of `{}` bytes, for a total of `{}` bytes. ",
                    "Therefore, you asked `{}` too many bytes."
                ),
                self.len,
                offset,
                self.len - offset,
                elements_len,
                std::mem::size_of::<T>(),
                bytes_len,
                (self.len - offset) - bytes_len,
            ));
        }

        Ok(unsafe { self.get_slice_mut_unchecked::<T>(offset, Some(elements_len)) })
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    pub unsafe fn get_slice_mut_unchecked<T>(
        &mut self,
        offset: usize,
        elements_len: Option<usize>,
    ) -> &mut [T] {
        let elements_len =
            elements_len.unwrap_or(self.len().saturating_sub(offset) / std::mem::size_of::<T>());

        // get a ptr to the start of the requested slice taking in
        // consideration offset
        let ptr = (self.addr as *mut u8).add(offset);

        // Create the actual slice
        std::slice::from_raw_parts_mut(ptr as *mut T, elements_len)
    }

    /// Returns a new str of `len` bytes starting from `offset`
    /// bytes from the start of the memory.
    ///
    /// # Safety
    /// This assumes that the data is valid utf8 chars.
    pub fn as_str(&self, offset: usize, len: Option<usize>) -> Result<&str, String> {
        unsafe {
            Ok(std::str::from_utf8_unchecked(
                self.get_slice::<u8>(offset, len)?,
            ))
        }
    }

    /// Returns a new str of `len` bytes starting from `offset`
    /// bytes from the start of the memory.
    ///
    /// # Safety
    /// This assumes that the data is valid utf8 chars.
    pub fn as_str_mut(&mut self, offset: usize, len: Option<usize>) -> Result<&mut str, String> {
        unsafe {
            Ok(std::str::from_utf8_unchecked_mut(
                self.get_slice_mut::<u8>(offset, len)?,
            ))
        }
    }
}
