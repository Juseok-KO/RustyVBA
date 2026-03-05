use core::Pointer;
use core::datatype::{Data, string::{CSTRING, copy_from_cstr}, Value};
use std::ptr::null;
use dll_loader::DLL;
use dll_finder;
use dynamic_library::{DynamicLibrary, LibCollection};

use std::path::PathBuf;

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

#[unsafe(no_mangle)]
pub extern "C" fn list_dll(default_root: *mut Pointer, ptr_result: *mut bool) -> *mut Pointer {

    let root_path = match copy_from_cstr(default_root) {
        Ok(root_path) => PathBuf::from(root_path),
        Err(e) => {    
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to parse passed root: {:?}", e))).into_raw_pointer()
        }
    };

    match dll_finder::list_all_dll(&root_path) {
        Ok(list) => {
            unsafe { *ptr_result = true };

            Data::from(list.into_iter().map(|line| {
                Data::from(line.into_iter().map(|item| Data::from(CSTRING::from(item)))
                .collect::<Vec<Data>>())
            }).collect::<Vec<Data>>())
            .into_raw_pointer()

        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("No dll found: {:?}", e))).into_raw_pointer()
        }
    }


}

#[unsafe(no_mangle)]
pub extern "C" fn list_dll_dirs(default_root: *mut Pointer, ptr_result: *mut bool) -> *mut Pointer {

    let root_path = match copy_from_cstr(default_root) {
        Ok(default_root) => PathBuf::from(default_root),
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to parse passed default_root: {:?}", e))).into_raw_pointer()
        }
    };

    match dll_finder::list_dll_dirs(&root_path) {
        Ok(dirs) => {
            unsafe { *ptr_result = true };
            Data::from(dirs.into_iter().map(|dir| Data::from(vec![Data::from(CSTRING::from(dir))]))
            .collect::<Vec<Data>>()).into_raw_pointer()
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("No dir found: {:?}", e))).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn list_dll_under_dir(default_root: *mut Pointer, dir_name: *mut Pointer, ptr_result: *mut bool) -> *mut Pointer {
    let default_root = match copy_from_cstr(default_root) {
        Ok(default_root) => PathBuf::from(default_root),
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to parse passed default_root: {:?}", e))).into_raw_pointer()
        }
    };

    let dir_name = match copy_from_cstr(dir_name) {
        Ok(dir_name) => dir_name,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to parse passed dir_name: {:?}", e))).into_raw_pointer()
        }
    };

    match dll_finder::list_dll_under_dir(&default_root, &dir_name) {
        Ok(dlls) => {
            unsafe { *ptr_result = true };
            Data::from(dlls.into_iter().map(|dll| Data::from(vec![Data::from(CSTRING::from(dll))]) )
            .collect::<Vec<Data>>()).into_raw_pointer()
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("No dll found: {:?}", e))).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn init_dylib_collection() -> *mut Pointer {
    let ptr = LibCollection::new();
    dynamic_library::scan(ptr);
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn drop_dylib_collection(ptr_collection: *mut Pointer) -> bool {
    LibCollection::drop(ptr_collection);
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn get_or_load_lib(ptr_collection: *mut Pointer, default_root: *mut Pointer, dir_name: *mut Pointer, dll_name: *mut Pointer, ptr_result: *mut bool) -> *mut Pointer {

    let lt = ();
    let lib_collection = match LibCollection::from_raw_ptr(ptr_collection, &lt) {
        Ok(lib_collection) => lib_collection,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    };

    let default_root = match copy_from_cstr(default_root) {
        Ok(default_root) => default_root,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to parse default_root: {}", e))).into_raw_pointer()
        }
    };

    let dir_name = match copy_from_cstr(dir_name) {
        Ok(dir_name) => dir_name,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to parse dir_name: {}", e))).into_raw_pointer()
        }
    };

    let dll_name = match copy_from_cstr(dll_name) {
        Ok(dll_name) => dll_name,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Faield to parse dll_name: {}", e))).into_raw_pointer()
        }
    };

    match lib_collection.read().map(|read_lock| {
        read_lock.get_lib(dir_name.clone(), dll_name.clone())
        .map(|dll| dll.into_ptr())
    })
    {
        Ok(Some(ptr_dll)) => {
            unsafe { *ptr_result = true };
            return ptr_dll
        }
        Ok(None) => {

        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to read-lock the dylib collection: {:?}", e))).into_raw_pointer()
        }
    }

    match lib_collection.write().map(|mut write_lock| {
        write_lock.load_lib(default_root, dir_name.clone(), dll_name.clone())
    }) 
    {
        Ok(Ok(_)) => {

        }
        Ok(Err(e)) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to load the lib: {}", e))).into_raw_pointer()
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to write-lock the dylib collection: {:?}", e))).into_raw_pointer()
        }
    }

    match lib_collection.read().map(|read_lock| {
        read_lock.get_lib(dir_name, dll_name)
        .map(|dll| dll.into_ptr())
    }) {
        Ok(Some(ptr_dll)) => {
            unsafe { *ptr_result = true };
            return ptr_dll
        }
        Ok(None) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to get the dylib after loading: None"))).into_raw_pointer()
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return  Data::from(CSTRING::from(format!("Failed to read-lock the dylib collection after loading: {:?}", e))).into_raw_pointer()
        }
    }
    
}

