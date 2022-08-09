use cuda_driver_sys::*;
use std::ffi::{c_void, CString};

use std::sync::atomic::{AtomicBool, Ordering};
static CUDA_DRIVERS_HAVE_BEEN_INITIALIZED: AtomicBool = <AtomicBool>::new(false);

/// Create arguments for a kernel
#[macro_export]
macro_rules! args {
    [$($value:expr,)*] => {
        &mut vec![
            $(
                & $value as *const _ as *mut core::ffi::c_void,
            )*
        ]
    };
}

macro_rules! impl_gpu_error {
    ($(
        $field:ident => $value:literal => $doc:literal,
    )*) => {

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum GPUError {
    $(
        #[doc=$doc]
        $field,
    )*
    /// This means that the given error code from is not a CUDA standard error.
    Invalid(usize),
}

impl ToString for GPUError {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl From<GPUError> for String {
    fn from(other: GPUError) -> String {
        other.to_string()
    }
}

impl std::fmt::Debug for GPUError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use GPUError::*;
        match self {
            $($field => {
                f.write_str(stringify!($field))?;
                f.write_str(" : ")?;
                f.write_str($doc)
            },)*
            Invalid(val) => f.write_str(format!("Invalid CUDA ERROR with code {}", val).as_str()),
        }
    }
}

impl From<CUresult> for GPUError {
    fn from(value: CUresult) -> Self {
        use GPUError::*;
        match value as usize {
            $(
                $value => $field,
            )*
            x @ _ => Invalid(x as _),
        }
    }
}

impl From<usize> for GPUError {
    fn from(value: usize) -> Self {
        use GPUError::*;
        match value {
            $(
                $value => $field,
            )*
            x @ _ => Invalid(x as _),
        }
    }
}

impl From<GPUError> for usize  {
    fn from(value: GPUError) -> usize {
        use GPUError::*;
        match value {
            $(
                $field => $value,
            )*
            Invalid(i) => i as _,
        }
    }
}

impl GPUError {
    pub fn into_result<T>(self, ok_val: T) -> Result<T, Self> {
        if self == GPUError::Success {
            return Ok(ok_val);
        }
        Err(self)
    }
}

    };
}

