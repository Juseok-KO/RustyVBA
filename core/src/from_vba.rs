use crate::Pointer;
use crate::datatype::{Data, Value};
use crate::datatype::string::CSTRING;

#[unsafe(no_mangle)]
pub extern "C" fn drop_data(ptr_data: *mut Pointer) -> bool {
    Data::drop(ptr_data);
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn get_type(ptr_data: *mut Pointer) -> i64 {
    Data::get_type_from_ptr(ptr_data) as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn get_i8(ptr_data: *const Pointer) -> i8 {
    Data::get_i8(ptr_data).unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_i16(ptr_data: *const Pointer) -> i16 {
    Data::get_i16(ptr_data).unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_i32(ptr_data: *const Pointer) -> i32 {
    Data::get_i32(ptr_data).unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_i64(ptr_data: *const Pointer) -> i64 {
    Data::get_i64(ptr_data).unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_f32(ptr_data: *const Pointer) -> f32 {
    Data::get_f32(ptr_data).unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_f64(ptr_data: *const Pointer) -> f64 {
    Data::get_f64(ptr_data).unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_bool(ptr_data: *const Pointer) -> bool {
    Data::get_bool(ptr_data).unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_ptr_str(ptr_data: *const Pointer) -> *const Pointer {
    Data::get_ptr_str(ptr_data).unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn init_array(row: i64, col: i64) -> *mut Pointer {
    Data::init_array(row, col)
}

#[unsafe(no_mangle)]
pub extern "C" fn arr_num_rows(ptr_arr: *const Pointer, ptr_result: *mut bool) -> *mut Pointer {

    match Data::get_arr_row(ptr_arr) {
        Ok(num) => {
            unsafe { *ptr_result = true };
            Data::from(num).into_raw_pointer()
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn arr_num_cols(ptr_arr: *const Pointer, ptr_result: *mut bool) -> *mut Pointer {

    match Data::get_arr_col(ptr_arr) {
        Ok(num) => {
            unsafe { *ptr_result = true };
            Data::from(num).into_raw_pointer()
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn arr_set_i8(ptr_arr:*mut Pointer, row: i64, col: i64, val: i8, ptr_result: *mut bool) -> *mut Pointer {

    match Data::set_i8(ptr_arr, row, col, val) {
        Ok(_) => {
            unsafe { *ptr_result = true };
            return 0 as *mut Pointer
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn arr_set_i16(ptr_arr: *mut Pointer, row: i64, col: i64, val: i16, ptr_result: *mut bool) -> *mut Pointer {
    match Data::set_i16(ptr_arr, row, col, val) {
        Ok(_) => {
            unsafe { *ptr_result = true };
            return 0 as *mut Pointer
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn arr_set_i32(ptr_arr: *mut Pointer, row: i64, col: i64, val: i32, ptr_result: *mut bool) -> *mut Pointer {
    match Data::set_i32(ptr_arr, row, col, val) {
        Ok(_) => {
            unsafe { *ptr_result = true };
            return 0 as *mut Pointer
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn arr_set_i64(ptr_arr: *mut Pointer, row: i64, col: i64, val: i64, ptr_result: *mut bool) -> *mut Pointer {
    match Data::set_i64(ptr_arr, row, col, val) {
        Ok(_) => {
            unsafe { *ptr_result = true };
            return 0 as *mut Pointer
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn arr_set_f32(ptr_arr: *mut Pointer, row: i64, col: i64, val: f32, ptr_result: *mut bool) -> *mut Pointer {
    match Data::set_f32(ptr_arr, row, col, val) {
        Ok(_) => {
            unsafe { *ptr_result = true };
            return 0 as *mut Pointer
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn arr_set_f64(ptr_arr: *mut Pointer, row: i64, col: i64, val: f64, ptr_result: *mut bool) -> *mut Pointer {
    match Data::set_f64(ptr_arr, row, col, val) {
        Ok(_) => {
            unsafe { *ptr_result = true };
            return 0 as *mut Pointer
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn arr_set_bool(ptr_arr: *mut Pointer, row: i64, col: i64, val: bool, ptr_result: *mut bool) -> *mut Pointer {
    match Data::set_bool(ptr_arr, row, col, val) {
        Ok(_) => {
            unsafe { *ptr_result = true };
            return 0 as *mut Pointer
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_elem_ptr(ptr_arr: *const Pointer, row:i64, col: i64, ptr_result: *mut bool) -> *const Pointer {
    match Data::get_ptr_arr_element(ptr_arr, row, col) {
        Ok(_) => {
            unsafe { *ptr_result = true };
            return 0 as *mut Pointer
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    }
}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
pub extern "C" fn string_test() -> *const Pointer {
    Data::from(CSTRING::from(String::from("Greeting from Rust!"))).into_raw_pointer()
}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
pub extern "C" fn string_pass_test(ptr_cstr: *const Pointer) -> *const Pointer {

    let returned = match crate::datatype::string::copy_from_cstr(ptr_cstr) {
        Ok(mut copied) => {
            copied.push_str(" <Rust>!");
            copied
        }
        Err(e) => {
            e
        }
    };

    Data::from(CSTRING::from(returned)).into_raw_pointer()

}
