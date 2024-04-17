use super::{Dtype, ToNumpyDtype};
use numpy::npyffi::*;
use numpy::*;
use pyo3::*;

#[pyclass]
/// Reimplenting stuff because the numpy don't expone it :)
pub struct PySliceContainer {
    ptr: usize,
    len: usize,
    cap: usize,
    dtype: Dtype,
}

impl<T: ToNumpyDtype> From<Vec<T>> for PySliceContainer {
    fn from(mut data: Vec<T>) -> Self {
        // stable version of into_raw_parts I guess
        let res = PySliceContainer {
            ptr: data.as_mut_ptr() as usize,
            len: data.len(),
            cap: data.capacity(),
            dtype: T::NUMPY_DTYPE,
        };

        // leak the data, we will free it properly on drop
        core::mem::forget(data);

        res
    }
}

impl Drop for PySliceContainer {
    fn drop(&mut self) {
        macro_rules! impl_dtype_drop {
            ($($t:ty),*) => {

        match self.dtype {
            $(
            <$t>::NUMPY_DTYPE => {
                let _ = unsafe {Vec::from_raw_parts(
                    self.ptr as *mut $t, self.len, self.cap
                )};
            },
        )*}
            };
        }

        impl_dtype_drop! {
            bool,
            u8,
            i8,
            u16,
            i16,
            u32,
            i32,
            u64,
            i64,
            f32,
            f64
        }
    }
}

pub fn to_numpy_array<'py, T: ToNumpyDtype>(
    py: Python<'py>,
    mut data: Vec<T>,
    shape: &[usize],
    fortran_order: bool,
) -> Result<Py<PyAny>, String> {
    let num_of_elements = shape.iter().fold(1, |a, b| a * b);

    if data.len() != num_of_elements {
        return Err(format!(
            "Wrong shape {:?} for the given vector of len {}",
            shape,
            data.len(),
        ));
    }

    let mut flags = NPY_ARRAY_WRITEABLE | NPY_ARRAY_ALIGNED;
    if fortran_order {
        flags |= NPY_ARRAY_F_CONTIGUOUS;
    } else {
        flags |= NPY_ARRAY_C_CONTIGUOUS;
    };

    let ptr = data.as_mut_ptr();

    let container = PyClassInitializer::from(PySliceContainer::from(data))
        .create_cell(py)
        .expect("Failed to create slice container");

    let dt: NPY_TYPES = T::NUMPY_DTYPE.into();
    Ok(unsafe {
        let ptr = PY_ARRAY_API.PyArray_New(
            py,
            PY_ARRAY_API.get_type_object(py, npyffi::NpyTypes::PyArray_Type),
            shape.len() as _,
            shape.as_ptr() as _,
            dt as i32,
            core::ptr::null_mut(),
            ptr as *mut _,
            core::mem::size_of::<T>() as _,
            flags,
            core::ptr::null_mut(),
        );

        // set the fake vec as the Base pointer so that the lifetime of the vec
        // is constrained by this array
        let result = PY_ARRAY_API.PyArray_SetBaseObject(
            py,
            ptr as *mut npyffi::PyArrayObject,
            container as *mut ffi::PyObject,
        );
        if result != 0 {
            panic!("Cant set base object")
        }
        Py::from_owned_ptr(py, ptr)
    })
}