impl_gpu_error! {
    Success => 0 => "The API call returned with no errors. In the case of query calls, this also means that the operation being queried is complete (see cuEventQuery() and cuStreamQuery()).",
    InvalidValue => 1 => "This indicates that one or more of the parameters passed to the API call is not within an acceptable range of values.",
    OutOfMemory => 2 => "The API call failed because it was unable to allocate enough memory to perform the requested operation.",
    NotInitialized => 3 => "This indicates that the CUDA driver has not been initialized with cuInit() or that initialization has failed.",
    Deinitialized => 4 => "This indicates that the CUDA driver is in the process of shutting down.",
    ProfilerDisabled => 5 => "This indicates profiler is not initialized for this run. This can happen when the application is running with external profiling tools like visual profiler.",
    ProfilerNotInitialized => 6 => "Deprecated This error return is deprecated as of CUDA 5.0. It is no longer an error to attempt to enable/disable the profiling via cuProfilerStart or cuProfilerStop without initialization.",
    ProfilerAlreadyStarted => 7 => "Deprecated This error return is deprecated as of CUDA 5.0. It is no longer an error to call cuProfilerStart() when profiling is already enabled.",
    ProfilerAlreadyStopped => 8 => "Deprecated This error return is deprecated as of CUDA 5.0. It is no longer an error to call cuProfilerStop() when profiling is already disabled.",
    StubLibrary => 34 => "This indicates that the CUDA driver that the application has loaded is a stub library. Applications that run with the stub rather than a real driver loaded will result in CUDA API returning this error.",
    NoDevice => 100 => "This indicates that no CUDA-capable devices were detected by the installed CUDA driver.",
    InvalidDevice => 101 => "This indicates that the device ordinal supplied by the user does not correspond to a valid CUDA device or that the action requested is invalid for the specified device.",
    DeviceNotLicensed => 102 => "This error indicates that the Grid license is not applied.",
    InvalidImage => 200 => "This indicates that the device kernel image is invalid. This can also indicate an invalid CUDA module.",
    InvalidContext => 201 => "This most frequently indicates that there is no context bound to the current thread. This can also be returned if the context passed to an API call is not a valid handle (such as a context that has had cuCtxDestroy() invoked on it). This can also be returned if a user mixes different API versions (i.e. 3010 context with 3020 API calls). See cuCtxGetApiVersion() for more details.",
    ContextAlreadyCurrent => 202 => "Deprecated This error return is deprecated as of CUDA 3.2. It is no longer an error to attempt to push the active context via cuCtxPushCurrent(). This indicated that the context being supplied as a parameter to the API call was already the active context.",
    MapFailed => 205 => "This indicates that a map or register operation has failed.",
    UnmapFailed => 206 => "This indicates that an unmap or unregister operation has failed.",
    ArrayIsMapped => 207 => "This indicates that the specified array is currently mapped and thus cannot be destroyed.",
    AlreadyMapped => 208 => "This indicates that the resource is already mapped.",
    NoBinaryForGpu => 209 => "This indicates that there is no kernel image available that is suitable for the device. This can occur when a user specifies code generation options for a particular CUDA source file that do not include the corresponding device configuration.",
    AlreadyAcquired => 210 => "This indicates that a resource has already been acquired.",
    NotMapped => 211 => "This indicates that a resource is not mapped.",
    NotMappedAsArray => 212 => "This indicates that a mapped resource is not available for access as an array.",
    NotMappedAsPointer => 213 => "This indicates that a mapped resource is not available for access as a pointer.",
    EccUncorrectable => 214 => "This indicates that an uncorrectable ECC error was detected during execution.",
    UnsupportedLimit => 215 => "This indicates that the CUlimit passed to the API call is not supported by the active device.",
    ContextAlreadyInUse => 216 => "This indicates that the CUcontext passed to the API call can only be bound to a single CPU thread at a time but is already bound to a CPU thread.",
    PeerAccessUnsupported => 217 => "This indicates that peer access is not supported across the given devices.",
    InvalidPtx => 218 => "This indicates that a PTX JIT compilation failed.",
    InvalidGraphicsContext => 219 => "This indicates an error with OpenGL or DirectX context.",
    NvlinkUncorrectable => 220 => "This indicates that an uncorrectable NVLink error was detected during the execution.",
    JitCompilerNotFound => 221 => "This indicates that the PTX JIT compiler library was not found.",
    UnsupportedPtxVersion => 222 => "This indicates that the provided PTX was compiled with an unsupported toolchain.",
    JitCompilationDisabled => 223 => "This indicates that the PTX JIT compilation was disabled.",
    UnsupportedExecAffinity => 224 => "This indicates that the CUexecAffinityType passed to the API call is not supported by the active device.",
    InvalidSource => 300 => "This indicates that the device kernel source is invalid. This includes compilation/linker errors encountered in device code or user error.",
    FileNotFound => 301 => "This indicates that the file specified was not found.",
    SharedObjectSymbolNotFound => 302 => "This indicates that a link to a shared object failed to resolve.",
    SharedObjectInitFailed => 303 => "This indicates that initialization of a shared object failed.",
    OperatingSystem => 304 => "This indicates that an OS call failed.",
    InvalidHandle => 400 => "This indicates that a resource handle passed to the API call was not valid. Resource handles are opaque types like CUstream and CUevent.",
    IllegalState => 401 => "This indicates that a resource required by the API call is not in a valid state to perform the requested operation.",
    NotFound => 500 => "This indicates that a named symbol was not found. Examples of symbols are global/constant variable names, driver function names, texture names, and surface names.",
    NotReady => 600 => "This indicates that asynchronous operations issued previously have not completed yet. This result is not actually an error, but must be indicated differently than CUDA_SUCCESS (which indicates completion). Calls that may return this value include cuEventQuery() and cuStreamQuery().",
    IllegalAddress => 700 => "While executing a kernel, the device encountered a load or store instruction on an invalid memory address. This leaves the process in an inconsistent state and any further CUDA work will return the same error. To continue using CUDA, the process must be terminated and relaunched.",
    LaunchOutOfResources => 701 => "This indicates that a launch did not occur because it did not have appropriate resources. This error usually indicates that the user has attempted to pass too many arguments to the device kernel, or the kernel launch specifies too many threads for the kernel's register count. Passing arguments of the wrong size (i.e. a 64-bit pointer when a 32-bit int is expected) is equivalent to passing too many arguments and can also result in this error.",
    LaunchTimeout => 702 => "This indicates that the device kernel took too long to execute. This can only occur if timeouts are enabled - see the device attribute CU_DEVICE_ATTRIBUTE_KERNEL_EXEC_TIMEOUT for more information. This leaves the process in an inconsistent state and any further CUDA work will return the same error. To continue using CUDA, the process must be terminated and relaunched.",
    LaunchIncompatibleTexturing => 703 => "This error indicates a kernel launch that uses an incompatible texturing mode.",
    PeerAccessAlreadyEnabled => 704 => "This error indicates that a call to cuCtxEnablePeerAccess() is trying to re-enable peer access to a context which has already had peer access to it enabled.",
    PeerAccessNotEnabled => 705 => "This error indicates that cuCtxDisablePeerAccess() is trying to disable peer access which has not been enabled yet via cuCtxEnablePeerAccess().",
    PrimaryContextActive => 708 => "This error indicates that the primary context for the specified device has already been initialized.",
    ContextIsDestroyed => 709 => "This error indicates that the context current to the calling thread has been destroyed using cuCtxDestroy, or is a primary context which has not yet been initialized.",
    Assert => 710 => "A device-side assert triggered during kernel execution. The context cannot be used anymore, and must be destroyed. All existing device memory allocations from this context are invalid and must be reconstructed if the program is to continue using CUDA.",
    TooManyPeers => 711 => "This error indicates that the hardware resources required to enable peer access have been exhausted for one or more of the devices passed to cuCtxEnablePeerAccess().",
    HostMemoryAlreadyRegistered => 712 => "This error indicates that the memory range passed to cuMemHostRegister() has already been registered.",
    HostMemoryNotRegistered => 713 => "This error indicates that the pointer passed to cuMemHostUnregister() does not correspond to any currently registered memory region.",
    HardwareStackError => 714 => "While executing a kernel, the device encountered a stack error. This can be due to stack corruption or exceeding the stack size limit. This leaves the process in an inconsistent state and any further CUDA work will return the same error. To continue using CUDA, the process must be terminated and relaunched.",
    IllegalInstruction => 715 => "While executing a kernel, the device encountered an illegal instruction. This leaves the process in an inconsistent state and any further CUDA work will return the same error. To continue using CUDA, the process must be terminated and relaunched.",
    MisalignedAddress => 716 => "While executing a kernel, the device encountered a load or store instruction on a memory address which is not aligned. This leaves the process in an inconsistent state and any further CUDA work will return the same error. To continue using CUDA, the process must be terminated and relaunched.",
    InvalidAddressSpace => 717 => "While executing a kernel, the device encountered an instruction which can only operate on memory locations in certain address spaces (global, shared, or local), but was supplied a memory address not belonging to an allowed address space. This leaves the process in an inconsistent state and any further CUDA work will return the same error. To continue using CUDA, the process must be terminated and relaunched.",
    InvalidPc => 718 => "While executing a kernel, the device program counter wrapped its address space. This leaves the process in an inconsistent state and any further CUDA work will return the same error. To continue using CUDA, the process must be terminated and relaunched.",
    LaunchFailed => 719 => "An exception occurred on the device while executing a kernel. Common causes include dereferencing an invalid device pointer and accessing out of bounds shared memory. Less common cases can be system specific - more information about these cases can be found in the system specific user guide. This leaves the process in an inconsistent state and any further CUDA work will return the same error. To continue using CUDA, the process must be terminated and relaunched.",
    CooperativeLaunchTooLarge => 720 => "This error indicates that the number of blocks launched per grid for a kernel that was launched via either cuLaunchCooperativeKernel or cuLaunchCooperativeKernelMultiDevice exceeds the maximum number of blocks as allowed by cuOccupancyMaxActiveBlocksPerMultiprocessor or cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags times the number of multiprocessors as specified by the device attribute CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT.",
    NotPermitted => 800 => "This error indicates that the attempted operation is not permitted.",
    NotSupported => 801 => "This error indicates that the attempted operation is not supported on the current system or device.",
    SystemNotReady => 802 => "This error indicates that the system is not yet ready to start any CUDA work. To continue using CUDA, verify the system configuration is in a valid state and all required driver daemons are actively running. More information about this error can be found in the system specific user guide.",
    SystemDriverMismatch => 803 => "This error indicates that there is a mismatch between the versions of the display driver and the CUDA driver. Refer to the compatibility documentation for supported versions.",
    CompatNotSupportedOnDevice => 804 => "This error indicates that the system was upgraded to run with forward compatibility but the visible hardware detected by CUDA does not support this configuration. Refer to the compatibility documentation for the supported hardware matrix or ensure that only supported hardware is visible during initialization via the CUDA_VISIBLE_DEVICES environment variable.",
    MpsConnectionFailed => 805 => "This error indicates that the MPS client failed to connect to the MPS control daemon or the MPS server.",
    MpsRpcFailure => 806 => "This error indicates that the remote procedural call between the MPS server and the MPS client failed.",
    MpsServerNotReady => 807 => "This error indicates that the MPS server is not ready to accept new MPS client requests. This error can be returned when the MPS server is in the process of recovering from a fatal failure.",
    MpsMaxClientsReached => 808 => "This error indicates that the hardware resources required to create MPS client have been exhausted.",
    MpsMaxConnectionsReached => 809 => "This error indicates the the hardware resources required to support device connections have been exhausted.",
    StreamCaptureUnsupported => 900 => "This error indicates that the operation is not permitted when the stream is capturing.",
    StreamCaptureInvalidated => 901 => "This error indicates that the current capture sequence on the stream has been invalidated due to a previous error.",
    StreamCaptureMerge => 902 => "This error indicates that the operation would have resulted in a merge of two independent capture sequences.",
    StreamCaptureUnmatched => 903 => "This error indicates that the capture was not initiated in this stream.",
    StreamCaptureUnjoined => 904 => "This error indicates that the capture sequence contains a fork that was not joined to the primary stream.",
    StreamCaptureIsolation => 905 => "This error indicates that a dependency would have been created which crosses the capture sequence boundary. Only implicit in-stream ordering dependencies are allowed to cross the boundary.",
    StreamCaptureImplicit => 906 => "This error indicates a disallowed implicit dependency on a current capture sequence from cudaStreamLegacy.",
    CapturedEvent => 907 => "This error indicates that the operation is not permitted on an event which was last recorded in a capturing stream.",
    StreamCaptureWrongThread => 908 => "A stream capture sequence not initiated with the CU_STREAM_CAPTURE_MODE_RELAXED argument to cuStreamBeginCapture was passed to cuStreamEndCapture in a different thread.",
    Timeout => 909 => "This error indicates that the timeout specified for the wait operation has lapsed.",
    GraphExecUpdateFailure => 910 => "This error indicates that the graph update was not performed because it included changes which violated constraints specific to instantiated graph update.",
    ExternalDevice => 911 => "This indicates that an async error has occurred in a device outside of CUDA. If CUDA was waiting for an external device's signal before consuming shared data, the external device signaled an error indicating that the data is not valid for consumption. This leaves the process in an inconsistent state and any further CUDA work will return the same error. To continue using CUDA, the process must be terminated and relaunched.",
    Unknown => 999 => "This indicates that an unknown internal error has occurred.",
    InvalidBufferSize => 1337 => "This error is raised when the given slice does not match in length with the GPU buffer.",
    InvalidGPUName => 1338 => "This error is raised when the name string given by the cuda driver is not properly NULL-terminated or it contains non ASCII / UTF-8 chars.",
}

