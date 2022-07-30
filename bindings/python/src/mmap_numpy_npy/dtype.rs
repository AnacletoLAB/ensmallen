use numpy::npyffi::NPY_TYPES;

#[derive(Clone)]
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

impl Dtype {
    pub fn to_npy_type(&self) -> NPY_TYPES {
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
