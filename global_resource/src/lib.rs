use core::Pointer;
use core::datatype::{Data, Value};
use std::ptr::null;
use dll_loader::DLL;
use std::sync::RwLock;

#[repr(C)]
pub struct Resource {
    dll: *mut Pointer,
    data: *mut Pointer,
}

impl Drop for Resource {
    fn drop(&mut self) {


        let dealloc = match unsafe { DLL::get_ptr_dealloc(self.dll) }  {
            Ok(dealloc) => dealloc,
            Err(e) => {
                return 
            }
        };

        unsafe { dealloc(self.data) };

        self.data = null::<Pointer>() as *mut Pointer;
        let dll = unsafe { Box::from_raw(self.dll as *mut DLL) };
        self.dll = null::<Pointer>() as *mut Pointer;
        drop(dll);

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

    pub fn set_item(&mut self, item_key: String, default_root: &str, dir_name: &str, dll_name: &str, ptr_args: *mut Pointer) -> Result<(), String> {

        let default_root = std::path::Path::new(default_root);
        let mut dll_path = std::path::PathBuf::from(dll_finder::dir_name_to_dir(default_root, dir_name)?);
        dll_path.push(dll_name);

        if !dll_path.exists() {
            return Err(format!("Specified dll_path does not exists: {}", dll_path.display()))
        }

        let dll_path = dll_path.into_iter().filter_map(|elem| elem.to_str().map(|s| s.to_string()))
        .collect::<Vec<String>>()
        .join("\\");

        let dll = dll_loader::DLL::load_and_wrap(&dll_path)?;
        let func = DLL::get_ptr_call_func(dll)?;

        let mut result = true;

        let return_val = unsafe { func(ptr_args, &mut result as *mut bool) };

        if result {

            self.inner.insert(item_key, Resource { dll, data: return_val} );
            Ok(())

        } else {

            let data = unsafe { &*(return_val as *mut Data )};
            let Value::CSTRING(err_msg) = data.get_value() else {

                DLL::drop(dll);
                return Err(format!("Failed to set item: funciton call failed, messege reading failed"))
            };

            let Ok(dealloc) = DLL::get_ptr_dealloc(dll) else {

                DLL::drop(dll);
                return Err(format!("Failed to set item: function call failed, failed to free memory allocated by the dll"))
            };

            let Ok(err_msg) = err_msg.get_string() else {
                
                DLL::drop(dll);
                return Err(format!("Failed to set item: function call failed, message recovery failed"))
            };

            unsafe { dealloc(return_val); }
            DLL::drop(dll);

            return Err(format!("Faield to set item: {}", err_msg))

        }
        
    }

    pub fn del_item(&mut self, key: &str) -> Result<(), String> {


        if let Some(deleted) = self.inner.remove(key) {
            drop(deleted);
        }

        Ok(())
    } 
}