/// Rustonic type for `CUdevice_attribute` which is used to query properties
/// of a device
#[repr(u32)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DeviceAttribute {
    /// Maximum number of threads per block
    MaxThreadsPerBlock = 1,
    /// Maximum block dimension X
    MaxBlockDimX = 2,
    /// Maximum block dimension Y
    MaxBlockDimY = 3,
    /// Maximum block dimension Z
    MaxBlockDimZ = 4,
    /// Maximum grid dimension X
    MaxGridDimX = 5,
    /// Maximum grid dimension Y
    MaxGridDimY = 6,
    /// Maximum grid dimension Z
    MaxGridDimZ = 7,
    /// Maximum shared memory available per block in bytes
    MaxSharedMemoryPerBlock = 8,
    /// Memory available on device for __constant__ variables in a CUDA C kernel in bytes
    TotalConstantMemory = 9,
    /// Warp size in threads
    WarpSize = 10,
    /// Maximum pitch in bytes allowed by memory copies
    MaxPitch = 11,
    /// Maximum number of 32-bit registers available per block
    MaxRegistersPerBlock = 12,
    /// Typical clock frequency in kilohertz
    ClockRate = 13,
    /// Alignment requirement for textures
    TextureAlignment = 14,
    /// Device can possibly copy memory and execute a kernel concurrently. Deprecated. Use instead CU_DEVICE_ATTRIBUTE_ASYNC_ENGINE_COUNT.
    GpuOverlap = 15,
    /// Number of multiprocessors on device
    MultiprocessorCount = 16,
    /// Specifies whether there is a run time limit on kernels
    KernelExecTimeout = 17,
    /// Device is integrated with host memory
    Integrated = 18,
    /// Device can map host memory into CUDA address space
    CanMapHostMemory = 19,
    /// Compute mode (See CUcomputemode for details)
    ComputeMode = 20,
    /// Maximum 1D texture width
    MaximumTexture1dWidth = 21,
    /// Maximum 2D texture width
    MaximumTexture2dWidth = 22,
    /// Maximum 2D texture height
    MaximumTexture2dHeight = 23,
    /// Maximum 3D texture width
    MaximumTexture3dWidth = 24,
    /// Maximum 3D texture height
    MaximumTexture3dHeight = 25,
    /// Maximum 3D texture depth
    MaximumTexture3dDepth = 26,
    /// Maximum 2D layered texture width
    MaximumTexture2dLayeredWidth = 27,
    /// Maximum 2D layered texture height
    MaximumTexture2dLayeredHeight = 28,
    /// Maximum layers in a 2D layered texture
    MaximumTexture2dLayeredLayers = 29,
    /// Alignment requirement for surfaces
    SurfaceAlignment = 30,
    /// Device can possibly execute multiple kernels concurrently
    ConcurrentKernels = 31,
    /// Device has ECC support enabled
    EccEnabled = 32,
    /// PCI bus ID of the device
    PciBusId = 33,
    /// PCI device ID of the device
    PciDeviceId = 34,
    /// Device is using TCC driver model
    TccDriver = 35,
    /// Peak memory clock frequency in kilohertz
    MemoryClockRate = 36,
    /// Global memory bus width in bits
    GlobalMemoryBusWidth = 37,
    /// Size of L2 cache in bytes
    L2CacheSize = 38,
    /// Maximum resident threads per multiprocessor
    MaxThreadsPerMultiprocessor = 39,
    /// Number of asynchronous engines
    AsyncEngineCount = 40,
    /// Device shares a unified address space with the host
    UnifiedAddressing = 41,
    /// Maximum 1D layered texture width
    MaximumTexture1dLayeredWidth = 42,
    /// Maximum layers in a 1D layered texture
    MaximumTexture1dLayeredLayers = 43,
    /// Deprecated, do not use.
    CanTex2dGather = 44,
    /// Maximum 2D texture width if CUDA_ARRAY3D_TEXTURE_GATHER is set
    MaximumTexture2dGatherWidth = 45,
    /// Maximum 2D texture height if CUDA_ARRAY3D_TEXTURE_GATHER is set
    MaximumTexture2dGatherHeight = 46,
    /// Alternate maximum 3D texture width
    MaximumTexture3dWidthAlternate = 47,
    /// Alternate maximum 3D texture height
    MaximumTexture3dHeightAlternate = 48,
    /// Alternate maximum 3D texture depth
    MaximumTexture3dDepthAlternate = 49,
    /// PCI domain ID of the device
    PciDomainId = 50,
    /// Pitch alignment requirement for textures
    TexturePitchAlignment = 51,
    /// Maximum cubemap texture width/height
    MaximumTexturecubemapWidth = 52,
    /// Maximum cubemap layered texture width/height
    MaximumTexturecubemapLayeredWidth = 53,
    /// Maximum layers in a cubemap layered texture
    MaximumTexturecubemapLayeredLayers = 54,
    /// Maximum 1D surface width
    MaximumSurface1dWidth = 55,
    /// Maximum 2D surface width
    MaximumSurface2dWidth = 56,
    /// Maximum 2D surface height
    MaximumSurface2dHeight = 57,
    /// Maximum 3D surface width
    MaximumSurface3dWidth = 58,
    /// Maximum 3D surface height
    MaximumSurface3dHeight = 59,
    /// Maximum 3D surface depth
    MaximumSurface3dDepth = 60,
    /// Maximum 1D layered surface width
    MaximumSurface1dLayeredWidth = 61,
    /// Maximum layers in a 1D layered surface
    MaximumSurface1dLayeredLayers = 62,
    /// Maximum 2D layered surface width
    MaximumSurface2dLayeredWidth = 63,
    /// Maximum 2D layered surface height
    MaximumSurface2dLayeredHeight = 64,
    /// Maximum layers in a 2D layered surface
    MaximumSurface2dLayeredLayers = 65,
    /// Maximum cubemap surface width
    MaximumSurfacecubemapWidth = 66,
    /// Maximum cubemap layered surface width
    MaximumSurfacecubemapLayeredWidth = 67,
    /// Maximum layers in a cubemap layered surface
    MaximumSurfacecubemapLayeredLayers = 68,
    /// Deprecated, do not use. Use cudaDeviceGetTexture1DLinearMaxWidth() or cuDeviceGetTexture1DLinearMaxWidth() instead.
    MaximumTexture1dLinearWidth = 69,
    /// Maximum 2D linear texture width
    MaximumTexture2dLinearWidth = 70,
    /// Maximum 2D linear texture height
    MaximumTexture2dLinearHeight = 71,
    /// Maximum 2D linear texture pitch in bytes
    MaximumTexture2dLinearPitch = 72,
    /// Maximum mipmapped 2D texture width
    MaximumTexture2dMipmappedWidth = 73,
    /// Maximum mipmapped 2D texture height
    MaximumTexture2dMipmappedHeight = 74,
    /// Major compute capability version number
    ComputeCapabilityMajor = 75,
    /// Minor compute capability version number
    ComputeCapabilityMinor = 76,
    /// Maximum mipmapped 1D texture width
    MaximumTexture1dMipmappedWidth = 77,
    /// Device supports stream priorities
    StreamPrioritiesSupported = 78,
    /// Device supports caching globals in L1
    GlobalL1CacheSupported = 79,
    /// Device supports caching locals in L1
    LocalL1CacheSupported = 80,
    /// Maximum shared memory available per multiprocessor in bytes
    MaxSharedMemoryPerMultiprocessor = 81,
    /// Maximum number of 32-bit registers available per multiprocessor
    MaxRegistersPerMultiprocessor = 82,
    /// Device can allocate managed memory on this system
    ManagedMemory = 83,
    /// Device is on a multi-GPU board
    MultiGpuBoard = 84,
    /// Unique id for a group of devices on the same multi-GPU board
    MultiGpuBoardGroupId = 85,
    /// Link between the device and the host supports native atomic operations (this is a placeholder attribute, and is not supported on any current hardware)
    HostNativeAtomicSupported = 86,
    /// Ratio of single precision performance (in floating-point operations per second) to double precision performance
    SingleToDoublePrecisionPerfRatio = 87,
    /// Device supports coherently accessing pageable memory without calling cudaHostRegister on it
    PageableMemoryAccess = 88,
    /// Device can coherently access managed memory concurrently with the CPU
    ConcurrentManagedAccess = 89,
    /// Device supports compute preemption.
    ComputePreemptionSupported = 90,
    /// Device can access host registered memory at the same virtual address as the CPU
    CanUseHostPointerForRegisteredMem = 91,
    /// cuStreamBatchMemOp and related APIs are supported.
    CanUseStreamMemOps = 92,
    /// 64-bit operations are supported in cuStreamBatchMemOp and related APIs.
    CanUse64BitStreamMemOps = 93,
    /// CU_STREAM_WAIT_VALUE_NOR is supported.
    CanUseStreamWaitValueNor = 94,
    /// Device supports launching cooperative kernels via cuLaunchCooperativeKernel
    CooperativeLaunch = 95,
    /// Deprecated, cuLaunchCooperativeKernelMultiDevice is deprecated.
    CooperativeMultiDeviceLaunch = 96,
    /// Maximum optin shared memory per block
    MaxSharedMemoryPerBlockOptin = 97,
    /// The CU_STREAM_WAIT_VALUE_FLUSH flag and the CU_STREAM_MEM_OP_FLUSH_REMOTE_WRITES MemOp are supported on the device. See Stream memory operations for additional details.
    CanFlushRemoteWrites = 98,
    /// Device supports host memory registration via cudaHostRegister.
    HostRegisterSupported = 99,
    /// Device accesses pageable memory via the host's page tables.
    PageableMemoryAccessUsesHostPageTables = 100,
    /// The host can directly access managed memory on the device without migration.
    DirectManagedMemAccessFromHost = 101,
    /// Device supports virtual memory management APIs like cuMemAddressReserve, cuMemCreate, cuMemMap and related APIs
    VirtualMemoryManagementSupported = 102,
    /// Device supports exporting memory to a posix file descriptor with cuMemExportToShareableHandle, if requested via cuMemCreate
    HandleTypePosixFileDescriptorSupported = 103,
    /// Device supports exporting memory to a Win32 NT handle with cuMemExportToShareableHandle, if requested via cuMemCreate
    HandleTypeWin32HandleSupported = 104,
    /// Device supports exporting memory to a Win32 KMT handle with cuMemExportToShareableHandle, if requested via cuMemCreate
    HandleTypeWin32KmtHandleSupported = 105,
    /// Maximum number of blocks per multiprocessor
    MaxBlocksPerMultiprocessor = 106,
    /// Device supports compression of memory
    GenericCompressionSupported = 107,
    /// Maximum L2 persisting lines capacity setting in bytes.
    MaxPersistingL2CacheSize = 108,
    /// Maximum value of CUaccessPolicyWindow::num_bytes.
    MaxAccessPolicyWindowSize = 109,
    /// Device supports specifying the GPUDirect RDMA flag with cuMemCreate
    GpuDirectRdmaWithCudaVmmSupported = 110,
    /// Shared memory reserved by CUDA driver per block in bytes
    ReservedSharedMemoryPerBlock = 111,
    /// Device supports sparse CUDA arrays and sparse CUDA mipmapped arrays
    SparseCudaArraySupported = 112,
    /// Device supports using the cuMemHostRegister flag CU_MEMHOSTERGISTER_READ_ONLY to register memory that must be mapped as read-only to the GPU
    ReadOnlyHostRegisterSupported = 113,
    /// External timeline semaphore interop is supported on the device
    TimelineSemaphoreInteropSupported = 114,
    /// Device supports using the cuMemAllocAsync and cuMemPool family of APIs
    MemoryPoolsSupported = 115,
    /// Device supports GPUDirect RDMA APIs, like nvidia_p2p_get_pages (see https://docs.nvidia.com/cuda/gpudirect-rdma for more information)
    GpuDirectRdmaSupported = 116,
    /// The returned attribute shall be interpreted as a bitmask, where the individual bits are described by the CUflushGPUDirectRDMAWritesOptions enum
    GpuDirectRdmaFlushWritesOptions = 117,
    /// GPUDirect RDMA writes to the device do not need to be flushed for consumers within the scope indicated by the returned attribute. See CUGPUDirectRDMAWritesOrdering for the numerical values returned here.
    GpuDirectRdmaWritesOrdering = 118,
    /// Handle types supported with mempool based IPC
    MempoolSupportedHandleTypes = 119,
    /// Device supports deferred mapping CUDA arrays and CUDA mipmapped arrays
    DeferredMappingCudaArraySupported = 121,
}

