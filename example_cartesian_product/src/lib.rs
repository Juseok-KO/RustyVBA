use core::Pointer;
use core::datatype::{Data, Value, TypeCode, string::CSTRING};
use dll_interface::{VbaInterface, parse_args};
use dll_interface::convert_into_dll_funcs;

struct CartesianProduct;

impl VbaInterface for CartesianProduct {

    fn note() -> &'static str {
        "Produce the cartesian product"
    }

    fn args_name() -> &'static [&'static str] {
        &["arr1", "arr2"]
    }

    fn args_type() -> &'static [TypeCode] {
        &[TypeCode::ARRAY, TypeCode::ARRAY]
    }

    fn call_func(ptr_args: *mut Pointer, ptr_err: *mut bool) -> *mut Pointer {
        let lt = ();

        let Ok(mut args) = parse_args(ptr_args, &lt) else {
            unsafe { *ptr_err = false };
            return Data::from(CSTRING::from(format!("Failed to parse args"))).into_raw_pointer()
        };

        match (args.next().map(|a1| a1.get_value() ), args.next().map(|a2| a2.get_value())) {

            (Some(Value::Array(arr1)), Some(Value::Array(arr2))) => {

                let mut output = Vec::new();

                let mut arr1_iter = arr1.iter();
                while let Some(Value::Array(row1)) = arr1_iter.next().map(|r| r.get_value()) {
                    let mut arr2_iter = arr2.iter();
                    while let Some(Value::Array(row2)) = arr2_iter.next().map(|r| r.get_value()) {

                        let new_row = Data::from(row1.iter().chain(row2.iter()).map(|item| {
                            match item.get_value() {
                                Value::I8(i) => Data::from(*i),
                                Value::I16(i) => Data::from(*i),
                                Value::I32(i) => Data::from(*i),
                                Value::I64(i) => Data::from(*i),
                                Value::F32(f) => Data::from(*f),
                                Value::F64(f) => Data::from(*f),
                                Value::CSTRING(s) => {
                                    s.get_string().ok()
                                    .map_or_else( || Data::from(CSTRING::from(String::from("!String Parsing Failure"))),
                                    |string| Data::from(CSTRING::from(string)))
                                },
                                Value::BOOL(b) => Data::from(*b),
                                _ => Data::from(CSTRING::from(format!("!Not Supported Type: {:?}", item.get_type().to_string())))
                            }
                        })
                        .collect::<Vec<Data>>());
                        
                        output.push(new_row);
                    }
                }

                Data::from(output).into_raw_pointer()
            },
            _ => {
                unsafe { *ptr_err = false };
                Data::from(CSTRING::from(format!("Unexpected types of args: {:?}", args))).into_raw_pointer()
            }
        }
    }
}

convert_into_dll_funcs!(CartesianProduct);