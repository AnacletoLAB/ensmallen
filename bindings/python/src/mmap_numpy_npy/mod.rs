use numpy::*;

use ::pyo3::conversion::AsPyPointer;
use ::pyo3::prelude::*;
use libc::intptr_t;
use mmap::*;
use numpy::npyffi::NPY_TYPES;

mod parse;
use parse::*;

mod dtype;
pub use dtype::*;

mod to_numpy;
pub use to_numpy::*;

const ARRAY_ALIGN: usize = 64;

/// Utility type to store the mmap the Python heap so that we can ensure proper drop
#[pyclass]
pub(crate) struct PyMemoryMapped {
    _mmap: MemoryMapped,
}

impl From<MemoryMapped> for PyMemoryMapped {
    fn from(other: MemoryMapped) -> Self {
        Self { _mmap: other }
    }
}

unsafe impl Send for PyMemoryMapped {}

pub fn create_memory_mapped_numpy_array(
    py: Python,
    path: Option<&str>,
    dtype: Dtype,
    shape: &[intptr_t],
    fortran_order: bool,
) -> Py<PyAny> {
    let dtype = dtype.into();

    let num_of_elements = shape.iter().product::<isize>();
    let data_size = num_of_elements * npy_type_to_bytes_size(dtype) as isize;

    #[cfg(target_endian = "little")]
    let endianess = '<';
    #[cfg(target_endian = "big")]
    let endianess = '>';

    let mut header = b"\x93NUMPY\x02\x00".to_vec();

    let description = format!(
        "{{'descr': '{endianess}{descr}', 'fortran_order': {fortran_order}, 'shape': ({shape}), }}",
        endianess = endianess,
        descr = dtype_to_descr(dtype),
        fortran_order = if fortran_order { "True" } else { "False" },
        shape = shape
            .iter()
            .map(|x| format!("{}, ", x))
            .collect::<Vec<String>>()
            .join(""),
    );

    let offset = description.len() + 12;
    let padding_size = (ARRAY_ALIGN - (offset % ARRAY_ALIGN)) % ARRAY_ALIGN;
    let aligned_len = offset + padding_size;

    assert!(
        aligned_len.is_multiple_of(ARRAY_ALIGN),
        "Error in the computation of the alignement of the npy header {} % {}",
        aligned_len,
        ARRAY_ALIGN,
    );

    header.extend_from_slice(&(aligned_len as u32 - 12).to_le_bytes());
    header.extend_from_slice(description.as_bytes());

    for _ in 0..padding_size {
        header.push(b' ');
    }

    // mmap the file
    let mut mmap = MemoryMapped::new_mut(path, Some(aligned_len + data_size as usize), None)
        .expect("Could not mmap the file");
    // write the header to the file
    mmap.get_slice_mut::<u8>(0, Some(aligned_len))
        .unwrap()
        .clone_from_slice(&header);

    let data = mmap
        .get_slice_mut::<u8>(aligned_len, None)
        .expect("Could get the data slice type.")
        .as_ptr();

    // put the mmap in the python heap to ensure memory safety
    let container = PyClassInitializer::from(PyMemoryMapped::from(mmap))
        .create_cell(py)
        .expect("Failed to create slice container");

    // do the magic
    

    unsafe {
        use numpy::npyffi::*;

        let mut flags = NPY_ARRAY_WRITEABLE | NPY_ARRAY_ALIGNED;
        if fortran_order {
            flags |= NPY_ARRAY_F_CONTIGUOUS;
        } else {
            flags |= NPY_ARRAY_C_CONTIGUOUS;
        };

        // create the new numpy array
        let ptr = PY_ARRAY_API.PyArray_New(
            py,
            PY_ARRAY_API.get_type_object(py, npyffi::NpyTypes::PyArray_Type),
            shape.len() as _,
            shape.as_ptr() as _,
            dtype as i32,
            core::ptr::null_mut(),
            data as *mut _,
            0,
            flags,
            core::ptr::null_mut(),
        );

        // set the mmap as the Base pointer so that the lifetime of the mmap
        // is constrained by this array
        let result = PY_ARRAY_API.PyArray_SetBaseObject(py, ptr as _, (*container).as_ptr());

        if result != 0 {
            panic!("Cant set base object")
        }
        Py::from_owned_ptr(py, ptr)
    }
}

pub fn load_memory_mapped_numpy_array(
    py: Python,
    path: Option<&str>,
) -> (NPY_TYPES, bool, Py<PyAny>) {
    // mmap the file
    let mut mmap = MemoryMapped::new_mut(path, None, None).expect("Could not mmap the file");

    // check the magic
    let magic = mmap
        .get::<u64>(0)
        .expect("Could not find the numpy object len.")
        .to_be();
    assert_eq!(magic & (!0xFFFF), u64::from_be_bytes(*b"\x93NUMPY\x00\x00"));

    // get the version
    let major = *mmap.get::<u8>(6).expect("Could not read major version");
    let minor = *mmap.get::<u8>(7).expect("Could not read minor version");

    assert_eq!(minor, 0);

    // get the header
    let (header_start, header_length) = match major {
        1 => {
            let header_length = mmap
                .get::<u16>(8)
                .expect("Could not find the numpy object len.")
                .to_le();
            (10, header_length as usize)
        }
        2 | 3 => {
            let header_length = mmap
                .get::<u32>(8)
                .expect("Could not find the numpy object len.")
                .to_le();
            (12, header_length as usize)
        }
        _ => {
            panic!("Unsupported numpy major version {}", major);
        }
    };

    let header_bytes = mmap
        .get_slice::<u8>(header_start, Some(header_length))
        .expect("Could not get the header");
    let header = core::str::from_utf8(header_bytes).expect("Invalid header as utf-8");

    let (dtype, shape, fortran_order) = parse_header(header);

    // TODO! compute the actual size so we can boundcheck on load the mmaped file
    let data = mmap
        .get_slice_mut::<u8>(header_start + header_length, None)
        .expect("Could get the data slice type.")
        .as_ptr();

    // put the mmap in the python heap to ensure memory safety
    let container = PyClassInitializer::from(PyMemoryMapped::from(mmap))
        .create_cell(py)
        .expect("Failed to create slice container");

    // do the magic
    let result = unsafe {
        use numpy::npyffi::*;

        let mut flags = NPY_ARRAY_WRITEABLE | NPY_ARRAY_ALIGNED;
        if fortran_order {
            flags |= NPY_ARRAY_F_CONTIGUOUS;
        } else {
            flags |= NPY_ARRAY_C_CONTIGUOUS;
        };

        // create the new numpy array
        let ptr = PY_ARRAY_API.PyArray_New(
            py,
            PY_ARRAY_API.get_type_object(py, npyffi::NpyTypes::PyArray_Type),
            shape.len() as _,
            shape.as_ptr() as _,
            dtype as i32,
            core::ptr::null_mut(),
            data as *mut _,
            0,
            flags,
            core::ptr::null_mut(),
        );

        // set the mmap as the Base pointer so that the lifetime of the mmap
        // is constrained by this array
        let result = PY_ARRAY_API.PyArray_SetBaseObject(py, ptr as _, (*container).as_ptr());

        if result != 0 {
            panic!("Cant set base object")
        }
        Py::from_owned_ptr(py, ptr)
    };

    (dtype, fortran_order, result)
}