pub fn get_driver_version() -> Result<isize, GPUError> {
    let mut result = 0;
    let error: GPUError = unsafe { cuDriverGetVersion(&mut result as *mut _) }.into();
    error.into_result(result as isize)
}

/// Fat pointer to a GPU buffer to simplify allocation, freeing, and copying
/// data to and from the GPU
pub struct GPUBuffer<T> {
    device_ptr: CUdeviceptr,
    len: usize,
    _marker: core::marker::PhantomData<T>,
}

/// Automatically free the buffer when its handle is out of scope
impl<T> std::ops::Drop for GPUBuffer<T> {
    fn drop(&mut self) {
        unsafe { cuMemFree_v2(self.device_ptr as _) };
    }
}

impl<T> GPUBuffer<T> {
    /// Copy the data from the CPU RAM to the GPU buffer
    pub fn copy_host2gpu(&mut self, src: &[T]) -> Result<(), GPUError> {
        if src.len() < self.len() {
            return Err(GPUError::InvalidBufferSize);
        }
        let error: GPUError = unsafe {
            cuMemcpyHtoD_v2(
                self.device_ptr,
                src.as_ptr() as _,
                self.len() * core::mem::size_of::<T>(),
            )
        }
        .into();
        error.into_result(())
    }

    /// Copy the buffer from the GPU to the CPU RAM
    pub fn copy_gpu2host(&self, dst: &mut [T]) -> Result<(), GPUError> {
        if dst.len() < self.len() {
            return Err(GPUError::InvalidBufferSize);
        }
        let error: GPUError = unsafe {
            cuMemcpyDtoH_v2(
                dst.as_mut_ptr() as _,
                self.device_ptr,
                self.len() * core::mem::size_of::<T>(),
            )
        }
        .into();
        error.into_result(())
    }

