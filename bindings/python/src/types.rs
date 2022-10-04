use super::*;
use half::f16;

pub struct ThreadDataRaceAware<T>
where
    T: ?Sized,
{
    pub(crate) t: T,
}

impl<T> From<T> for ThreadDataRaceAware<T> {
    fn from(value: T) -> Self {
        Self { t: value }
    }
}

unsafe impl<T> Sync for ThreadDataRaceAware<T> {}
unsafe impl<T> Send for ThreadDataRaceAware<T> {}

pub(crate) trait FromPyDict {
    fn from_pydict(py_kwargs: Option<&PyDict>) -> PyResult<Self>
    where
        Self: Sized;
}

pub(crate) enum NumpyArray<'a> {
    F16(&'a PyArray2<f16>),
    F32(&'a PyArray2<f32>),
    F64(&'a PyArray2<f64>),
    U8(&'a PyArray2<u8>),
    U16(&'a PyArray2<u16>),
    U32(&'a PyArray2<u32>),
    U64(&'a PyArray2<u64>),
    I8(&'a PyArray2<i8>),
    I16(&'a PyArray2<i16>),
    I32(&'a PyArray2<i32>),
    I64(&'a PyArray2<i64>),
}
