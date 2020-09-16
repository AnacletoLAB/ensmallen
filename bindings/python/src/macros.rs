
macro_rules! python_exception {
    ($value: expr, $msg: expr) => {
        match $value {
            Ok(v) => Ok(v),
            Err(_) => Err(PyErr::new::<exceptions::ValueError, _>($msg)),
        }
    };
}

macro_rules! to_python_exception {
    ($value: expr) => {
        match $value {
            Ok(v) => Ok(v),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    };
}

macro_rules! extract_value {
    ($kwargs: ident, $key: literal, $_type: ty) => {
        match $kwargs.get_item($key) {
            None => None,
            Some(v) => {
                if v.get_type().name() == "NoneType" {
                    None
                } else {
                    let extracted = v.extract::<$_type>();
                    Some(python_exception!(
                        extracted,
                        format!(
                            "The value passed for {} cannot be casted from {} to {}.",
                            $key,
                            v.get_type().name(),
                            stringify!($_type)
                        )
                    )?)
                }
            }
        }
    };
}

macro_rules! to_nparray_1d {
    ($gil: expr, $value: expr, $_type: ty) => {
        python_exception!(
            PyArray::from_vec($gil.python(), $value).cast::<$_type>(false),
            format!(
                "The given array cannot be casted to {}.",
                stringify!($_type)
            )
        )?
        .to_owned()
    };
}

macro_rules! to_nparray_2d {
    ($gil: expr, $value: expr, $_type: ty) => {
        python_exception!(
            python_exception!(
                PyArray::from_vec2($gil.python(), &$value),
                "The given value cannot be casted to a 2d numpy array."
            )?
            .cast::<$_type>(false),
            format!(
                "The given 2d array cannot be casted to {}.",
                stringify!($_type)
            )
        )?
        .to_owned()
    };
}

fn validate_kwargs(kwargs: &PyDict, columns: &[&str]) -> PyResult<()> {
    let mut keys: HashSet<&str> = kwargs
        .keys()
        .iter()
        .map(|v| v.extract::<&str>().unwrap())
        .collect();
    let columns: HashSet<&str> = columns.iter().cloned().collect();
    to_python_exception!(if keys.is_subset(&columns) {
        return Ok(());
    } else {
        for k in &columns {
            keys.remove(k);
        }
        let mut err_msg = String::new();
        for k in &keys {
            let (distance, column) = columns
                .iter()
                .map(|col| (edit_distance(k, col), col))
                .min_by_key(|x| x.0)
                .unwrap();

            if distance <= 2 {
                err_msg = format!(
                        "The passed argument {} is not a valid one.\n Did you mean {} ?\nThe available ones are: \n{:?}",
                        k, column, columns
                    );
                break;
            }
        }
        if err_msg.is_empty() {
            err_msg = format!(
                "The following arguments are not valid keyword arguments for this function. \n{:?}\n the available ones are: \n{:?}",
                keys, columns
            );
        }
        Err(err_msg)
    })
}