    /// Copy the buffer from the GPU to a new vector in the CPU RAM
    pub fn to_vec(&self) -> Result<Vec<T>, GPUError> {
        let mut result = Vec::with_capacity(self.len);
        unsafe { result.set_len(self.len) };
        self.copy_gpu2host(&mut result)?;
        Ok(result)
    }

    /// Get how many objects of type `T` the buffer can fit
    pub fn len(&self) -> usize {
        self.len
    }

    /// Get a raw pointer to pass to a kernel
    pub fn as_device_ptr(&self) -> CUdeviceptr {
        self.device_ptr
    }
}

/// How many threads / blocks / grids will be used to run the kernel
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Grid {
    block_x: usize,
    block_y: usize,
    block_z: usize,
    grid_x: usize,
    grid_y: usize,
    grid_z: usize,
}

impl core::default::Default for Grid {
    fn default() -> Self {
        Grid {
            block_x: 1,
            block_y: 1,
            block_z: 1,
            grid_x: 1,
            grid_y: 1,
            grid_z: 1,
        }
    }
}

impl Grid {
    pub fn set_block_x(mut self, block_x: usize) -> Result<Self, GPUError> {
        if block_x > i32::MAX as usize {
            return Err(GPUError::InvalidValue);
        }
        self.block_x = block_x;
        Ok(self)
    }

