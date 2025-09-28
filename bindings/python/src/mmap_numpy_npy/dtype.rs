use numpy::npyffi::NPY_TYPES;
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Dtype {
    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    F32,
    I64,
    U64,
    F64,
}

impl ToString for Dtype {
    fn to_string(&self) -> std::string::String {
        match self {
            Dtype::Bool => "bool",
            Dtype::I8 => "i8",
            Dtype::U8 => "u8",
            Dtype::I16 => "i16",
            Dtype::U16 => "u16",
            Dtype::I32 => "i32",
            Dtype::U32 => "u32",
            Dtype::F32 => "f32",
            Dtype::I64 => "i64",
            Dtype::U64 => "u64",
            Dtype::F64 => "f64",
        }
        .to_string()
    }
}

impl TryFrom<&str> for Dtype {
    type Error = String;

    fn try_from(dtype: &str) -> Result<Dtype, String> {
        Ok(match dtype {
            "bool" => Dtype::Bool,
            "i8" => Dtype::I8,
            "u8" => Dtype::U8,
            "i16" => Dtype::I16,
            "u16" => Dtype::U16,
            "i32" => Dtype::I32,
            "u32" => Dtype::U32,
            "f32" => Dtype::F32,
            "i64" => Dtype::I64,
            "u64" => Dtype::U64,
            "f64" => Dtype::F64,
            _ => {
                return Err(format!(
                    "The provided type {:?} is not supported!",
                    dtype.to_string()
                ))
            }
        })
    }
}

impl TryFrom<NPY_TYPES> for Dtype {
    type Error = String;

    fn try_from(numpy_types: NPY_TYPES) -> Result<Dtype, String> {
        use NPY_TYPES::*;
        Ok(match numpy_types {
            NPY_BOOL => Dtype::Bool,
            NPY_BYTE => Dtype::I8,
            NPY_UBYTE => Dtype::U8,
            NPY_SHORT => Dtype::I16,
            NPY_USHORT => Dtype::U16,
            NPY_INT => Dtype::I32,
            NPY_UINT => Dtype::U32,
            NPY_FLOAT => Dtype::F32,
            NPY_LONGLONG => Dtype::I64,
            NPY_ULONGLONG => Dtype::U64,
            NPY_DOUBLE => Dtype::F64,
            _ => {
                return Err(format!(
                    "The provided type {:?} is not supported!",
                    numpy_types
                ))
            }
        })
    }
}

impl From<Dtype> for NPY_TYPES {
    fn from(val: Dtype) -> Self {
        use NPY_TYPES::*;
        match val {
            Dtype::Bool => NPY_BOOL,
            Dtype::I8 => NPY_BYTE,
            Dtype::U8 => NPY_UBYTE,
            Dtype::I16 => NPY_SHORT,
            Dtype::U16 => NPY_USHORT,
            Dtype::I32 => NPY_INT,
            Dtype::U32 => NPY_UINT,
            Dtype::F32 => NPY_FLOAT,
            Dtype::I64 => NPY_LONGLONG,
            Dtype::U64 => NPY_ULONGLONG,
            Dtype::F64 => NPY_DOUBLE,
        }
    }
}

pub trait ToNumpyDtype {
    const NUMPY_DTYPE: Dtype;
}

macro_rules! impl_to_numpy_dtype {
    ($($t:ty => $d:ident),*) => {$(
impl ToNumpyDtype for $t {
    const NUMPY_DTYPE: Dtype = Dtype::$d;
}
    )*};
}

impl_to_numpy_dtype! {
    bool => Bool,
    u8 => U8,
    i8 => I8,
    u16 => U16,
    i16 => I16,
    u32 => U32,
    i32 => I32,
    u64 => U64,
    i64 => I64,
    f32 => F32,
    f64 => F64
}