#[unsafe(no_mangle)]
pub extern "C" fn get_dll_note(ptr_dll: *mut Pointer, ptr_result: *mut bool) -> *mut Pointer {

    let lt = ();
    let dll = match DynamicLibrary::from_raw_ptr(ptr_dll, &lt) {
        Ok(dll) => dll,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to convert raw_ptr to dll: {:?}", e))).into_raw_pointer()
        }
    };

    match dll.as_ref().get_ptr_note().map(|ptr_func_note| unsafe { ptr_func_note() }) {
        Ok(note) => {
            unsafe { *ptr_result = true };
            return  note
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to get note: {:?}", e))).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_dll_args_info(ptr_dll: *mut Pointer, ptr_result: *mut bool) -> *mut Pointer {

    let lt = ();
    let dll = match DynamicLibrary::from_raw_ptr(ptr_dll, &lt) {
        Ok(dll) => dll,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Faield to convert raw_ptr to dll: {:?}", e))).into_raw_pointer()
        }
    };

    match dll.as_ref().get_ptr_args_info().map(|ptr_func_arg_info| unsafe { ptr_func_arg_info() }) {
        Ok(arg_info) => {
            unsafe { *ptr_result = true };
            return arg_info
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to get arg_info: {:?}", e))).into_raw_pointer()
        }
    }
}
/// When the dll function call failed, this caller first frees the return value from the dll, and return another error message allocated by itself.
#[unsafe(no_mangle)]
pub extern "C" fn call_dll_func(ptr_dll: *mut Pointer, ptr_args: *mut Pointer, ptr_result: *mut bool) -> *mut Pointer {

    let lt = ();
    let dll = match DynamicLibrary::from_raw_ptr(ptr_dll, &lt) {
        Ok(dll) => dll,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to convert raw_ptr into dll: {:?}", e))).into_raw_pointer()
        }
    };

    match dll.as_ref().get_ptr_call_func().map(|ptr_func| unsafe { ptr_func(ptr_args, ptr_result) }) {
        Ok(output) => {
            if unsafe { *ptr_result } {
                return output
            } else {

                let d = unsafe { &*(ptr_result as *mut Data as *const Data) };
                let err_msg = if let Value::CSTRING(err_msg) = d.get_value() {
                    match err_msg.get_string() {
                        Ok(err_msg) => {
                            err_msg
                        }
                        Err(e) => {
                            unsafe { *ptr_result = false };
                            format!("Failed to parse err_msg from the function: {:?}", e)
                        }
                    }
                } else {
                    format!("The function returned an Error in a form other than String")
                };

                dll.as_ref().get_ptr_simple_dealloc().iter().for_each(|ptr_simple_dealloc| unsafe {
                    ptr_simple_dealloc(output);
                });

                unsafe { *ptr_result = false };
                Data::from(CSTRING::from(err_msg)).into_raw_pointer()
            }
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to call function: {:?}", e))).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_dll_result(ptr_dll: *mut Pointer, ptr_dll_result: *mut Pointer, ptr_result: *mut bool) -> *mut Pointer {

    let lt = ();
    let dll = match DynamicLibrary::from_raw_ptr(ptr_dll, &lt) {
        Ok(dll) => dll,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to convert raw_ptr into dll: {:?}", e))).into_raw_pointer()
        }
    };

    match dll.as_ref().get_ptr_dealloc() {
        Ok(func_dealloc) => {
            unsafe { func_dealloc(ptr_dll_result)};

            unsafe { *ptr_result = true };
            return null::<Pointer>() as *mut Pointer

        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to get ptr_dealloc: {:?}", e))).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_simple_dll_result(ptr_dll: *mut Pointer, ptr_dll_result: *mut Pointer, ptr_result: *mut bool) -> *mut Pointer {

    let lt = ();
    let dll = match DynamicLibrary::from_raw_ptr(ptr_dll, &lt) {
        Ok(dll) => dll,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to convert raw_ptr into dll: {:?}", e))).into_raw_pointer()
        }
    };

    match dll.as_ref().get_ptr_simple_dealloc() {
        Ok(func_simple_dealloc) => {
            unsafe { func_simple_dealloc(ptr_dll_result )};

            unsafe { *ptr_result = true };
            return null::<Pointer>() as *mut Pointer
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(format!("Failed to get ptr_simple_dealloc: {:?}", e))).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn drop_dll(ptr_dll: *mut Pointer) -> bool {
    DynamicLibrary::free(ptr_dll);
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn init_resources() -> *mut Pointer {
    global_resource::Resources::new()
}

#[unsafe(no_mangle)]
pub extern "C" fn drop_resources(ptr_resources: *mut Pointer) -> bool {
    global_resource::Resources::drop(ptr_resources);
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn set_resource(ptr_resources: *mut Pointer, ptr_lib_collection: *mut Pointer, item_key: *mut Pointer, default_root: *mut Pointer, dir_name: *mut Pointer, dll_name: *mut Pointer, ptr_args: *mut Pointer, ptr_result: *mut bool) -> *mut Pointer {

    let item_key = match copy_from_cstr(item_key) {
        Ok(item_key) => item_key,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    };

    let default_root = match copy_from_cstr(default_root) {
        Ok(default_root) => default_root,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    };

    let dir_name = match copy_from_cstr(dir_name) {
        Ok(dir_name) => dir_name,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    };

    let dll_name = match copy_from_cstr(dll_name) {
        Ok(dll_name) => dll_name,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    };

    let lt = ();
    let resources = match global_resource::Resources::from_raw_ptr(ptr_resources, &lt) {
        Ok(resource) => resource,
        Err(e) => {
            unsafe {*ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    };

    match resources.write().map_err(|e| format!("Resources Write Lock Error: {:?}", e)).and_then(| mut r|r.set_item(ptr_lib_collection, item_key, &default_root, &dir_name, &dll_name, ptr_args)) {
        Ok(_) => { 
            unsafe { *ptr_result = true };
            return null::<Pointer>() as *mut Pointer
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_resource(ptr_resources: *mut Pointer, item_key: *mut Pointer, ptr_result: *mut bool) -> *mut Pointer {
    let item_key = match copy_from_cstr(item_key) {
        Ok(item_key) => item_key,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    };

    let lt = ();
    let resources = match global_resource::Resources::from_raw_ptr(ptr_resources, &lt) {
        Ok(resources) => resources,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    };

    match resources.read().map_err(|e| format!("Resources Read Lock Failed: {:?}", e)).and_then(|r| r.get_item(&item_key)) {
        Ok(res) => {
            unsafe { *ptr_result = true };
            return res
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn del_resource(ptr_resources: *mut Pointer, item_key: *mut Pointer, ptr_result: *mut bool) -> *mut Pointer {
    let item_key = match copy_from_cstr(item_key) {
        Ok(item_key) => item_key,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    };

    let lt = ();
    let resources = match global_resource::Resources::from_raw_ptr(ptr_resources, &lt) {
        Ok(resources) => resources,
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    };

    match resources.write().map_err(|e| format!("Resource Write Lock Failed: {:?}", e)).and_then(|mut r| r.del_item(&item_key)) {
        Ok(_) => {
            unsafe { *ptr_result = true };
            return null::<Pointer>() as *mut Pointer
        }
        Err(e) => {
            unsafe { *ptr_result = false };
            return Data::from(CSTRING::from(e)).into_raw_pointer()
        }
    }
}