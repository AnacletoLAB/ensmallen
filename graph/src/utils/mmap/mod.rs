//! https://www.kernel.org/doc/Documentation/admin-guide/mm/hugetlbpage.rst
//! https://man7.org/linux/man-pages/man2/mmap.2.html
//! https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile
//! https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createfilemappinga
//! https://docs.microsoft.com/en-us/windows/win32/memory/creating-a-file-mapping-object

#[cfg(target_os = "windows")]
mod windows_mmap;
#[cfg(target_os = "windows")]
pub use windows_mmap::MemoryMappedReadOnlyFile;

#[cfg(not(target_os = "windows"))]
mod unix;
#[cfg(not(target_os = "windows"))]
pub use unix::MemoryMappedReadOnlyFile;

unsafe impl Sync for MemoryMappedReadOnlyFile {}
unsafe impl Send for MemoryMappedReadOnlyFile {}
