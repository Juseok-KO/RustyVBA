use core::datatype::{TypeCode, Data, Value, string::CSTRING};
use core::Pointer;

pub trait VbaInterface {

    fn note() -> &'static str;

    fn args_name() -> &'static [&'static str];

    fn args_type() -> &'static [TypeCode];

    fn call_func(ptr_args: *mut Pointer, ptr_err: *mut bool) -> *mut Pointer;
}


/// assumption: args are provided in the form of Vec<Data>
pub fn parse_args<'a>(ptr_args: *mut Pointer, _lifetime: &'a ()) -> Result<&'a Vec<Data>, String> {

    let ref_arg = unsafe { &*(ptr_args as *mut Data) };

    // The args whoudl be provided as 2 dimensional array
    let Value::Array(args) = ref_arg.get_value() else {
        return Err(format!("Provided args is not an Array: {:?}", ref_arg.get_type()))
    };

    // only the first row of ptr_args has relevant data
    let Some(Value::Array(first_row)) = args.get(0).and_then(|args| Some(args.get_value())) else {
        return Err(format!("The provided args should be the 2-dimensional array"))
    };

    Ok(first_row)
}

#[macro_export]
macro_rules! convert_into_dll_funcs {

    ($type:ty) => {

        #[unsafe(no_mangle)]
        pub extern "C" fn note() -> *mut Pointer {
            Data::from(CSTRING::from(String::from(<$type>::note()))).into_raw_pointer()
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn args_info() -> *mut Pointer {
            Data::from(vec![
                Data::from(<$type>::args_name().into_iter().map(|name| Data::from(CSTRING::from(name.to_string())))
                .collect::<Vec<Data>>()),
                
                Data::from(<$type>::args_type().into_iter().map(|t_code| Data::from(CSTRING::from(t_code.to_string())))
                .collect::<Vec<Data>>())
            ])
            .into_raw_pointer()
            
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn call_func(ptr_args: *mut Pointer, ptr_err: *mut bool) -> *mut Pointer {
            <$type>::call_func(ptr_args, ptr_err)
        }
    }
}