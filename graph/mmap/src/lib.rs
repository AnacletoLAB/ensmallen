//! https://www.kernel.org/doc/Documentation/admin-guide/mm/hugetlbpage.rst
//! https://man7.org/linux/man-pages/man2/mmap.2.html
//! https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile
//! https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createfilemappinga
//! https://docs.microsoft.com/en-us/windows/win32/memory/creating-a-file-mapping-object

pub(crate) mod mmap_trait;
pub use mmap_trait::*;

#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
pub use win::MemoryMapped;

#[cfg(target_os = "windows")]
mod win_ro;
#[cfg(target_os = "windows")]
pub use win_ro::MemoryMappedReadOnly;

#[cfg(not(target_os = "windows"))]
mod unix;
#[cfg(not(target_os = "windows"))]
pub use unix::MemoryMapped;

#[cfg(not(target_os = "windows"))]
mod unix_ro;
#[cfg(not(target_os = "windows"))]
pub use unix_ro::MemoryMappedReadOnly;

unsafe impl Sync for MemoryMappedReadOnly {}
unsafe impl Send for MemoryMappedReadOnly {}
impl MemoryMappedReadOnlyImpl for MemoryMappedReadOnly {}

impl MemoryMappedImpl for MemoryMapped {}
impl MemoryMappedReadOnlyImpl for MemoryMapped {}
