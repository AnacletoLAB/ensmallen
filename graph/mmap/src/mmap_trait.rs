use core::fmt::Debug;

pub trait MemoryMapCore: Sized {
    fn new_mut<S: AsRef<str> + Debug>(path: Option<S>, len: Option<usize>) 
        -> Result<Self, String>;

    fn sync_flush(&self) -> Result<(), String>;
    fn async_flush(&self) -> Result<(), String>;
}

pub trait MemoryMapReadOnlyCore: Sized {
    fn new<S: AsRef<str> + Debug>(path: S) -> Result<Self, String>;

    fn get_addr(&self) -> *mut u8;
    fn len(&self) -> usize;
    fn get_path(&self) -> Option<String>;
}

pub trait MemoryMappedReadOnlyImpl: MemoryMapReadOnlyCore {
    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    fn get<T>(&self, offset: usize) -> Result<&T, String>{
        if offset + std::mem::size_of::<T>() > self.len() {
            return Err(format!(
                concat!(
                    "Could not create a slice on the MMapped memory because the ",
                    "offset of `{}` bytes is bigger than the len of the mmap `{}`."
                ),
                offset, self.len(),
            ));
        }

        Ok(unsafe { self.get_unchecked(offset) })
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    unsafe fn get_unchecked<T>(&self, offset: usize) -> &T{
        // get a ptr to the start of the requested slice taking in
        // consideration offset
        let ptr = (self.get_addr() as *const u8).add(offset);

        // Create the actual slice
        &*(ptr as *const T)
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    fn get_slice<T>(&self, offset: usize, elements_len: Option<usize>) 
        -> Result<&[T], String>{
        let elements_len =
            elements_len.unwrap_or(self.len().saturating_sub(offset) / std::mem::size_of::<T>());
        // Convert from number of elements to number of bytes
        let bytes_len = elements_len * std::mem::size_of::<T>();

        if offset > self.len() {
            return Err(format!(
                concat!(
                    "Could not create a slice on the MMapped memory because the ",
                    "offset of `{}` bytes is bigger than the len of the mmap `{}`."
                ),
                offset, self.len(),
            ));
        }

        if bytes_len > self.len() - offset {
            return Err(format!(
                concat!(
                    "The current MMap has size of {} bytes, you are asking to ",
                    "skip {} bytes leaving {} bytes available. You asked for `{}` ",
                    "elements of `{}` bytes, for a total of `{}` bytes. ",
                    "Therefore, you asked `{}` too many bytes."
                ),
                self.len(),
                offset,
                self.len() - offset,
                elements_len,
                std::mem::size_of::<T>(),
                bytes_len,
                (self.len() - offset) - bytes_len,
            ));
        }

        Ok(unsafe { self.get_slice_unchecked(offset, Some(elements_len)) })
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    unsafe fn get_slice_unchecked<T>(
        &self,
        offset: usize,
        elements_len: Option<usize>,
    ) -> &[T]{
        let elements_len =
            elements_len.unwrap_or(self.len().saturating_sub(offset) / std::mem::size_of::<T>());
        // get a ptr to the start of the requested slice taking in
        // consideration offset
        let ptr = (self.get_addr() as *const u8).add(offset);

        // Create the actual slice
        std::slice::from_raw_parts(ptr as *const T, elements_len)
    }

    /// Returns a new str of `len` bytes starting from `offset`
    /// bytes from the start of the memory.
    ///
    /// # Safety
    /// This assumes that the data is valid utf8 chars.
    fn as_str(&self, offset: usize, len: Option<usize>) -> Result<&str, String> {
        unsafe {
            Ok(std::str::from_utf8_unchecked(
                self.get_slice::<u8>(offset, len)?,
            ))
        }
    }
}

pub trait MemoryMappedImpl: MemoryMappedReadOnlyImpl + MemoryMapCore {
    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    fn get_mut<T>(&mut self, offset: usize) -> Result<&mut T, String> {
        if offset + std::mem::size_of::<T>() > self.len() {
            return Err(format!(
                concat!(
                    "Could not create a slice on the MMapped memory because the ",
                    "offset of `{}` bytes is bigger than the len of the mmap `{}`."
                ),
                offset, self.len(),
            ));
        }

        Ok(unsafe { self.get_mut_unchecked(offset) })
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    unsafe fn get_mut_unchecked<T>(&mut self, offset: usize) -> &mut T {
        // get a ptr to the start of the requested slice taking in
        // consideration offset
        let ptr = (self.get_addr() as *mut u8).add(offset);

        // Create the actual slice
        &mut *(ptr as *mut T)
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    fn get_slice_mut<T>(
        &mut self,
        offset: usize,
        elements_len: Option<usize>,
    ) -> Result<&mut [T], String> {
        let elements_len =
            elements_len.unwrap_or(self.len().saturating_sub(offset) / std::mem::size_of::<T>());
        // Convert from number of elements to number of bytes
        let bytes_len = elements_len * std::mem::size_of::<T>();

        if offset >= self.len() {
            return Err(format!(
                concat!(
                    "Could not create a slice on the MMapped memory because the ",
                    "offset of `{}` bytes is bigger than the len of the mmap `{}`."
                ),
                offset, self.len(),
            ));
        }

        if bytes_len > self.len() - offset {
            return Err(format!(
                concat!(
                    "The current MMap has size of {} bytes, you are asking to ",
                    "skip {} bytes leaving {} bytes available. You asked for `{}` ",
                    "elements of `{}` bytes, for a total of `{}` bytes. ",
                    "Therefore, you asked `{}` too many bytes."
                ),
                self.len(),
                offset,
                self.len() - offset,
                elements_len,
                std::mem::size_of::<T>(),
                bytes_len,
                (self.len() - offset) - bytes_len,
            ));
        }

        Ok(unsafe { self.get_slice_mut_unchecked::<T>(offset, Some(elements_len)) })
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    unsafe fn get_slice_mut_unchecked<T>(
        &mut self,
        offset: usize,
        elements_len: Option<usize>,
    ) -> &mut [T] {
        let elements_len =
            elements_len.unwrap_or(self.len().saturating_sub(offset) / std::mem::size_of::<T>());

        // get a ptr to the start of the requested slice taking in
        // consideration offset
        let ptr = (self.get_addr() as *mut u8).add(offset);

        // Create the actual slice
        std::slice::from_raw_parts_mut(ptr as *mut T, elements_len)
    }

    /// Returns a new str of `len` bytes starting from `offset`
    /// bytes from the start of the memory.
    ///
    /// # Safety
    /// This assumes that the data is valid utf8 chars.
    fn as_str_mut(&mut self, offset: usize, len: Option<usize>) -> Result<&mut str, String> {
        unsafe {
            Ok(std::str::from_utf8_unchecked_mut(
                self.get_slice_mut::<u8>(offset, len)?,
            ))
        }
    }
}