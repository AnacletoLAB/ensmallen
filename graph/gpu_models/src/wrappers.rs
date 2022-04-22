use cuda_driver_sys::*;
use cuda_runtime_sys::*;
use std::ffi::{CString, c_void};

/// Create arguments for a kernel
#[macro_export]
macro_rules! args {
    [$($value:expr,)*] => {
        &mut vec![
            $(
                &mut $value as *mut _ as *mut core::ffi::c_void, 
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

impl std::fmt::Debug for GPUError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use GPUError::*;
        match self {
            $($field => f.write_str($doc),)*
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

impl_gpu_error!{
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
        unsafe{cudaFree(self.device_ptr as _)};
    }
}

impl<T> GPUBuffer<T> {
    /// Copy the data from the CPU RAM to the GPU buffer
    pub fn copy_host2gpu(&mut self, src: &[T]) -> Result<(), GPUError> {
        if src.len() < self.len() {
            return Err(GPUError::InvalidBufferSize);
        }
        let error: GPUError = unsafe{
            cuMemcpyHtoD_v2(
                self.device_ptr,
                src.as_ptr() as _,
                self.len() * core::mem::size_of::<T>(),
            )
        }.into();
        error.into_result(())
    }

    /// Copy the buffer from the GPU to the CPU RAM
    pub fn copy_gpu2host(&self, dst: &mut [T]) -> Result<(), GPUError> {
        if dst.len() < self.len() {
            return Err(GPUError::InvalidBufferSize);
        }
        let error: GPUError = unsafe{
            cuMemcpyDtoH_v2(
                dst.as_mut_ptr() as _,
                self.device_ptr,
                self.len() * core::mem::size_of::<T>(),
            )
        }.into();
        error.into_result(())
    }

    /// Copy the buffer from the GPU to a new vector in the CPU RAM
    pub fn to_vec(&self) -> Result<Vec<T>, GPUError> {
        let mut result = Vec::with_capacity(self.len);
        unsafe{result.set_len(self.len)};
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

impl PTX {
    /// Get a kernel by name from the module
    pub fn get_kernel(&mut self, kernel_name: &str) -> Result<Kernel, GPUError> {
        let func_name = CString::new(kernel_name).unwrap();
        let mut func: CUfunction = core::ptr::null_mut();
        let error: GPUError = unsafe{cuModuleGetFunction(
            &mut func as *mut CUfunction, 
            self.0, 
            func_name.as_ptr(),
        )}.into();
        error.into_result(Kernel(func))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Device(usize);

impl Device {
    pub fn new(device_id: usize) -> Result<Device, GPUError> {
        // Check that the device asked is reasonable
        if device_id > Device::get_device_count()? {
            return Err(GPUError::InvalidDevice);
        }
        Ok(Device(device_id))
    }

    pub fn get_name(&self) -> Result<String, GPUError> {
        let props = self.get_properties()?;
        let bytes = unsafe{
            std::mem::transmute::<&[i8], &[u8]>(props.name.as_slice())
        };
        Ok(std::str::from_utf8(bytes).unwrap().to_string())
    }

    /// Get informations about a Device
    pub fn get_properties(&self) -> Result<cudaDeviceProp, GPUError> {
        let mut props: cudaDeviceProp = unsafe{core::mem::MaybeUninit::uninit().assume_init()};
        let error = unsafe{
            cudaGetDeviceProperties(
                &mut props as *mut _,
                self.0 as _,
            )
        };
        if error != cudaError::cudaSuccess {
            return Err(GPUError::from(error as usize));
        }
        Ok(props)
    }

    pub fn get_devices() -> Result<Vec<Device>, GPUError> {
        Ok((0..Device::get_device_count()?).map(|i| Device(i)).collect())
    }

    /// Get the number of available devices
    pub fn get_device_count() -> Result<usize, GPUError> {
        let mut number_of_devices = 0;
        let error = unsafe { cudaGetDeviceCount(&mut number_of_devices as *mut _) }; 
        if error != cudaError::cudaSuccess {
            return Err(GPUError::from(error as usize));
        }
        Ok(number_of_devices as usize)
    }   
}

/// Wrapper for the context and stream of a device
pub struct GPU {
    device: CUdevice,
    context: CUcontext,
    stream: CUstream,
}

/// Automatically free the buffer when its handle is out of scope
impl std::ops::Drop for GPU {
    fn drop(&mut self) {
        unsafe{cudaStreamDestroy(self.stream as _)};
        unsafe{cuCtxDestroy_v2(self.context as _)};
    }
}

impl GPU {
    /// Create a new GPU contex and stream from a device
    pub fn new(device: Device) -> Result<Self, GPUError> {
        // Init the cuda library
        unsafe { cuInit(0) };

        // Get the first available device
        let mut device: CUdevice = device.0 as _;
        let error: GPUError = unsafe{ cuDeviceGet(&mut device as *mut CUdevice, 0) }.into();    
        if error != GPUError::Success {
            return Err(error);
        }

        // create a context
        let mut context: CUcontext = core::ptr::null_mut();
        let error: GPUError = unsafe{ cuCtxCreate_v2(
            &mut context as *mut CUcontext, 
            cudaDeviceScheduleAuto, 
            device
        )}.into();  
        if error != GPUError::Success {
            return Err(error);
        }

        // Create a stream
        let mut stream = unsafe{core::mem::MaybeUninit::uninit().assume_init()};
        let error: GPUError = unsafe{ cuStreamCreate(
            &mut stream as *mut CUstream, 
            0,
        )}.into();  
        if error != GPUError::Success {
            return Err(error);
        }

        Ok(GPU{
            device,
            context,
            stream,
        })
    }

    pub fn load_ptx(&mut self, ptx: &str) -> Result<PTX, GPUError> {
        let mut module: CUmodule = core::ptr::null_mut();
        let file_name = CString::new(ptx).unwrap();
        let error: GPUError = unsafe{cuModuleLoad(
            &mut module as *mut CUmodule,
            file_name.as_ptr(),
        )}.into();  
        error.into_result(PTX(module))
    }

    /// Wait for the GPU to finish all the launched kernels
    pub fn synchronize(&mut self) -> Result<(), GPUError> {
        let error: GPUError = unsafe{ cuStreamSynchronize(self.stream) }.into();  
        error.into_result(())
    }

    pub fn launch_kernel(&mut self, kernel: Kernel, grid: Grid, args: &mut [*mut c_void]) -> Result<(), GPUError> {
        let error = unsafe{ cuLaunchKernel(
                kernel.0, 
                grid.block_x as _,
                grid.block_y as _,
                grid.block_z as _,
                grid.grid_x  as _,
                grid.grid_y  as _,
                grid.grid_z  as _,
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
                len * core::mem::size_of::<T>()
            )
        }.into();  
        error.into_result(GPUBuffer{
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