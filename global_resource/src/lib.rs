use core::Pointer;
use core::datatype::{Data, Value};
use std::ptr::null;
use dll_loader::DLL;
use std::sync::atomic::{AtomicI64, AtomicBool, Ordering};
use std::thread::yield_now;

const STATE: AtomicBool = AtomicBool::new(true);
const COUNTER: AtomicI64 = AtomicI64::new(0);

pub fn writer_lock() {

    STATE.store(false, Ordering::Release);

    while let Err(_e) = COUNTER.compare_exchange(0, -1, Ordering::Acquire, Ordering::Relaxed) {
        yield_now();
    }
}

pub fn writer_release() {

    STATE.store(true, Ordering::Release);

    while let Err(_e) = COUNTER.compare_exchange(-1, 0, Ordering::Acquire, Ordering::Relaxed) {
        yield_now();
    }
}

pub fn reader_lock() {

    while !STATE.load(Ordering::Acquire) {
        yield_now();
    }

    COUNTER.fetch_add(1, Ordering::Acquire);
}

pub fn reader_release() {

    while !STATE.load(Ordering::Acquire) {
        yield_now();
    }

    COUNTER.fetch_add(-1, Ordering::Acquire);
}

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
        let r = Box::leak(Box::new(Resources { inner: std::collections::HashMap::new() })) as *mut Resources as i64;
        Data::from(r).into_raw_pointer()
    }

    pub fn drop(ptr_r: *mut Pointer) {
        if ptr_r.is_null() {
            return
        }

        let addr = unsafe { Box::from_raw(ptr_r as *mut Data) };
        if let Value::I64(addr) = addr.get_value() {

            let r = unsafe { Box::from_raw(*addr as usize as *const Resources as *mut Resources) };

        } else {
            return
        }
    }

    pub fn as_mut(ptr_r: *mut Pointer, _lt: &()) -> Result<&mut Resources, String> {
        if ptr_r.is_null() {
            return Err(format!("Null pointer as resources"))
        }

        let d = unsafe { &*(ptr_r as *mut Data) };
        let Value::I64(addr) = d.get_value() else {
            return Err(format!("ptr_r does not contain a valid address"))
        };

        let r = unsafe { &mut *(*addr as *const Resources as *mut Resources) };

        Ok(r)
        
    }

    pub fn get_item(ptr_r: *mut Pointer, key: &str) -> Result<*mut Pointer, String> {

        let lt = ();
        Self::as_mut(ptr_r, &lt)?
        .inner.get(key)
        .ok_or(format!("No value with the key {}", key))
        .map(|r| r.data)
    }

    pub fn set_item(ptr_r: *mut Pointer, item_key: String, default_root: &str, dir_name: &str, dll_name: &str, ptr_args: *mut Pointer) -> Result<(), String> {

        let lt = ();
        let r = Resources::as_mut(ptr_r, &lt)?;
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

            r.inner.insert(item_key, Resource { dll, data: return_val} );
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

    pub fn del_item(ptr_r: *mut Pointer, key: &str) -> Result<(), String> {

        let lt = ();
        let r = Resources::as_mut(ptr_r, &lt)?;

        if let Some(deleted) = r.inner.remove(key) {
            drop(deleted);
        }

        Ok(())
    } 
}