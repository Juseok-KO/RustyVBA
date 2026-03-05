use core::Pointer;
use core::datatype::{Data, Value};
use std::ptr::null;
use dll_loader::DLL;
use std::sync::RwLock;
use dynamic_library::{DynamicLibrary, LibCollection};

#[repr(C)]
pub struct Resource {
    dll: DynamicLibrary,
    data: *mut Pointer,
}

impl Drop for Resource {
    fn drop(&mut self) {

        let func_dealloc = match self.dll.as_ref().get_ptr_dealloc()  {
            Ok(func_dealloc) => func_dealloc,
            Err(e) => {
                #[cfg(debug_assertions)] {
                    println!("Resource::drop() failed: {:?}", e);
                    return
                }
            }
        };

        unsafe { func_dealloc(self.data) };
        self.data = null::<Pointer>() as *mut Pointer;
    }
}

pub struct Resources {
    inner: std::collections::HashMap<String, Resource>
}

impl Resources {
    pub fn new() -> *mut Pointer {
        let r = Box::leak(Box::new(RwLock::new(Resources { inner: std::collections::HashMap::new() }))) as *mut RwLock<Resources> as i64;
        Data::from(r).into_raw_pointer()
    }

    pub fn drop(ptr_r: *mut Pointer) {
        if ptr_r.is_null() {
            return
        }

        let addr = unsafe { Box::from_raw(ptr_r as *mut Data) };
        if let Value::I64(addr) = addr.get_value() {

            let r = unsafe { Box::from_raw(*addr as usize as *const RwLock<Resources> as *mut RwLock<Resources>) };

        } else {
            return
        }
    }

    pub fn from_raw_ptr(ptr_r: *mut Pointer, _lt: &()) -> Result<&RwLock<Resources>, String> {
        if ptr_r.is_null() {
            return Err(format!("Null pointer as resources"))
        }

        let d = unsafe { &*(ptr_r as *mut Data) };
        let Value::I64(addr) = d.get_value() else {
            return Err(format!("ptr_r does not contain a valid address"))
        };

        let r = unsafe { &*(*addr as *const RwLock<Resources>) };

        Ok(r)
        
    }

    pub fn get_item(&self, key: &str) -> Result<*mut Pointer, String> {

        self.inner.get(key)
        .ok_or(format!("No value with the key {}", key))
        .map(|r| r.data)
    }

    pub fn set_item(&mut self, dylib_collections: *mut Pointer, item_key: String, default_root: &str, dir_name: &str, dll_name: &str, ptr_args: *mut Pointer) -> Result<(), String> {

        let lt = ();
        let lib_collection = LibCollection::from_raw_ptr(dylib_collections, &lt)?;

        let mut result = true;

        match lib_collection.read().map_err(|e| format!("Failed to read-lock the dylib collection"))?
        .get_lib(dir_name.to_string(), dll_name.to_string()) {
            Some(dll) => {
                let item_val = dll.as_ref().get_ptr_call_func().map(|ptr_func| unsafe { ptr_func(ptr_args, &mut result as *mut bool)})?;
                    
                if result {

                    self.inner.insert(item_key, Resource { dll: dll, data: item_val});
                    return Ok(())

                } else {
                    let d = unsafe { &*(item_val as *mut Data)};
                    let mut err_msg = if let Value::CSTRING(err_msg) = d.get_value() {
                        match err_msg.get_string() {
                            Ok(err_msg) => err_msg,
                            Err(e) => {
                                e
                            }
                        }
                    } else {
                        format!("Failed to parse error from the function")
                    };

                    match dll.as_ref().get_ptr_simple_dealloc().map(|ptr_simple_dealloc|  unsafe {
                        ptr_simple_dealloc(item_val);
                     }) {
                        Ok(_) => {},
                        Err(e) => {
                            err_msg.push(' ');
                            err_msg.push_str(&e);
                        }
                    }

                    return Err(err_msg)
                }
            }
            None => {

            }
        }

        match lib_collection.write().map_err(|e| format!("Failed to write-lock the dylib collection"))?
        .load_lib(default_root.to_string(), dir_name.to_string(), dll_name.to_string()) {
            Ok(_) => {

            }
            Err(e) => {
                return Err(format!("Failed to set item: {}", e))
            }
        }

        match lib_collection.read().map_err(|e| format!("Failed to read-lock the dylib collection after loading lib"))?
        .get_lib(dir_name.to_string(), dll_name.to_string()) {
            Some(dll) => {
                let item_val = dll.as_ref().get_ptr_call_func().map(|ptr_func| unsafe {ptr_func(ptr_args, &mut result as *mut bool)})?;

                if result {
                    self.inner.insert(item_key, Resource { dll, data: item_val});
                    return Ok(())
                
                } else {
                    let d = unsafe { &*(item_val as *mut Data)};
                    let mut err_msg = if let Value::CSTRING(err_msg) = d.get_value() {
                        match err_msg.get_string() {
                            Ok(err_msg) => err_msg,
                            Err(e) => {
                                e
                            }
                        }
                    } else {
                        format!("Failed to parse error from the function")
                    };

                    match dll.as_ref().get_ptr_simple_dealloc().map(|ptr_simple_dealloc| unsafe {
                        ptr_simple_dealloc(item_val);
                    }) {
                        Ok(_) => {}
                        Err(e) => {
                            err_msg.push(' ');
                            err_msg.push_str(&e);
                        }
                    }

                    err_msg.push_str(". Failure after loading the dll.");
                    return Err(err_msg)
                }
            }
            None => {
                return Err(format!("Failed to get the dll after loading"))
            }
        }        
    }

    pub fn del_item(&mut self, key: &str) -> Result<(), String> {


        if let Some(deleted) = self.inner.remove(key) {
            drop(deleted);
        }

        Ok(())
    } 
}