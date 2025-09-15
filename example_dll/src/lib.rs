use core::Pointer;
use core::datatype::{Data, Value, TypeCode, string::CSTRING, string::copy_from_cstr};
use dll_interface::{VbaInterface, parse_args};
use dll_interface::convert_into_dll_funcs;

struct SampleDll;

impl VbaInterface for SampleDll {

    fn note() -> &'static str {
        "This is a Sample to demonstrate how to use this interface. It will append 'From Rust!' to String argument, and double any numeric arguments"
    }

    fn args_name() -> &'static [&'static str] {
        &["any singular arg"]       
    }

    fn args_type() -> &'static [TypeCode] {
        &[TypeCode::None]
    }

    fn call_func(ptr_args: *mut Pointer, ptr_err: *mut bool) -> *mut Pointer {
        let lt = ();

        let Ok(args) = parse_args(ptr_args, &lt) else {

            unsafe { *ptr_err = false };
            return Data::from(CSTRING::from(format!("Failed to parse args"))).into_raw_pointer()
        };

        // In this example, only single argument is supplied
        match args.get(0).and_then(|arg| Some(arg.get_value())) {
            Some(Value::CSTRING(cstr)) => {
                let Ok(mut string) = cstr.get_string() else {
                    unsafe { *ptr_err = false};
                    return Data::from(CSTRING::from(format!("Failed to parse cstr arg"))).into_raw_pointer()
                };

                string.push_str(" From Rust!");

                unsafe { *ptr_err = true };
                Data::from(CSTRING::from(string)).into_raw_pointer()
            }

            Some(Value::I8(num)) => {
                unsafe { *ptr_err = true };
                Data::from(num * 2).into_raw_pointer()
            }

            Some(Value::I16(num)) => {
                unsafe { *ptr_err = true };
                Data::from(num * 2).into_raw_pointer()
            }

            Some(Value::I32(num)) => {
                unsafe { *ptr_err = true };
                Data::from(num * 2).into_raw_pointer()
            }

            Some(Value::I64(num)) => {
                unsafe { *ptr_err = true };
                Data::from(num * 2).into_raw_pointer()
            }

            Some(Value::F32(fnum)) => {
                unsafe { *ptr_err = true };
                Data::from(fnum * 2.0).into_raw_pointer()
            }

            Some(Value::F64(fnum)) => {
                unsafe { *ptr_err = true };
                Data::from(fnum * 2.0).into_raw_pointer()
            }
            _ => {
                unsafe { *ptr_err = false };
                return Data::from(CSTRING::from(format!("Not supported type provided"))).into_raw_pointer()
            }
        }

    }

}

convert_into_dll_funcs!(SampleDll);