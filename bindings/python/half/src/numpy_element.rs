use crate::f16;
use numpy::npyffi::NPY_TYPES;
use numpy::pyo3::Python;
use numpy::{Element, PyArrayDescr, PY_ARRAY_API};

unsafe impl Element for f16 {
    const IS_COPY: bool = true;
    fn get_dtype(py: Python) -> &PyArrayDescr {
        unsafe {
            let descr = PY_ARRAY_API.PyArray_DescrFromType(py, NPY_TYPES::NPY_HALF as _);
            py.from_owned_ptr(descr as _)
        }
    }
}