    pub fn set_block_y(mut self, block_y: usize) -> Result<Self, GPUError> {
        if block_y > 65535 {
            return Err(GPUError::InvalidValue);
        }
        self.block_y = block_y;
        Ok(self)
    }

    pub fn set_block_z(mut self, block_z: usize) -> Result<Self, GPUError> {
        if block_z > 65535 {
            return Err(GPUError::InvalidValue);
        }
        self.block_x = block_z;
        Ok(self)
    }

    pub fn set_grid_x(mut self, grid_x: usize) -> Result<Self, GPUError> {
        if grid_x > i32::MAX as usize {
            return Err(GPUError::InvalidValue);
        }
        self.grid_x = grid_x;
        Ok(self)
    }

    pub fn set_grid_y(mut self, grid_y: usize) -> Result<Self, GPUError> {
        if grid_y > 65535 {
            return Err(GPUError::InvalidValue);
        }
        self.grid_y = grid_y;
        Ok(self)
    }

    pub fn set_grid_z(mut self, grid_z: usize) -> Result<Self, GPUError> {
        if grid_z > 65535 {
            return Err(GPUError::InvalidValue);
        }
        self.grid_x = grid_z;
        Ok(self)
    }
}

/// Wrapper for a gpu kernel
pub struct Kernel(CUfunction);

/// Wrapper for a loaded module that contains callable kernels
pub struct PTX(CUmodule);

impl std::ops::Drop for PTX {
    fn drop(&mut self) {
        unsafe { cuModuleUnload(self.0) };
    }
}

