use dll_loader::DLL;
use dll_finder::dir_name_to_dir;
use core::Pointer;


use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::{Arc, RwLock};
use std::thread::{spawn, sleep};
use std::time::Duration;

#[derive(Debug)]
pub struct DynamicLibrary {
    dll: Arc<DLL>,
    counter: Arc<AtomicIsize>
}

impl DynamicLibrary {
    pub fn into_ptr(self) -> *mut Pointer {
        Box::leak(Box::new(self)) as *mut DynamicLibrary as *mut Pointer
    }

    pub fn from_raw_ptr(ptr_lib: *mut Pointer, lt: &()) -> Result<&Self, String> {
        if ptr_lib.is_null() {
            return Err(format!("Null ptr passed as ptr lib"))
        }

        unsafe { Ok(&*(ptr_lib as *mut DynamicLibrary as *const DynamicLibrary)) }
    }

    pub fn free(ptr_lib: *mut Pointer) {
        if ptr_lib.is_null() {
            return
        }

        let dylib = unsafe { Box::from_raw(ptr_lib as *mut DynamicLibrary) };
        drop(dylib)
    }
}

impl AsRef<DLL> for DynamicLibrary {
    fn as_ref(&self) -> &DLL {
        self.dll.as_ref()
    }
}

impl Drop for DynamicLibrary {
    fn drop(&mut self) {
        self.counter.fetch_sub(1, Ordering::SeqCst);
    }
}

#[derive(Debug)]
pub struct LibCollection {
    internal: std::collections::HashMap<(String, String), DynamicLibrary>
}

/// Sucess valuees are returned from the loaded dll. The error values are returned from dispatcher.
impl LibCollection {

    pub fn new() -> *mut pointer {

        Box::leak(Box::new(RwLock::new(LibCollection{ internal: std::collections::HashMap::new() }))) as *mut RwLock<LibCollection> 
        as *mut Pointer
    }

    pub fn drop(ptr_col: *mut Pointer) {
        if ptr_col.is_null() { return }

        let collection = unsafe { Box::from_raw(ptr_col as *mut RwLock<LibCollection>) };
        drop(collection);
    }

    pub fn from_raw_ptr(ptr_col: *mut Pointer, lt: &()) -> Result<&RwLock<LibCollection>, String> {

        if ptr_col.is_null() {
            return Err(format!("Null Pointer passed as ptr_col"))
        }

        unsafe { Ok(&*(ptr_col as *mut RwLock<LibCollection> as *const RwLock<LibCollection>)) }
    }

    pub fn get_lib(&self, folder_name: String, lib_name: String) -> Option<DynamicLibrary> {
        self.internal.get(&(folder_name, lib_name)).map(|dll| {
            dll.counter.fetch_add(1, Ordering::SeqCst);
            DynamicLibrary { dll: dll.dll.clone(), counter: dll.counter.clone()}
        })
    }

    pub fn load_lib(&mut self, default_folder_path: String, folder_name: String, lib_name: String) -> Result<(), String> {

        let default_folder_path = std::path::Path::from(default_folder_path.as_str());
        let mut dll_full_path = std::path::PathBuf::from(dir_name_to_dir(&default_folder_path, &folder_name)?);
        dll_full_path.push(lib_name);

        if !dll_full_path.exists() {
            return Err(format!("Does not exist: {}", dll_full_path.display()))
        }

        let full_path_str = dll_full_path.into_iter().filter_map(|elem| elem.to_str().map(|s| s.to_string()))
        .collect::<Vec<String>>()
        .join("\\");

        let dylib = DynamicLibrary {
            dll: Arc::new(DLL::load(&full_path_str)?),
            counter: Arc::new(AtomicIsize::new(0))
        };

        self.internal.insert((folder_name, lib_name), dylib);

        Ok(())
    }

    pub fn unload_lib(&mut self, folder_name: String, lib_name: String) {

        self.internal.remove(&(folder_name, lib_name));
    }

    pub fn get_counters(&self) -> Vec<((String, String), Arc<AtomicIsize>)> {

        self.internal.iter().map(|((folder_name, lib_name), dylib)| {
            ((folder_name.to_string(), lib_name.to_string()), dylib.counter.clone())
        })
        .collect::<Vec<_>>()
    }

    pub fn free_idle_libs(&mut self) {

        let targets = self.get_counters().into_iter().filter(|((dir_name, dll_name), counter)| {
            counter.load(Ordering::SeqCst) < 1
        }).collect::<Vec<(String, String)>>();

        for t in targets {
            self.internal.remove(&t);
        }
    }
}

pub fn scan(ptr_lib_collection: *mut Pointer) {

    spawn(move || {
        loop {
            let lt = ();
            if let Ok(lib_collection) = LibCollection::from_raw_ptr(ptr_lib_collection, &lt) {
                if let Ok(mut write_lock) = lib_collection.write() {
                    write_lock.free_idle_libs();
                }
            }
            sleep(Duration::from_mins(5));
        }
    });
    
}