use core::Pointer;
use core::datatype::{Data, Value, string::{CSTRING, copy_from_cstr}};
use dll_loader::DLL;

use std::path::{Path, PathBuf};

/// It seems that VBA does not allow a function without any return value.
#[unsafe(no_mangle)]
pub extern "C" fn drop_data(ptr_data: *mut Pointer) -> bool {

    if ptr_data.is_null() {
        true
    
    } else {
        Data::drop(ptr_data);
        true
    }
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
pub extern "C" fn init_array(row: i32, col: i32) -> *mut Pointer {
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

    if ptr_arr.is_null() {
        unsafe { *ptr_result = false };
        return Data::from(CSTRING::from(format!("Null pointer detected"))).into_raw_pointer()
    }

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
pub extern "C" fn arr_set_i8(ptr_arr:*mut Pointer, row: i32, col: i32, val: i8, ptr_result: *mut bool) -> *mut Pointer {

    if ptr_arr.is_null() {
        unsafe { *ptr_result = false };
        return Data::from(CSTRING::from(format!("Null pointer detected"))).into_raw_pointer()
    }

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
pub extern "C" fn arr_set_i16(ptr_arr: *mut Pointer, row: i32, col: i32, val: i16, ptr_result: *mut bool) -> *mut Pointer {

    if ptr_arr.is_null() {
        unsafe { *ptr_result = false };
        return Data::from(CSTRING::from(format!("Null pointer detected"))).into_raw_pointer()
    }

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
pub extern "C" fn arr_set_i32(ptr_arr: *mut Pointer, row: i32, col: i32, val: i32, ptr_result: *mut bool) -> *mut Pointer {

    if ptr_arr.is_null() {
        unsafe { *ptr_result = false };
        return Data::from(CSTRING::from(format!("Null pointer detected"))).into_raw_pointer()
    }

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
pub extern "C" fn arr_set_i64(ptr_arr: *mut Pointer, row: i32, col: i32, val: i64, ptr_result: *mut bool) -> *mut Pointer {

    if ptr_arr.is_null() {
        unsafe { *ptr_result = false };
        return Data::from(CSTRING::from(format!("Null pointer detected"))).into_raw_pointer()
    }

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
pub extern "C" fn arr_set_f32(ptr_arr: *mut Pointer, row: i32, col: i32, val: f32, ptr_result: *mut bool) -> *mut Pointer {

    if ptr_arr.is_null() {
        unsafe { *ptr_result = false };
        return Data::from(CSTRING::from(format!("Null pointer detected"))).into_raw_pointer()
    }

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
pub extern "C" fn arr_set_f64(ptr_arr: *mut Pointer, row: i32, col: i32, val: f64, ptr_result: *mut bool) -> *mut Pointer {

    if ptr_arr.is_null() {
        unsafe { *ptr_result = false };
        return Data::from(CSTRING::from(format!("Null pointer detected"))).into_raw_pointer()
    }

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
pub extern "C" fn arr_set_bool(ptr_arr: *mut Pointer, row: i32, col: i32, val: bool, ptr_result: *mut bool) -> *mut Pointer {

    if ptr_arr.is_null() {
        unsafe { *ptr_result = false };
        return Data::from(CSTRING::from(format!("Null pointer detected"))).into_raw_pointer()
    }

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
pub extern "C" fn arr_set_none(ptr_arr: *mut Pointer, row: i32, col: i32, ptr_result: *mut bool) -> *mut Pointer {

    if ptr_arr.is_null() {
        unsafe { *ptr_result = false };
        return Data::from(CSTRING::from(format!("Null pointer detected"))).into_raw_pointer()
    }

    match Data::set_none(ptr_arr, row, col) {
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
pub extern "C" fn arr_set_str(ptr_arr: *mut Pointer, row: i32, col: i32, val: *const Pointer, ptr_result: *mut bool) -> *mut Pointer {

    if ptr_arr.is_null() {
        unsafe { *ptr_result = false };
        return Data::from(CSTRING::from(format!("Null pointer detected"))).into_raw_pointer()
    }

    match Data::set_str(ptr_arr, row, col, val) {
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
pub extern "C" fn arr_set_arr(ptr_p_arr: *mut Pointer, row: i32, col: i32, ptr_m_arr: *mut Pointer, ptr_result: *mut bool) -> *mut Pointer {

    if ptr_p_arr.is_null() {
        unsafe { *ptr_result = false };
        return Data::from(CSTRING::from(format!("Null pointer detected"))).into_raw_pointer()
    }

    match Data::set_array(ptr_p_arr, row, col, ptr_m_arr) {
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
pub extern "C" fn get_elem_ptr(ptr_arr: *const Pointer, row:i32, col: i32, ptr_result: *mut bool) -> *const Pointer {

    if ptr_arr.is_null() {
        unsafe { *ptr_result = false };
        return Data::from(CSTRING::from(format!("Null pointer detected"))).into_raw_pointer()
    }

    match Data::get_ptr_arr_element(ptr_arr, row, col) {
        Ok(ptr) => {
            unsafe { *ptr_result = true };
            return ptr
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

    let returned = match core::datatype::string::copy_from_cstr(ptr_cstr) {
        Ok(mut copied) => {
            copied.push_str(" <Rust>　このすごい言語!");
            copied
        }
        Err(e) => {
            e
        }
    };

    Data::from(CSTRING::from(returned)).into_raw_pointer()
}

#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
pub extern "C" fn array_test() -> *const Pointer {

    Data::from(vec![
        Data::from(vec![Data::from(CSTRING::from(String::from("Hello, this is Rust"))), Data::from(100_i8), Data::from(true)])
        ]
    ).into_raw_pointer()
}

#[test]
fn array_sample_test() {

    
    let ptr_arr = Data::from(vec![
        Data::from(vec![Data::from(CSTRING::from(String::from("Hello, this is Rust"))), Data::from(100_i8), Data::from(true)])
        ]
    ).into_raw_pointer();

    let mut result: bool = false;

    let ptr_num_row = arr_num_rows(ptr_arr, &mut result as *mut bool);

    if result {
        println!("num_row {}", get_i32(ptr_num_row as *const Pointer))
    }

    println!("{:?}", drop_data(ptr_num_row));

    let ptr_num_col = arr_num_cols(ptr_arr, &mut result as *mut bool);

    if result {
        println!("num_col {}", get_i32(ptr_num_col as *const Pointer))
    }

    println!("{:?}", drop_data(ptr_num_col));

    println!("{:?}", drop_data(ptr_arr));

}


#[unsafe(no_mangle)]
pub extern "C" fn list_dll(root: *mut Pointer, ptr_err: *mut bool) -> *mut Pointer {

    let root_path = match copy_from_cstr(root) {
        Ok(root_path) => PathBuf::from(root_path),
        Err(e) => {    
            unsafe { *ptr_err = false };
            return Data::from(CSTRING::from(format!("Failed to parse passed root: {:?}", e))).into_raw_pointer()
        }
    };

    if !root_path.exists() {
        unsafe { *ptr_err = false };
        return Data::from(CSTRING::from(format!("Not existing path: {}", root_path.display()))).into_raw_pointer()
    }

    let dir = match std::fs::read_dir(&root_path) {
        Ok(dir) => dir,
        Err(e) => {
            unsafe { *ptr_err = false };
            return Data::from(CSTRING::from(format!("Failed to read root dir: {:?}", e))).into_raw_pointer()
        }
    };

    let mut created_dll_name = String::from(env!("CARGO_CRATE_NAME"));
    created_dll_name.push_str(".dll");

    let dirs = dir.into_iter().filter_map(|sub_dir| {
        sub_dir.ok()
        .and_then(|sub_dir| {
            sub_dir.file_name().to_str()    
            .and_then(|sub_dir_str| {
                if sub_dir_str.ends_with(".dll") & !sub_dir_str.ends_with(&created_dll_name) {
                    Some(Data::from(CSTRING::from(sub_dir_str.to_string())))
                } else {
                    None
                }
            })
        })
    }).collect::<Vec<Data>>();

    unsafe { *ptr_err = true };

    Data::from(
        vec![
           dirs 
        ]
    ).into_raw_pointer()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_dll_note(root: *mut Pointer, dll_name: *mut Pointer, ptr_err: *mut bool) -> *mut Pointer {
    let root_path = match copy_from_cstr(root) {
        Ok(root_path) => PathBuf::from(root_path),
        Err(e) => {
            unsafe { *ptr_err = false };
            return Data::from(CSTRING::from(format!("Faield to parse passed root: {:?}", e))).into_raw_pointer()
        }
    };

    let dll_name = match copy_from_cstr(dll_name) {
        Ok(dll_name) => dll_name,
        Err(e) => {
            unsafe { *ptr_err = false };
            return Data::from(CSTRING::from(format!("Failed to parse dll name: {:?}", e))).into_raw_pointer()
        }
    };

    let mut full_dll_path = root_path;
    full_dll_path.push(&dll_name);

    if !full_dll_path.exists() {
        unsafe { *ptr_err = false };
        return Data::from(CSTRING::from(format!("Specified dll_path does not exist: {}", full_dll_path.display()))).into_raw_pointer()
    }

    let full_path_str = full_dll_path.into_iter().filter_map(|elem| elem.to_str().map(|s| s.to_string()))
    .collect::<Vec<String>>()
    .join("\\");

    let dll = match DLL::load(&full_path_str) {
        Ok(dll) => dll,
        Err(e) => {
            unsafe { *ptr_err = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    };
    
    match dll.get_ptr_note() {
        Ok(ptr_note) => {
            unsafe { *ptr_err = true };
            return unsafe { ptr_note() }
        }
        Err(e) => {
            unsafe { *ptr_err = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_dll_args_info(root: *mut Pointer, dll_name: *mut Pointer, ptr_err: *mut bool) -> *mut Pointer {
    let root_path = match copy_from_cstr(root) {
        Ok(root_path) => PathBuf::from(root_path),
        Err(e) => {
            unsafe { *ptr_err = false };
            return Data::from(CSTRING::from(format!("Failed to parse passed root: {:?}", e))).into_raw_pointer()
        }
    };

    let dll_name = match copy_from_cstr(dll_name) {
        Ok(dll_name) => dll_name,
        Err(e) => {
            unsafe { *ptr_err = false };
            return Data::from(CSTRING::from(format!("Failed to parse dll name: {:?}", e))).into_raw_pointer()
        }
    };

    let mut full_dll_path = root_path;
    full_dll_path.push(dll_name);

    if !full_dll_path.exists() {
        unsafe { *ptr_err = false };
        return Data::from(CSTRING::from(format!("Specified dll_path does not exist: {}", full_dll_path.display()))).into_raw_pointer()
    }

    let full_path_str = full_dll_path.into_iter().filter_map(|elem| elem.to_str().map(|s| s.to_string()))
    .collect::<Vec<String>>()
    .join("\\");

    let dll = match DLL::load(&full_path_str) {
        Ok(dll) => dll,
        Err(e) => {
            unsafe { *ptr_err = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    };

    match dll.get_ptr_args_info() {
        Ok(ptr_args_info) => {
            unsafe { *ptr_err = true };
            return unsafe { ptr_args_info() }
        }
        Err(e) => {
            unsafe {*ptr_err = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn call_dll(root: *mut Pointer, dll_name: *mut Pointer, ptr_args: *mut Pointer, ptr_err: *mut bool) -> *mut Pointer {
    let root_path = match copy_from_cstr(root) {
        Ok(root_path) => PathBuf::from(root_path),
        Err(e) => {
            unsafe { *ptr_err = false };
            return Data::from(CSTRING::from(format!("Faield to parse passed root: {:?}", e))).into_raw_pointer()
        }
    };

    let dll_name = match copy_from_cstr(dll_name) {
        Ok(dll_name) => dll_name,
        Err(e) => {
            unsafe { *ptr_err = false };
            return Data::from(CSTRING::from(format!("Failed to parse dll name: {:?}", e))).into_raw_pointer()
        }
    };

    let mut full_dll_path = root_path;
    full_dll_path.push(dll_name);

    if !full_dll_path.exists() {
        unsafe { *ptr_err = false };
        return Data::from(CSTRING::from(format!("Specified dll_path does not exist: {}", full_dll_path.display()))).into_raw_pointer()
    }

    let full_path_str = full_dll_path.into_iter().filter_map(|elem| elem.to_str().map(|s| s.to_string()))
    .collect::<Vec<String>>()
    .join("\\");

    let dll = match DLL::load(&full_path_str) {
        Ok(dll) => dll,
        Err(e) => {
            unsafe { *ptr_err = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    };

    match dll.get_prt_call_func() {
        Ok(ptr_func) => {
            unsafe { *ptr_err = true };
            return unsafe { ptr_func(ptr_args, ptr_err) }
        }
        Err(e) => {
            unsafe { *ptr_err = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    }
}