impl PTX {
    /// Get a kernel by name from the module
    pub fn get_kernel(&mut self, kernel_name: &str) -> Result<Kernel, GPUError> {
        let func_name = CString::new(kernel_name).unwrap();
        let mut func: CUfunction = core::ptr::null_mut();
        let error: GPUError = unsafe {
            cuModuleGetFunction(&mut func as *mut CUfunction, self.0, func_name.as_ptr())
        }
        .into();
        error.into_result(Kernel(func))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Device(usize);

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        macro_rules! debug_fmt_device {
            {$($field:literal => $value:ident,)*} => {
                let mut d = f.debug_struct("Device");
                let d = d.field("device_id", &self.0);
                let d = match self.get_name() {
                    Ok(value) => d.field("name", &value),
                    Err(error) => d.field("name", &Result::<(), _>::Err(error)),
                };
                let d = match self.get_total_mem() {
                    Ok(value) => d.field("total_mem", &value),
                    Err(error) => d.field("total_mem", &Result::<(), _>::Err(error)),
                };

                $(
                    let d = match self.get_attribute(DeviceAttribute::$value) {
                        Ok(value) => d.field($field, &value),
                        Err(error) => d.field($field, &Result::<(), _>::Err(error)),
                    };
                )*

                let d = match self.get_grid_limits() {
                    Ok(value) => d.field("grid_limits", &value),
                    Err(error) => d.field("grid_limits", &Result::<(), _>::Err(error)),
                };
                d.finish()
            };
        }

        debug_fmt_device! {
            "compute_mode" => ComputeMode,
            "compute_capability_major" => ComputeCapabilityMajor,
            "compute_capability_minor" => ComputeCapabilityMinor,
            "pci_bus_id" => PciBusId,
            "pci_device_id" => PciDeviceId,
            "pci_domain_id" => PciDomainId,
            "multi_gpu_board" => MultiGpuBoard,
            "multi_gpu_board_group_id" => MultiGpuBoardGroupId,
            "clock_rate_khz" => ClockRate,
            "memory_clock_rate_khz" => MemoryClockRate,
            "global_memory_buswidth" => GlobalMemoryBusWidth,
            "single_to_double_precision_perf_ratio" => SingleToDoublePrecisionPerfRatio,
            "total_const_memory" => TotalConstantMemory,
            "max_persisting_l2_cache_size" => MaxPersistingL2CacheSize,
            "wrap_size" => WarpSize,
            "multiprocessor_count" => MultiprocessorCount,
            "max_threads_per_multiprocessor" => MaxThreadsPerMultiprocessor,
            "max_shared_memory_per_multiprocessor" => MaxSharedMemoryPerMultiprocessor,
            "max_registers_per_multiprocessor" => MaxRegistersPerMultiprocessor,
            "max_blocks_per_multiprocessor" => MaxBlocksPerMultiprocessor,
            "max_shared_memory_per_block" => MaxSharedMemoryPerBlock,
            "max_registers_per_block" => MaxRegistersPerBlock,
            "reserved_shared_memory_per_block" => ReservedSharedMemoryPerBlock,
            "max_pitch" => MaxPitch,
            "sparse_cuda_array_supported" => SparseCudaArraySupported,
            "kernel_exec_timeout" => KernelExecTimeout,
            "concurrent_kernels" => ConcurrentKernels,
            "async_engine_count" => AsyncEngineCount,
            "unified_addressing" => UnifiedAddressing,
            "generic_compression_supported" => GenericCompressionSupported,
            "can_map_host_memory" => CanMapHostMemory,
            "pageable_memory_access_use_host_page_tables" => PageableMemoryAccessUsesHostPageTables,
            "direct_managed_mem_Access_from_host" => DirectManagedMemAccessFromHost,
            "managed_memory" => ManagedMemory,
            "concurred_managed_access" => ConcurrentManagedAccess,
            "can_use_host_pointer_for_registered_mem" => CanUseHostPointerForRegisteredMem,
        }
    }
}

impl Device {
    pub fn new(device_id: usize) -> Result<Device, GPUError> {
        // Check that the device asked is reasonable
        if device_id > Device::get_device_count()? {
            return Err(GPUError::InvalidDevice);
        }
        Ok(Device(device_id))
    }

    pub fn get_name(&self) -> Result<String, GPUError> {
        // allocate the buffer
        let mut buffer = Vec::with_capacity(256);
        // into raw parts
        let (ptr, capacity) = (buffer.as_mut_ptr(), buffer.capacity());
        // forget the buffer so we don't get a double free when `result` will
        // be freed
        core::mem::forget(buffer);
        // fill the buffer
        let error: GPUError =
            unsafe { cuDeviceGetName(ptr as *mut _, capacity as _, self.0 as _) }.into();
        // return if error
        error.into_result(())?;

        // the string is null-terminated so we need to compute the length to get
        // a proper rust string
        let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, capacity) };

        let len = slice.iter().position(|b| *b == b'\0').unwrap_or(capacity);

        let result = unsafe { String::from_raw_parts(ptr, len, capacity) };
        // TODO!: should we validate that it's proper ASCII?
        Ok(result)
    }

    pub fn get_attribute(&self, attribute: DeviceAttribute) -> Result<isize, GPUError> {
        // yes this is a crime against nature, but our enum has docs and the
        // crates one doesn't, this should be safe as both have `#[repr(u32)]`
        let attr =
            unsafe { core::mem::transmute::<DeviceAttribute, CUdevice_attribute>(attribute) };
        let mut result = 0;
        let error: GPUError =
            unsafe { cuDeviceGetAttribute(&mut result as *mut _, attr, self.0 as _) }.into();
        error.into_result(result as isize)
    }

    /// Returns the total amount of memory available on the device in bytes.
    pub fn get_total_mem(&self) -> Result<usize, GPUError> {
        let mut result = 0;
        let error: GPUError =
            unsafe { cuDeviceTotalMem_v2(&mut result as *mut _, self.0 as _) }.into();
        error.into_result(result)
    }

    pub fn get_devices() -> Result<Vec<Device>, GPUError> {
        Ok((0..Device::get_device_count()?)
            .map(|i| Device(i))
            .collect())
    }

    /// Get the number of available devices
    pub fn get_device_count() -> Result<usize, GPUError> {
        // Init the cuda library if this wasn't already done
        // This should be a mutex, and not an atomic since other threads could
        // go on and call driver methods while this is being initzializzated
        // but if you are trying to concurrently initialize GPU devices fuck you
        if !CUDA_DRIVERS_HAVE_BEEN_INITIALIZED.swap(true, Ordering::SeqCst) {
            unsafe { cuInit(0) };
        }

        let mut number_of_devices = 0;
        let error: GPUError = unsafe { cuDeviceGetCount(&mut number_of_devices as *mut _) }.into();
        error.into_result(number_of_devices as usize)
    }

    /// Return the Max dimensions for blocks and grid for this device
    pub fn get_grid_limits(&self) -> Result<Grid, GPUError> {
        Ok(Grid {
            block_x: self.get_attribute(DeviceAttribute::MaxBlockDimX)? as usize,
            block_y: self.get_attribute(DeviceAttribute::MaxBlockDimY)? as usize,
            block_z: self.get_attribute(DeviceAttribute::MaxBlockDimZ)? as usize,
            grid_x: self.get_attribute(DeviceAttribute::MaxGridDimX)? as usize,
            grid_y: self.get_attribute(DeviceAttribute::MaxGridDimY)? as usize,
            grid_z: self.get_attribute(DeviceAttribute::MaxGridDimZ)? as usize,
        })
    }
}

