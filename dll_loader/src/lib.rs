use windows::Win32::System::LibraryLoader::{LoadLibraryW, GetProcAddress};
use windows::Win32::Foundation::{FreeLibrary, HMODULE};
use windows::core::{PCWSTR, PCSTR};

use core::Pointer;
use dll_interface::{INTERFACE_NAME_ARGS_INFO, INTERFACE_NAME_NOTE, INTERFACE_NAME_CALL_FUNC};

use std::path::PathBuf;

pub struct DLL(HMODULE);

impl Drop for DLL {
    fn drop(&mut self) {
        let hmodule = self.0;
        unsafe { 
            match FreeLibrary(hmodule) {
                Ok(_) => {
                    #[cfg(debug_assertions)] {
                        println!("Library freed")
                    }
                }
                Err(e) => {
                    #[cfg(debug_assertions)] {
                        println!("Error occured while freeing a library: {:?}", e)
                    }
                }
            } 
        };
    }
}

impl DLL {
    pub fn load(path: &str) -> Result<Self, String> {

        let path_buf = PathBuf::from(path);
        if !path_buf.exists() {
            return Err(format!("path: {} does not exist", path))
        }

        let mut utf16_path = path.encode_utf16().into_iter().collect::<Vec<u16>>();
        utf16_path.push(0);

        let ptr_path = PCWSTR(utf16_path.as_ptr());

        unsafe {LoadLibraryW(ptr_path) }
        .map_err(|e| format!("Failed to load a dll at {}: {:?}", path, e))
        .map(|hmodule| DLL(hmodule))
    }

    fn get_ptr_func(&self, func_name: &str) -> Result<*const Pointer, String> {

        let mut utf8_func_name = func_name.bytes().collect::<Vec<u8>>();
        utf8_func_name.push(0);

        let ptr_func_name = PCSTR(utf8_func_name.as_ptr());

        unsafe { GetProcAddress(self.0, ptr_func_name) }
        .map(|addr| {
            addr as *const Pointer
        })
        .map_or_else(|| Err(format!("Failed to find the specified function")), |addr| Ok(addr))
    }

    pub fn get_ptr_note(&self) -> Result<unsafe extern "C" fn () -> *mut Pointer, String> {

        self.get_ptr_func(INTERFACE_NAME_NOTE)
        .map(|ptr| {
           let ptr: unsafe extern "C" fn() -> *mut Pointer = unsafe { std::mem::transmute(ptr) };
           ptr
        })
    }

    pub fn get_ptr_args_info(&self) -> Result<unsafe extern "C" fn () -> *mut Pointer, String> {
        self.get_ptr_func(INTERFACE_NAME_ARGS_INFO)
        .map(|ptr| {
            let ptr: unsafe extern "C" fn () -> *mut Pointer = unsafe { std::mem::transmute(ptr) };
            ptr
        })
    }

    pub fn get_prt_call_func(&self) -> Result<unsafe extern "C" fn (*mut Pointer, *mut bool) -> *mut Pointer, String>  {
        self.get_ptr_func(INTERFACE_NAME_CALL_FUNC)
        .map(|ptr| {
            let ptr: unsafe extern "C" fn (*mut Pointer, *mut bool) -> *mut Pointer = unsafe { std::mem::transmute(ptr) };
            ptr
        })
    }
}