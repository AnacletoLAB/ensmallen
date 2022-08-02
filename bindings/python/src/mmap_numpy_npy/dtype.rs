use numpy::npyffi::NPY_TYPES;
use std::convert::TryFrom;

#[derive(Clone, Debug)]
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

impl Into<NPY_TYPES> for Dtype {
    fn into(self) -> NPY_TYPES {
        use NPY_TYPES::*;
        match self {
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