/// Wrapper for the context and stream of a device
pub struct GPU {
    #[allow(dead_code)]
    // currently it' not used but we will need in the soon future to query
    // the GPU properties
    device: CUdevice,
    context: CUcontext,
    stream: CUstream,
}

/// Automatically free the buffer when its handle is out of scope
impl std::ops::Drop for GPU {
    fn drop(&mut self) {
        unsafe { cuStreamDestroy_v2(self.stream as _) };
        unsafe { cuCtxDestroy_v2(self.context as _) };
    }
}

impl GPU {
    /// Create a new GPU contex and stream from a device
    pub fn new(device: Device) -> Result<Self, GPUError> {
        // Get the first available device
        let mut device: CUdevice = device.0 as _;
        let error: GPUError = unsafe { cuDeviceGet(&mut device as *mut CUdevice, 0) }.into();
        if error != GPUError::Success {
            return Err(error);
        }

        // create a context
        let mut context: CUcontext = core::ptr::null_mut();
        let error: GPUError = unsafe {
            cuCtxCreate_v2(
                &mut context as *mut CUcontext,
                0 as _, // CU_CTX_SCHED_AUTO
                device,
            )
        }
        .into();
        if error != GPUError::Success {
            return Err(error);
        }

        // Create a stream
        let mut stream = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        let error: GPUError = unsafe { cuStreamCreate(&mut stream as *mut CUstream, 0) }.into();
        if error != GPUError::Success {
            return Err(error);
        }

        Ok(GPU {
            device,
            context,
            stream,
        })
    }

    pub fn load_ptx(&mut self, ptx: &str) -> Result<PTX, GPUError> {
        let mut module: CUmodule = core::ptr::null_mut();
        let file = CString::new(ptx).unwrap();
        let error: GPUError =
            unsafe { cuModuleLoadData(&mut module as *mut CUmodule, file.as_ptr() as *const _) }
                .into();
        error.into_result(PTX(module))
    }

    /// Wait for the GPU to finish all the launched kernels
    pub fn synchronize(&mut self) -> Result<(), GPUError> {
        let error: GPUError = unsafe { cuStreamSynchronize(self.stream) }.into();
        error.into_result(())
    }

    pub fn launch_kernel(
        &mut self,
        kernel: &Kernel,
        grid: &Grid,
        args: &mut [*mut c_void],
    ) -> Result<(), GPUError> {
        let error = unsafe {
            cuLaunchKernel(
                kernel.0,
                grid.block_x as _,
                grid.block_y as _,
                grid.block_z as _,
                grid.grid_x as _,
                grid.grid_y as _,
                grid.grid_z as _,
                0,
                self.stream,
                args.as_mut_ptr(),
                core::ptr::null_mut(),
            )
        };
        if error != cudaError_enum::CUDA_SUCCESS {
            return Err(GPUError::from(error as usize));
        }
        Ok(())
    }

    fn allocate_buffer<T>(&mut self, len: usize) -> Result<GPUBuffer<T>, GPUError> {
        let mut device_ptr: CUdeviceptr = 0;
        let error: GPUError = unsafe {
            cuMemAlloc_v2(
                (&mut device_ptr) as *mut CUdeviceptr,
                len * core::mem::size_of::<T>(),
            )
        }
        .into();
        error.into_result(GPUBuffer {
            device_ptr,
            len,
            _marker: core::marker::PhantomData::default(),
        })
    }

    /// Create a new GPU buffer with a copy of the data form the given slice
    pub fn buffer_from_slice<T>(&mut self, src: &[T]) -> Result<GPUBuffer<T>, GPUError> {
        let mut result = self.allocate_buffer::<T>(src.len())?;
        result.copy_host2gpu(src)?;
        Ok(result)
    }

    /// Create a new GPU buffer from a slice with no initializzation that can
    /// store `len` objects of type `T`
    pub unsafe fn buffer_uninitialized<T>(&mut self, len: usize) -> Result<GPUBuffer<T>, GPUError> {
        self.allocate_buffer::<T>(len)
    }
}
