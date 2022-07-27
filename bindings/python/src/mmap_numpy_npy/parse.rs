use regex::Regex;
use libc::intptr_t;
use numpy::npyffi::NPY_TYPES;

pub(crate) fn parse_header(header: &str) -> (NPY_TYPES, Vec<intptr_t>, bool) {
    let descr_re = Regex::new(r"'descr'\s*:\s*'([^']+)'").unwrap();
    let shape_re = Regex::new(r"'shape'\s*:\s*\(([^\)]+)\)").unwrap();
    let fortran_order_re = Regex::new(r"'fortran_order'\s*:\s*(True|False)").unwrap();

    let descr_str = descr_re.captures(header).unwrap().get(1).unwrap().as_str();
    let shape_str = shape_re.captures(header).unwrap().get(1).unwrap().as_str();
    let fortran_order_str = fortran_order_re.captures(header).unwrap().get(1).unwrap().as_str();

    let fortran_order = fortran_order_str == "True";
    let shape = shape_str.split(",")
        .filter(|chunk| chunk.trim().is_empty())
        .map(|chunk| chunk.trim().parse::<intptr_t>().unwrap())
        .collect::<Vec<intptr_t>>();

    #[cfg(target_endian="little")]
    assert_eq!(&descr_str[..1], "<");
    #[cfg(target_endian="big")]
    assert_eq!(&descr_str[..1], ">");
    let byte_size = descr_str[2..].parse::<usize>().unwrap();

    let descr = match &descr_str[1..2] {
        "u" => match byte_size {
            1  => NPY_TYPES::NPY_UBYTE,
            2 => NPY_TYPES::NPY_USHORT,
            4 => NPY_TYPES::NPY_UINT,
            8 => NPY_TYPES::NPY_ULONGLONG,
            _ => panic!("Bit size {} not supported fior", byte_size),
        },
        "i" => match byte_size {
            1  => NPY_TYPES::NPY_BYTE,
            2 => NPY_TYPES::NPY_SHORT,
            4 => NPY_TYPES::NPY_INT,
            8 => NPY_TYPES::NPY_LONGLONG,
            _ => panic!("Bit size {} not supported fior", byte_size),
        },
        "c" => panic!("complex type is not supported"),
        "f" => match byte_size {
            4 => NPY_TYPES::NPY_FLOAT,
            8 => NPY_TYPES::NPY_DOUBLE,
            _ => panic!("Bit size {} not supported fior", byte_size),
        },
        "b" => NPY_TYPES::NPY_BOOL,
        "V" => panic!("void type is not supported"),
        "O" => panic!("object type is not supported"),
        "M" => panic!("datetime type is not supported"),
        "m" => panic!("timedelta type is not supported"),
        "S" => panic!("bytes type is not supported"),
        "U" => panic!("string type is not supported"),
        _ => panic!("Type {} not supported", descr_str),
    };

    (descr, shape, fortran_order)
}

pub(crate) fn dtype_to_descr(dtype: NPY_TYPES) -> &'static str {
    use NPY_TYPES::*;
    match dtype {
        NPY_BOOL      => "b",
        NPY_BYTE      => "i1", 
        NPY_UBYTE     => "u1",
        NPY_SHORT     => "i2",
        NPY_USHORT    => "u2",
        NPY_INT       => "i4",
        NPY_UINT      => "u4",
        NPY_FLOAT     => "f4",
        NPY_LONGLONG  => "i8",
        NPY_ULONGLONG => "u8",
        NPY_DOUBLE    => "f8",
        _ => panic!("Not supported {}", dtype as i32),
    }
}

pub(crate) fn npy_type_to_bytes_size(dtype: NPY_TYPES) -> usize {
    use NPY_TYPES::*;
    match dtype {
        NPY_BOOL | NPY_BYTE | NPY_UBYTE => 1,
        NPY_SHORT | NPY_USHORT => 2,
        NPY_INT | NPY_UINT | NPY_FLOAT => 4,
        NPY_LONGLONG | NPY_ULONGLONG | NPY_DOUBLE => 8,
        _ => panic!("Not supported {}", dtype as i32),
    }
}
